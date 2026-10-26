// Browser and Node timing harness. One page loads every arm.
// Package scripts are not run. Upstream wasm is unpacked from the shipped
// module text by unpack-upstream.mjs before this file loads those bytes.

const MODES = { ATTRIBUTES: 0, TRIANGLES: 1, INDICES: 2 };
const FILTERS = { NONE: 0, OCTAHEDRAL: 1, QUATERNION: 2, EXPONENTIAL: 3 };

function firstDiff(a, b) {
    if (a.length !== b.length) return 0;
    const n = a.length;
    for (let i = 0; i < n; i++) {
        if (a[i] !== b[i]) return i;
    }
    return -1;
}

function resolutionMs() {
    let prev = performance.now();
    let best = Infinity;
    for (let i = 0; i < 30; i++) {
        let now = performance.now();
        while (now === prev) now = performance.now();
        const d = now - prev;
        if (d < best) best = d;
        prev = now;
    }
    return best;
}

class RustArm {
    constructor(name, instance) {
        this.name = name;
        this.kind = "rust";
        this.instance = instance;
        this.exports = instance.exports;
        this.srcPtr = 0;
        this.srcLen = 0;
        this.outPtr = 0;
        this.outLen = 0;
        this.source = null;
        this.target = null;
    }
    static async load(name, bytes) {
        const { instance } = await WebAssembly.instantiate(bytes, {});
        return new RustArm(name, instance);
    }
    prepare(src, outLen) {
        this.release();
        this.source = src;
        this.target = new Uint8Array(outLen);
        this.srcLen = src.length;
        this.outLen = outLen;
        this.srcPtr = this.exports.moss_alloc(src.length);
        this.outPtr = this.exports.moss_alloc(outLen);
        new Uint8Array(this.exports.memory.buffer).set(src, this.srcPtr);
    }
    call(count, stride, mode, filter) {
        new Uint8Array(this.exports.memory.buffer).set(this.source, this.srcPtr);
        const result = this.exports.moss_decode(
            this.srcPtr,
            this.srcLen,
            count,
            stride,
            mode,
            filter,
            this.outPtr,
            this.outLen,
        );
        this.target.set(new Uint8Array(this.exports.memory.buffer, this.outPtr, this.outLen));
        return result;
    }
    read() {
        return this.target;
    }
    release() {
        if (this.srcPtr) this.exports.moss_free(this.srcPtr, this.srcLen);
        if (this.outPtr) this.exports.moss_free(this.outPtr, this.outLen);
        this.srcPtr = 0;
        this.outPtr = 0;
    }
}

function shippedDecode(instance, fun, target, count, size, source, filter) {
    const sbrk = instance.exports.sbrk;
    const count4 = (count + 3) & ~3;
    const tp = sbrk(count4 * size);
    const sp = sbrk(source.length);
    const heap = new Uint8Array(instance.exports.memory.buffer);
    heap.set(source, sp);
    const res = fun(tp, count, size, sp, source.length);
    if (res === 0 && filter) filter(tp, count4, size);
    target.set(heap.subarray(tp, tp + count * size));
    sbrk(tp - sbrk(0));
    if (res !== 0) return false;
    return true;
}

class JsArm {
    constructor(name, instance) {
        this.name = name;
        this.kind = "upstream-js";
        this.instance = instance;
        this.target = null;
    }
    static async load(name, bytes) {
        const { instance } = await WebAssembly.instantiate(bytes, {});
        instance.exports.__wasm_call_ctors();
        return new JsArm(name, instance);
    }
    prepare(_src, outLen) {
        this.target = new Uint8Array(outLen);
    }
    call(count, stride, mode, filter, src) {
        const exp = this.instance.exports;
        const fun = mode === 0 ? exp.meshopt_decodeVertexBuffer
            : mode === 1 ? exp.meshopt_decodeIndexBuffer
            : exp.meshopt_decodeIndexSequence;
        const filt = filter === 1 ? exp.meshopt_decodeFilterOct
            : filter === 2 ? exp.meshopt_decodeFilterQuat
            : filter === 3 ? exp.meshopt_decodeFilterExp
            : null;
        return shippedDecode(this.instance, fun, this.target, count, stride, src, filt);
    }
    read() {
        return this.target;
    }
    release() {
        this.target = null;
    }
}

function rotate(list, shift) {
    const n = list.length;
    const k = shift % n;
    return list.slice(k).concat(list.slice(0, k));
}

