#!/usr/bin/env python3
"""Offline differential qualification. Records bind sources before execution."""
import argparse
import array
import base64
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import random
import statistics
import struct
import subprocess
import sys
import time
import zipfile

ROOT = Path(__file__).resolve().parent.parent
PIN = "4c203430ca565cb59a468a91922c76c208169536"
FLAGS = ["-std=c++17", "-O3", "-DNDEBUG", "-DMESHOPTIMIZER_NO_SIMD",
         "-fno-fast-math", "-ffp-contract=off"]
FAMILIES = {1: "vertex_cache", 2: "overdraw"}
ENV = {**os.environ, "CARGO_NET_OFFLINE": "true", "GIT_OPTIONAL_LOCKS": "0",
       "RUSTFLAGS": "", "CARGO_ENCODED_RUSTFLAGS": "", "RUSTC_WRAPPER": "", "RUSTC_WORKSPACE_WRAPPER": ""}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def sha(path):
    return digest(path.read_bytes())


def command(args, **kwargs):
    return subprocess.run([str(a) for a in args], check=True, env=ENV, **kwargs)


def text_command(args):
    return command(args, capture_output=True, text=True).stdout.strip()


def paths():
    if sys.platform != "linux" or platform.machine() != "x86_64":
        raise ValueError("this lane qualifies Linux x86-64 and wasm32 only")
    reference = Path(os.environ["MESHOPT_REFERENCE"]).resolve()
    target = Path(os.environ["CARGO_TARGET_DIR"]).resolve()
    if target == ROOT or ROOT in target.parents or target == reference or reference in target.parents:
        raise ValueError("build target must be isolated from both source trees")
    results = Path(os.environ.get("MESHOPT_RESULTS", ROOT / "parity/results")).resolve()
    results.mkdir(parents=True, exist_ok=True)
    (target / "tmp").mkdir(parents=True, exist_ok=True)
    ENV["TMPDIR"] = str(target / "tmp")
    return reference, target, results


def snapshot(reference):
    files = [ROOT / "Cargo.toml", ROOT / "Cargo.lock"]
    for directory in ["src", "tests", "parity", "fuzz"]:
        files += [p for p in (ROOT / directory).rglob("*") if p.is_file()
                  and "results" not in p.parts and p.suffix in {".rs", ".py", ".sh", ".cpp", ".cjs", ".toml"}]
    files += [ROOT / "parity/Cargo.lock", ROOT / "fuzz/Cargo.lock"]
    sources = {str(p.relative_to(ROOT)): sha(p) for p in sorted(set(files))}
    oracle_files = [*reference.glob("src/*"), *reference.glob("js/*"),
                    reference / "demo/tests.cpp", reference / "LICENSE.md"]
    oracle = {str(p.relative_to(reference)): sha(p) for p in sorted(oracle_files) if p.is_file()}
    metadata = json.loads(text_command(["cargo", "metadata", "--offline", "--locked", "--format-version", "1"]))
    dependencies = {}
    for package in metadata["packages"]:
        if package["source"] is None:
            continue
        directory = Path(package["manifest_path"]).parent
        for p in sorted(directory.rglob("*")):
            if p.is_file():
                dependencies[f'{package["name"]}-{package["version"]}/{p.relative_to(directory)}'] = sha(p)
    return {"rust_and_harness": sources, "upstream": oracle, "dependencies": dependencies}


def reference_check():
    command([ROOT / "parity/check-reference.sh"])


