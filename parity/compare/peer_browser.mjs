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
    }
    static async load(name, bytes) {
        const { instance } = await WebAssembly.instantiate(bytes, {});
        return new RustArm(name, instance);
    }
    prepare(src, outLen) {
        this.release();
        this.srcLen = src.length;
        this.outLen = outLen;
        this.srcPtr = this.exports.moss_alloc(src.length);
        this.outPtr = this.exports.moss_alloc(outLen);
        new Uint8Array(this.exports.memory.buffer).set(src, this.srcPtr);
    }
    call(count, stride, mode, filter) {
        return this.exports.moss_decode(
            this.srcPtr,
            this.srcLen,
            count,
            stride,
            mode,
            filter,
            this.outPtr,
            this.outLen,
        );
    }
    read() {
        const heap = new Uint8Array(this.exports.memory.buffer);
        return heap.slice(this.outPtr, this.outPtr + this.outLen);
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

export async function loadArms(wasmBytes) {
    const arms = [];
    for (const spec of wasmBytes) {
        if (spec.kind === "rust") arms.push(await RustArm.load(spec.name, spec.bytes));
        else arms.push(await JsArm.load(spec.name, spec.bytes));
    }
    return arms;
}

function interval(ratios) {
 const logs=ratios.map(Math.log),n=logs.length,mu=logs.reduce((a,b)=>a+b)/n;
 const variance=logs.reduce((a,b)=>a+(b-mu)**2,0)/(n-1);
 const radius=({5:2.776445,20:2.093024})[n]*Math.sqrt(variance/n);
 return [Math.exp(mu-radius),Math.exp(mu+radius)];
}
async function receipt(data){const r=await fetch('/receipt',{method:'POST',body:JSON.stringify({utc:Date.now()/1000,...data})});if(!r.ok)throw Error('receipt write failed');}
export async function measure(arms, streams, bin, options) {
 const rows=[],bytesOnly=options.bytesOnly;
 const existing=options.existing||[];
 const resolution=resolutionMs();
 for (const stream of streams) {
  const waiting=await(await fetch('/train-waiters')).json();
  if(waiting.length){await receipt({event:'PAUSE',reason:'train first',waiting});return {rows,paused:true,bytes_only:bytesOnly,timer_resolution_ms:resolution};}
  const src=bin.subarray(stream.offset,stream.offset+stream.encoded_len),mode=MODES[stream.mode],filter=FILTERS[stream.filter],outLen=stream.count*stream.stride;
  const reference=new Uint8Array(await(await fetch(stream.reference_file)).arrayBuffer());
  const row={id:stream.id,mode:stream.mode,count:stream.count,stride:stream.stride,bytes:{},peers:{}};
  const call=arm=>arm.kind==='rust'?arm.call(stream.count,stream.stride,mode,filter):arm.call(stream.count,stream.stride,mode,filter,src);
  for(const arm of arms){arm.prepare(src,outLen);const result=call(arm),ok=arm.kind==='rust'?result===outLen:result;row.bytes[arm.name]=ok&&firstDiff(reference,arm.read())===-1?'identical':'invalid';}
  await receipt({event:'correctness',row});
  if(!bytesOnly){
   const ours=arms.find(a=>a.name==='O');
   for(const peer of arms.filter(a=>a.name!=='O')){
    const prior=existing.find(r=>r.id===stream.id&&r.peers?.[peer.name]?.completed);
    if(prior){row.peers[peer.name]=prior.peers[peer.name];continue;}
    if(row.bytes.O!=='identical'||row.bytes[peer.name]!=='identical')continue;
    for(let w=0;w<5;w++){call(ours);call(peer);}
    // Resident input, warm workspace; calibrate both arms to >=25ms per timed batch.
    const batch=(arm,n)=>{const t=performance.now();for(let k=0;k<n;k++)call(arm);return (performance.now()-t)*1e6/n;};
    const calibrate=arm=>{let n=1,elapsed=0;while(elapsed<25){const t=performance.now();for(let k=0;k<n;k++)call(arm);elapsed=performance.now()-t;if(elapsed<25)n*=2;if(n>1048576)break;}return n;};
    const iterations=Math.max(calibrate(ours),calibrate(peer)),samples=[];let first;
    for(let pair=0;pair<20;pair++){
     if(pair>=5&&!(first[0]<=1&&1<=first[1]))break;
     const order=pair%2?[peer,ours]:[ours,peer],ns={};
     for(const arm of order){ns[arm.name]=batch(arm,iterations);await receipt({event:'sample-arm',stream:stream.id,peer:peer.name,pair,arm:arm.name,ns:ns[arm.name],iterations});}
     const sample={utc:Date.now()/1000,pair,order:order.map(a=>a.name),ns,ratio:ns[peer.name]/ns.O};samples.push(sample);await receipt({event:'pair',stream:stream.id,peer:peer.name,sample});
     if(pair===4)first=interval(samples.map(s=>s.ratio));
    }
    row.peers[peer.name]={samples,iterations,ci95:interval(samples.map(s=>s.ratio)),gm:Math.exp(samples.reduce((a,s)=>a+Math.log(s.ratio),0)/samples.length),completed:true};
   }
  }
  for(const arm of arms)arm.release();rows.push(row);await receipt({event:'stream-complete',row});
 }
 return {rows,bytes_only:bytesOnly,paused:false,timer_resolution_ms:resolution};
}