export async function measure(arms, streams, bin, options) {
    const warmup = options.warmup;
    const repeats = options.repeats;
    const bytesOnly = options.bytesOnly;
    const timerResolutionMs = resolutionMs();
    const stopTiming = new Set();
    const rows = [];
    for (let s = 0; s < streams.length; s++) {
        const stream = streams[s];
        const src = bin.subarray(stream.offset, stream.offset + stream.encoded_len);
        const mode = MODES[stream.mode];
        const filter = FILTERS[stream.filter];
        const outLen = stream.count * stream.stride;
        const row = {
            id: stream.id,
            encoded_len: stream.encoded_len,
            count: stream.count,
            stride: stream.stride,
            mode: stream.mode,
            bytes: {},
            samples: {},
            excluded: false,
        };
        const refArm = arms.find((arm) => arm.name === "arm-m");
        refArm.prepare(src, outLen);
        const refN = refArm.call(stream.count, stream.stride, mode, filter);
        if (refN !== outLen) {
            row.excluded = true;
            row.bytes["arm-m"] = "fail";
            refArm.release();
            for (const arm of arms) arm.release();
            rows.push(row);
            continue;
        }
        const ref = refArm.read();
        row.bytes["arm-m"] = "match";
        const matched = [refArm];
        for (const arm of arms) {
            if (arm.name === "arm-m") continue;
            arm.prepare(src, outLen);
            let ok = false;
            let out = null;
            if (arm.kind === "rust") {
                ok = arm.call(stream.count, stream.stride, mode, filter) === outLen;
                if (ok) out = arm.read();
            } else {
                ok = arm.call(stream.count, stream.stride, mode, filter, src);
                if (ok) out = arm.read();
            }
            if (!ok || !out) {
                row.bytes[arm.name] = "fail";
                stopTiming.add(arm.name);
                arm.release();
                continue;
            }
            const at = firstDiff(ref, out);
            if (at !== -1) {
                row.bytes[arm.name] = "mismatch@" + at;
                stopTiming.add(arm.name);
                arm.release();
                continue;
            }
            row.bytes[arm.name] = "match";
            if (!stopTiming.has(arm.name)) matched.push(arm);
            else arm.release();
        }
        if (!bytesOnly && matched.length > 0) {
            const order = rotate(matched, s);
            for (let w = 0; w < warmup; w++) {
                for (const arm of order) {
                    if (arm.kind === "rust") arm.call(stream.count, stream.stride, mode, filter);
                    else arm.call(stream.count, stream.stride, mode, filter, src);
                }
            }
            for (const arm of matched) row.samples[arm.name] = [];
            for (let r = 0; r < repeats; r++) {
                for (const arm of rotate(order, r)) {
                    const t0 = performance.now();
                    if (arm.kind === "rust") arm.call(stream.count, stream.stride, mode, filter);
                    else arm.call(stream.count, stream.stride, mode, filter, src);
                    const t1 = performance.now();
                    row.samples[arm.name].push(t1 - t0);
                }
            }
        }
        for (const arm of arms) arm.release();
        rows.push(row);
    }
    return {
        timer_resolution_ms: timerResolutionMs,
        warmup,
        repeats,
        bytes_only: bytesOnly,
        stop_timing: [...stopTiming],
        rows,
    };
}

export async function loadArms(wasmBytes) {
    const arms = [];
    for (const spec of wasmBytes) {
        if (spec.kind === "rust") arms.push(await RustArm.load(spec.name, spec.bytes));
        else arms.push(await JsArm.load(spec.name, spec.bytes));
    }
    return arms;
}

async function nodeMain() {
    const { readFileSync, writeFileSync } = await import("node:fs");
    const root = "/mnt/linux-extra/moss-scratch/meshopt-decode/browser";
    const streams = JSON.parse(readFileSync(root + "/data/streams.json", "utf8"));
    const bin = new Uint8Array(readFileSync(root + "/data/streams.bin"));
    const manifest = JSON.parse(readFileSync(root + "/data/arms.json", "utf8"));
    const bytesOnly = process.argv.includes("--bytes-only");
    const loaded = [];
    for (const spec of manifest) {
        loaded.push({
            name: spec.name,
            kind: spec.kind,
            bytes: readFileSync(root + "/" + spec.file),
        });
    }
    const arms = await loadArms(loaded);
    const result = await measure(arms, streams, bin, { warmup: 5, repeats: 15, bytesOnly });
    result.host = "node";
    result.node = process.version;
    const out = bytesOnly ? root + "/data/node-bytes.json" : root + "/data/node-timing.json";
    writeFileSync(out, JSON.stringify(result));
    const fails = result.rows.filter((row) => row.excluded || Object.values(row.bytes).some((v) => v !== "match"));
    console.log(JSON.stringify({
        out,
        streams: result.rows.length,
        excluded: result.rows.filter((row) => row.excluded).length,
        not_match: fails.length,
        stop_timing: result.stop_timing,
        timer_resolution_ms: result.timer_resolution_ms,
    }));
}

if (typeof process !== "undefined" && process.argv[1] && process.argv[1].endsWith("harness.mjs")) {
    nodeMain();
}