def build(reference, target, wasm=True):
    reference_check()
    before = snapshot(reference)
    build_dir = target / "qualification-build"
    build_dir.mkdir(parents=True, exist_ok=True)
    cpp = build_dir / "reference"
    compiler = os.environ.get("CXX", "c++")
    cpp_command = [compiler, *FLAGS, "-I", reference / "src", ROOT / "parity/reference.cpp",
                   reference / "src/vcacheoptimizer.cpp", reference / "src/overdrawoptimizer.cpp",
                   reference / "src/allocator.cpp", "-o", cpp]
    command(cpp_command)
    command(["cargo", "build", "--offline", "--locked", "--release", "--manifest-path", ROOT / "parity/Cargo.toml"])
    binaries = {"cpp": cpp, "rust": target / "release/meshopt-driver"}
    if wasm:
        command(["cargo", "build", "--offline", "--locked", "--release", "--target", "wasm32-unknown-unknown",
                 "--manifest-path", ROOT / "parity/Cargo.toml", "--lib"])
        binaries["wasm"] = target / "wasm32-unknown-unknown/release/meshoptimizer_parity.wasm"
    if before != snapshot(reference):
        raise ValueError("source changed during build")
    def redact(arg):
        return str(arg).replace(str(target), "$CARGO_TARGET_DIR").replace(str(reference), "$MESHOPT_REFERENCE").replace(str(ROOT), ".")
    identities = {
        "source_sha256": before, "executable_sha256": {n: sha(p) for n, p in binaries.items()},
        "rustc": text_command(["rustc", "-Vv"]), "cargo": text_command(["cargo", "-V"]),
        "cxx": text_command([compiler, "--version"]), "cxx_command": [redact(a) for a in cpp_command],
        "rust_profile": "release; default generic target; no fast math or contraction; Rust flags and compiler wrappers empty",
        "math": "libm 0.2.16 sqrtf in std and no_std; exact f32 bits",
        "fp_environment": "C++ driver checks FE_TONEAREST and gradual underflow; no x87 target",
        "hardware": platform.uname()._asdict(), "cpu": cpu(), "logical_cpus": os.cpu_count(),
        "node": text_command(["node", "--version"]) if wasm else None,
        "reference_revision": PIN, "upstream_tag": "v1.3", "upstream_src_identical_to_tag": True,
        "rust_revision": "uncommitted working sources; identity is the SHA-256 manifest",
    }
    return binaries, identities


def cpu():
    path = Path("/proc/cpuinfo")
    if path.exists():
        for line in path.read_text().splitlines():
            if line.startswith("model name"):
                return line.partition(":")[2].strip()
    return platform.processor()


def check_unchanged(reference, binaries, identities):
    reference_check()
    if snapshot(reference) != identities["source_sha256"] or {n: sha(p) for n, p in binaries.items()} != identities["executable_sha256"]:
        raise ValueError("source or executable changed during execution")


def packed(values, kind):
    a = array.array(kind, values)
    if sys.byteorder != "little":
        a.byteswap()
    return a.tobytes()


def message(op, positions, indices, threshold=1.05, mode=0, samples=0):
    return struct.pack("<4sIIIfII", b"MO01", op, len(positions) // 3, len(indices), threshold, mode, samples) + packed(positions, "f") + packed(indices, "I")


def response(data, count, samples=0):
    if len(data) != 16 + count * 4 + samples * 8 or struct.unpack_from("<4sIII", data) != (b"MR01", 0, count, samples):
        raise ValueError("malformed response or status")
    return data[16:16 + count * 4], list(struct.unpack_from(f"<{samples}d", data, 16 + count * 4))


def execute(binary, data):
    result = subprocess.run([str(binary)], input=data, capture_output=True, env=ENV, timeout=180)
    if result.returncode:
        raise ValueError(f"driver exit {result.returncode}: {result.stderr.decode(errors='replace')}")
    return result.stdout


class Wasm:
    def __init__(self, binary):
        self.process = subprocess.Popen(["node", str(ROOT / "parity/wasm.cjs"), str(binary)],
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE, env=ENV)

    def execute(self, data):
        self.process.stdin.write(base64.b64encode(data) + b"\n")
        self.process.stdin.flush()
        line = self.process.stdout.readline()
        if not line:
            raise ValueError("missing WASM response")
        return base64.b64decode(line.strip(), validate=True)

    def close(self):
        self.process.stdin.close()
        if self.process.wait(timeout=20):
            raise ValueError("WASM runtime failed")


def grid(triangles):
    side = math.ceil(math.sqrt(triangles / 2))
    p = array.array("f")
    for y in range(side + 1):
        for x in range(side + 1):
            p.extend((float(x), float(y), float((x * 7 + y * 11) % 31) / 32))
    ib = array.array("I")
    for y in range(side):
        for x in range(side):
            a = y * (side + 1) + x
            ib.extend((a, a + 1, a + side + 1, a + 1, a + side + 2, a + side + 1))
    del ib[triangles * 3:]
    return p, ib


def fixtures():
    yield "upstream-emptyMesh", [], [], 1.0
    yield "empty-with-unused-vertices", [0., -0., 1.], [], 1.05
    yield "single", [0., 0., 0., 1., 0., 0., 0., 1., 0.], [0, 1, 2], 1.05
    yield "degenerate", [0.] * 9, [0, 0, 0, 0, 1, 1, 1, 0, 1], 1.05
    yield "js-encoder-reorderMesh-topology", [float(i % 3) for i in range(18)], [4, 2, 5, 3, 1, 4, 0, 1, 3, 1, 2, 4], 1.05
    p, ib = grid(128)
    for threshold in [-1., 0., 0.5, 1., 1.01, 1.05, 1.1, 2.]:
        yield f"grid-threshold-{threshold}", p, ib, threshold
    yield "disconnected-with-unused-vertex", [float(i % 7 - 3) for i in range(93)], list(range(30)), 1.05
    yield "duplicate-triangles", [0.] * 9, [0, 1, 2] * 100, 1.05
    yield "signed-zero-and-subnormal", [0., -0., 1e-40, 1e-20, 0., 0., 0., 1e-20, 0.], [0, 1, 2], 1.05
    p, ib = grid(333334)
    yield "million-indices-grid", p, ib, 1.05


def random_mesh(rng, number):
    if number == 0:
        return (*grid(333334), 1.05, "million-indices-grid")
    if number % 100 == 0:
        return (*grid(10000 + number), 1.05, "medium-grid")
    v = rng.randint(0, 160)
    triangles = rng.choice([0, 1, 2, 7, 32, 128, 512]) if v else 0
    p = [rng.randint(-10000, 10000) / 128 for _ in range(v * 3)]
    kind = number % 6
    if kind == 0:
        p = [0.] * (v * 3)
    elif kind == 1:
        p = [x * 1e-10 for x in p]
    elif kind == 2:
        p = [x * 1e5 for x in p]
    ib = [rng.randrange(v) for _ in range(triangles * 3)]
    if kind == 3 and v:
        ib = [rng.randrange(min(v, 3)) for _ in ib]
    elif kind == 4 and v:
        ib = [i % v for i in range(triangles * 3)]
    elif kind == 5 and v:
        for start in range(0, len(ib), 9):
            ib[start:start + 3] = [ib[start]] * 3
    return p, ib, rng.choice([-1., 0., 0.5, 1., 1.01, 1.05, 1.1, 2.]), f"random-kind-{kind}"


def retain(archive, name, data):
    archive.writestr(name, data)
    return {"member": name, "bytes": len(data), "sha256": digest(data)}


def differential(args):
    reference, target, results = paths()
    binaries, identities = build(reference, target)
    record = {"schema": 1, "command": args.action, "phase": "0.1-lane1", "profile": "scalar-strict",
              "seed": args.seed, "started_unix": time.time(), "identities": identities, "cases": [],
              "mismatches": 0, "counts": {name: 0 for name in FAMILIES.values()},
              "required_cases_per_family": args.cases_per_family if args.action == "sweep" else None,
              "wasm_reference": "qualified native Rust on identical messages", "passed": False}
    archive_path = results / f"{args.action}-buffers.zip"
    wasm = Wasm(binaries["wasm"])
    try:
        with zipfile.ZipFile(archive_path, "w", zipfile.ZIP_DEFLATED, compresslevel=6) as archive:
            rng = random.Random(args.seed)
            cases = fixtures() if args.action == "run" else (
                (f"seed-{args.seed}-case-{i}-{kind}", p, ib, threshold)
                for i in range(args.cases_per_family)
                for p, ib, threshold, kind in [random_mesh(rng, i)])
            for case_id, p, ib, threshold in cases:
                for op, family in FAMILIES.items():
                    # Overdraw's documented input is the standard cache optimizer's output.
                    # The exact cache comparison occurs first on the same topology.
                    if op == 2 and len(ib):
                        cached, _ = response(execute(binaries["rust"], message(1, p, ib)), len(ib))
                        ib = array.array("I")
                        ib.frombytes(cached)
                        if sys.byteorder != "little":
                            ib.byteswap()
                    data = message(op, p, ib, threshold)
                    outputs = {"cpp": execute(binaries["cpp"], data), "rust": execute(binaries["rust"], data)}
                    outputs["wasm"] = wasm.execute(data)
                    for out in outputs.values():
                        response(out, len(ib))
                    match = outputs["cpp"] == outputs["rust"] == outputs["wasm"]
                    prefix = f"{family}/{case_id}"
                    record["cases"].append({"id": case_id, "family": family, "vertices": len(p) // 3,
                        "indices": len(ib), "threshold_bits": struct.unpack("<I", struct.pack("<f", threshold))[0],
                        "input": retain(archive, prefix + ".input", data),
                        "outputs": {n: retain(archive, prefix + "." + n, out) for n, out in outputs.items()}, "match": match})
                    record["counts"][family] += 1
                    if not match:
                        record["mismatches"] += 1
                        raise ValueError(f"exact parity failure: {prefix}; buffers retained")
                if args.action == "sweep" and sum(record["counts"].values()) % 500 == 0:
                    print(f'sweep: {record["counts"]}', flush=True)
            if args.action == "run":
                math_probe(record, archive, binaries, wasm, args.seed)
                record["protocol_negative_controls"] = protocol_checks(binaries)
        check_unchanged(reference, binaries, identities)
        record["passed"] = record["mismatches"] == 0 and all(record["counts"].values())
    finally:
        wasm.close()
        record["finished_unix"] = time.time()
        record["artifacts"] = {archive_path.name: sha(archive_path)}
        (results / f"{args.action}.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps({k: record[k] for k in ["command", "counts", "mismatches", "passed"]}))


def math_probe(record, archive, binaries, wasm, seed):
    rng = random.Random(seed)
    values = [0, 0x80000000, 1, 0x7fffff, 0x800000, 0x3f800000, 0x7f7fffff]
    values += [rng.randrange(0x7f800000) for _ in range(65536)]
    data = message(3, [], values)
    outputs = {n: execute(binaries[n], data) for n in ["cpp", "rust"]}
    outputs["wasm"] = wasm.execute(data)
    for out in outputs.values():
        response(out, len(values))
    match = len(set(outputs.values())) == 1
    record["math_probe"] = {"count": len(values), "match": match, "input": retain(archive, "math.input", data),
                            "outputs": {n: retain(archive, "math." + n, out) for n, out in outputs.items()}}
    if not match:
        raise ValueError("sqrtf bit parity failure")


def protocol_checks(binaries):
    valid = message(1, [0.] * 9, [0, 1, 2])
    cases = {"truncated-header": b"MO01", "wrong-version": b"XX01" + valid[4:],
             "truncated-body": valid[:-1], "extra-byte": valid + b"x",
             "invalid-topology": message(1, [0.] * 9, [0, 1]),
             "invalid-index": message(1, [0.] * 9, [0, 1, 3]),
             "nonfinite-threshold": message(2, [0.] * 9, [0, 1, 2], float("nan")),
             "nonfinite-position": message(2, [float("inf")] * 9, [0, 1, 2]),
             "overflow-count": struct.pack("<4sIIIfII", b"MO01", 1, 0xffffffff, 0xffffffff, 1., 0, 0)}
    for name, data in cases.items():
        for backend in ["rust", "cpp"]:
            result = subprocess.run([str(binaries[backend])], input=data, capture_output=True, env=ENV, timeout=10)
            if result.returncode == 0 or result.stdout:
                raise ValueError(f"protocol accepted {name} on {backend}")
    return {"cases": list(cases), "backends": ["rust", "cpp"], "passed": True}


def benchmark(args):
    if args.enforce:
        raise ValueError("lane 1 records performance; enforcement is a later qualification gate")
    reference, target, results = paths()
    binaries, identities = build(reference, target, wasm=False)
    p, ib = grid(1000000)
    record = {"schema": 1, "command": "benchmark", "identities": identities, "started_unix": time.time(),
              "triangles": 1000000, "vertices": len(p) // 3, "samples": 10, "threads": 1,
              "timing": "validation, output/scratch allocation and algorithm; excludes generation, I/O and startup",
              "cpp_reference": "scalar-strict; shipped lane modules have no explicit SIMD paths", "workloads": {}, "passed": False}
    artifact = results / "benchmark-buffers.zip"
    with zipfile.ZipFile(artifact, "w", zipfile.ZIP_DEFLATED) as archive:
        for op, family in FAMILIES.items():
            if op == 2:
                cached, _ = response(execute(binaries["rust"], message(1, p, ib)), len(ib))
                ib = array.array("I")
                ib.frombytes(cached)
                if sys.byteorder != "little":
                    ib.byteswap()
            samples = {"rust": [], "cpp": []}
            data = message(op, p, ib, mode=1, samples=1)
            input_artifact = retain(archive, family + ".input", data)
            outputs = []
            for pair in range(10):
                order = ["rust", "cpp"] if pair % 2 == 0 else ["cpp", "rust"]
                pair_outputs = {}
                for backend in order:
                    raw = execute(binaries[backend], data)
                    out, times = response(raw, len(ib), 1)
                    if not all(math.isfinite(t) and t > 0 for t in times):
                        raise ValueError("invalid benchmark time")
                    samples[backend] += times
                    pair_outputs[backend] = out
                    outputs.append(retain(archive, f"{family}-{pair}-{backend}.output", raw))
                if pair_outputs["rust"] != pair_outputs["cpp"]:
                    raise ValueError("benchmark outputs differ")
            stats = {n: {"median_seconds": statistics.median(ts), "min_seconds": min(ts), "max_seconds": max(ts),
                         "stdev_seconds": statistics.stdev(ts), "raw_seconds": ts} for n, ts in samples.items()}
            ratio = stats["rust"]["median_seconds"] / stats["cpp"]["median_seconds"]
            record["workloads"][family] = {"stats": stats, "rust_cpp_ratio": ratio, "input": input_artifact, "outputs": outputs}
    check_unchanged(reference, binaries, identities)
    record.update(passed=True, finished_unix=time.time(), artifacts={artifact.name: sha(artifact)})
    (results / "benchmark.json").write_text(json.dumps(record, indent=2) + "\n")
    for family, result in record["workloads"].items():
        print(f'{family}: Rust {result["stats"]["rust"]["median_seconds"]:.6f}s, C++ {result["stats"]["cpp"]["median_seconds"]:.6f}s, ratio {result["rust_cpp_ratio"]:.3f}')


def js(args):
    reference, target, results = paths()
    reference_check()
    before = snapshot(reference)
    suites = []
    for name in ["encoder", "decoder", "simplifier", "clusterizer", "tangents"]:
        source = reference / f"js/meshopt_{name}.test.js"
        result = command(["node", source], capture_output=True, text=True, timeout=120)
        suites.append({"suite": name, "source_sha256": sha(source), "exit": result.returncode,
                       "stdout": result.stdout, "stderr": result.stderr,
                       "coverage": "upstream sanity only; no direct shipped-module fixture"})
    if before != snapshot(reference):
        raise ValueError("source changed during JS sanity")
    record = {"schema": 1, "passed": True, "node": text_command(["node", "--version"]), "source_sha256": before,
              "suites": suites, "applicability": "Only reorderMesh invokes a cache optimizer; its optsize=true selects Strip (0.1.x). Its input topology is also in the Rust lane corpus. No JS overdraw fixture exists."}
    (results / "js.json").write_text(json.dumps(record, indent=2) + "\n")
    print("all five unchanged upstream JS suites passed; direct lane fixture coverage documented")


def verify_record(record, results):
    if record.get("passed") is not True:
        raise ValueError("missing passing record")
    if record.get("command") in {"run", "sweep", "benchmark"} and set(record.get("artifacts", {})) != {record["command"] + "-buffers.zip"}:
        raise ValueError("missing required buffer archive")
    for name, expected in record.get("artifacts", {}).items():
        path = results / name
        if path.parent != results or sha(path) != expected:
            raise ValueError("artifact identity failure")
        with zipfile.ZipFile(path) as archive:
            def verify(item):
                data = archive.read(item["member"])
                if len(data) != item["bytes"] or digest(data) != item["sha256"]:
                    raise ValueError("buffer identity failure")
            for case in record.get("cases", []):
                verify(case["input"])
                for out in case["outputs"].values():
                    verify(out)
                data = archive.read(case["input"]["member"])
                header = struct.unpack_from("<4sIIIfII", data)
                if header[0] != b"MO01" or header[1] not in FAMILIES or FAMILIES[header[1]] != case["family"] or header[2:4] != (case["vertices"], case["indices"]):
                    raise ValueError("case metadata does not describe retained input")
                if set(case["outputs"]) != {"cpp", "rust", "wasm"}:
                    raise ValueError("missing output backend")
                outputs = [archive.read(out["member"]) for out in case["outputs"].values()]
                for out in outputs:
                    response(out, case["indices"])
                if not case["match"] or len(set(outputs)) != 1:
                    raise ValueError("recorded mismatch")
            if "math_probe" in record:
                verify(record["math_probe"]["input"])
                for out in record["math_probe"]["outputs"].values():
                    verify(out)
                if set(record["math_probe"]["outputs"]) != {"cpp", "rust", "wasm"} or len({archive.read(out["member"]) for out in record["math_probe"]["outputs"].values()}) != 1:
                    raise ValueError("math outputs not qualified")
            for work in record.get("workloads", {}).values():
                verify(work["input"])
                for out in work["outputs"]:
                    verify(out)
                if len(work["outputs"]) != 20 or any(len(s["raw_seconds"]) != 10 for s in work["stats"].values()):
                    raise ValueError("missing benchmark samples")


def report(args):
    reference, _, results = paths()
    current = snapshot(reference)
    records = {}
    for name in ["run", "sweep", "benchmark", "js"]:
        record = json.loads((results / f"{name}.json").read_text())
        verify_record(record, results)
        bound = record.get("identities", {}).get("source_sha256", record.get("source_sha256"))
        if bound != current:
            raise ValueError(f"stale {name} record")
        records[name] = record
    for name in ["run", "sweep"]:
        counts = records[name]["counts"]
        actual = {f: sum(c["family"] == f for c in records[name]["cases"]) for f in FAMILIES.values()}
        if counts != actual or any(n < (args.cases_per_family if name == "sweep" else 17) for n in counts.values()):
            raise ValueError("missing required cases")
        for family in FAMILIES.values():
            cases = [c for c in records[name]["cases"] if c["family"] == family]
            if len({c["id"] for c in cases}) != len(cases) or max(c["indices"] for c in cases) < 1000000:
                raise ValueError("missing distinct cases or large mesh coverage")
            if name == "run" and {c["id"] for c in cases} != {case_id for case_id, _, _, _ in fixtures()}:
                raise ValueError("missing required fixture")
    if not records["run"]["math_probe"]["match"] or records["sweep"]["mismatches"]:
        raise ValueError("unqualified outputs")
    if {s["suite"] for s in records["js"]["suites"]} != {"encoder", "decoder", "simplifier", "clusterizer", "tangents"} or any(s["exit"] for s in records["js"]["suites"]):
        raise ValueError("missing upstream sanity suite")
    fuzz = json.loads((results / "fuzz.json").read_text())
    if not fuzz["identities_unchanged"] or fuzz["budget_seconds"] < 600 or {r["target"] for r in fuzz["runs"]} != set(FAMILIES.values()) or any(r["exit"] for r in fuzz["runs"]):
        raise ValueError("fuzz smoke incomplete")
    if fuzz["dependency_sha256"] != current["dependencies"] or any(r["statistics"]["seconds"] < 600 or r["statistics"]["executions"] < 1 or r["statistics"]["successes"] < 1 or r["statistics"]["crashes"] != 0 for r in fuzz["runs"]):
        raise ValueError("fuzz identities or executed budget incomplete")
    for name, expected in fuzz["source_sha256"].items():
        if sha(ROOT / name) != expected:
            raise ValueError("stale fuzz record")
    gates = json.loads((results / "gates.json").read_text())
    if not gates["passed"] or any(c["exit"] for c in gates["commands"]) or not gates["identities_unchanged"]:
        raise ValueError("build/package gates incomplete")
    for name, expected in gates["source_sha256"].items():
        if sha(ROOT / name) != expected:
            raise ValueError("stale build/package gates")
    if sha(results / gates["package_artifact"]["name"]) != gates["package_artifact"]["sha256"]:
        raise ValueError("package artifact identity failure")
    records["fuzz"] = fuzz
    records["gates"] = gates
    summary = {"schema": 1, "scope": "lane 1 foundation; not complete release 0.1", "passed": True,
               "counts": {n: records[n]["counts"] for n in ["run", "sweep"]}, "mismatches": 0,
               "record_sha256": {n + ".json": sha(results / (n + ".json")) for n in records},
               "targets": ["linux-x86_64", "wasm32 (native Rust identity)"], "benchmark": records["benchmark"]["workloads"]}
    (ROOT / "parity/MEASURED_RESULTS.json").write_text(json.dumps(summary, indent=2) + "\n")
    lines = ["# Measured lane 1 parity", "", "Exact output comparison with scalar-strict C++ 1.3 and executed wasm32 Rust.", "",
             "| Function | Corpus | Seeded sweep | Mismatches |", "|---|---:|---:|---:|"]
    for f in FAMILIES.values():
        lines.append(f'| {f} | {records["run"]["counts"][f]} | {records["sweep"]["counts"][f]} | 0 |')
    lines += ["", f'Seed: {records["sweep"]["seed"]}. Both functions include 1,000,002-index cases.', "",
              f'Square-root probe: {records["run"]["math_probe"]["count"]} exact f32 results, including signed zero and subnormals.', "",
              "All five upstream JS suites passed unchanged; their applicable-input inventory is in COVERAGE.md.", "",
              "Stable fuzz smoke ran 600 seconds per module. See results/fuzz.json for executions and the instrumentation limits.", "",
              "These are Linux x86-64 and wasm32 lane records. AArch64, complete 0.1 simplification and the release fuzz/performance gates remain outside this lane.", "",
              "The records and compressed input/output buffers in results/ retain SHA-256 identities. report.sh rejects stale source and missing or changed artifacts."]
    (ROOT / "parity/MEASURED_PARITY.md").write_text("\n".join(lines) + "\n")
    lines = ["# Measured lane 1 performance", "", "Single thread, 1,000,000 triangles; medians of ten interleaved pairs after warm-up.", "",
             "Validation, output and scratch allocation and execution are timed; generation, I/O and process startup are excluded.", "",
             "| Function | Rust (ms) | C++ (ms) | Rust/C++ |", "|---|---:|---:|---:|"]
    for family, work in records["benchmark"]["workloads"].items():
        lines.append(f'| {family} | {work["stats"]["rust"]["median_seconds"] * 1000:.3f} | {work["stats"]["cpp"]["median_seconds"] * 1000:.3f} | {work["rust_cpp_ratio"]:.3f} |')
    lines += ["", "Allocating APIs against scalar-strict C++. These modules have no explicit upstream SIMD paths. Raw samples, dispersion, hardware and executable identities are in results/benchmark.json.", "",
              "Recorded only: this lane does not enforce the RFC's later release performance bars or qualify all workload sizes and API variants."]
    (ROOT / "parity/MEASURED_PERFORMANCE.md").write_text("\n".join(lines) + "\n")
    print("verified source identities, required counts and retained artifacts; generated measured reports")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=["run", "sweep", "benchmark", "js", "report", "qualify"])
    parser.add_argument("--phase", choices=["0.1"], default="0.1")
    parser.add_argument("--profile", choices=["scalar-strict"], default="scalar-strict")
    parser.add_argument("--seed", type=int, default=20261002)
    parser.add_argument("--cases-per-family", type=int, default=2000)
    parser.add_argument("--require-coverage", action="store_true")
    parser.add_argument("--verify-artifacts", action="store_true")
    parser.add_argument("--enforce", action="store_true")
    args = parser.parse_args()
    if args.cases_per_family < 1000:
        parser.error("at least 1000 cases per family are required (2000 by default)")
    if args.action in {"run", "sweep"}:
        differential(args)
    elif args.action == "benchmark":
        benchmark(args)
    elif args.action == "js":
        js(args)
    elif args.action == "qualify":
        report(args)
    else:
        report(args)


if __name__ == "__main__":
    main()
