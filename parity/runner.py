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
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parent.parent
PIN = "4c203430ca565cb59a468a91922c76c208169536"
FLAGS = ["-std=c++17", "-O3", "-DNDEBUG", "-DMESHOPTIMIZER_NO_SIMD",
         "-fno-fast-math", "-ffp-contract=off"]
FAMILIES = {1: "vertex_cache", 2: "overdraw", 4: "simplify", 5: "simplify_with_attributes", 6: "simplify_scale"}
UPSTREAM_FIXTURES = {'js-getScale-5': 'simplify_scale', 'js-simplify-0': 'simplify', 'js-simplify-2': 'simplify', 'js-simplify16-1': 'simplify', 'js-simplifyWithAttributes-3': 'simplify_with_attributes', 'js-simplifyWithAttributes-4': 'simplify_with_attributes', 'native-simplify-0': 'simplify', 'native-simplifyAttr-10': 'simplify_with_attributes', 'native-simplifyAttr-zero-weight-11': 'simplify_with_attributes', 'native-simplifyDegenerate-8': 'simplify', 'native-simplifyErrorAbsolute-18': 'simplify', 'native-simplifyFlip-6': 'simplify', 'native-simplifyLockBorder-9': 'simplify', 'native-simplifyLockFlags-12': 'simplify_with_attributes', 'native-simplifyLockFlagsSeam-13': 'simplify_with_attributes', 'native-simplifyLockFlagsSeam-14': 'simplify_with_attributes', 'native-simplifyLockFlagsSeam-15': 'simplify_with_attributes', 'native-simplifyLockFlagsSeam-16': 'simplify_with_attributes', 'native-simplifyLockFlagsSeam-17': 'simplify_with_attributes', 'native-simplifyScale-7': 'simplify_scale', 'native-simplifySeam-19': 'simplify', 'native-simplifySeam-20': 'simplify_with_attributes', 'native-simplifySeamAttr-22': 'simplify_with_attributes', 'native-simplifySeamFake-21': 'simplify', 'native-simplifyStuck-1': 'simplify', 'native-simplifyStuck-2': 'simplify', 'native-simplifyStuck-3': 'simplify', 'native-simplifyStuck-4': 'simplify', 'native-simplifyStuck-5': 'simplify'}
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


def artifacts_path():
    # Default follows the isolated build target, never the source tree.
    path = Path(os.environ.get("MESHOPT_ARTIFACTS", Path(os.environ["CARGO_TARGET_DIR"]) / "parity-artifacts")).resolve()
    if path == ROOT or ROOT in path.parents:
        raise ValueError("MESHOPT_ARTIFACTS must be outside the repository")
    path.mkdir(parents=True, exist_ok=True)
    return path


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
                   reference / "src/allocator.cpp", reference / "src/simplifier.cpp", "-o", cpp]
    command(cpp_command)
    command(["cargo", "build", "--offline", "--locked", "--release", "--manifest-path", ROOT / "parity/Cargo.toml"])
    binaries = {"cpp": cpp, "rust": target / "release/meshopt-driver"}
    if wasm:
        command(["cargo", "build", "--offline", "--locked", "--release", "--target", "wasm32-unknown-unknown",
                 "--manifest-path", ROOT / "parity/Cargo.toml", "--lib"])
        binaries["wasm"] = target / "wasm32-unknown-unknown/release/meshoptimizer_parity.wasm"
    fixture_dir = target / "upstream-fixtures"
    fixture_dir.mkdir(exist_ok=True)
    for path in fixture_dir.glob("*.input"):
        path.unlink()
    generated = build_dir / "native-fixtures.cpp"
    command(["python3", ROOT / "parity/native-fixtures.py", reference, generated])
    fixture_binary = build_dir / "native-fixtures"
    command([compiler, *[f for f in FLAGS if f != "-DNDEBUG"], "-I", reference / "src", generated,
             reference / "src/simplifier.cpp", reference / "src/allocator.cpp", "-o", fixture_binary])
    command([fixture_binary, fixture_dir])
    command(["node", ROOT / "parity/js-fixtures.cjs", reference, fixture_dir])
    binaries["fixture_exporter"] = fixture_binary
    if before != snapshot(reference):
        raise ValueError("source changed during build")
    def redact(arg):
        return str(arg).replace(str(target), "$CARGO_TARGET_DIR").replace(str(reference), "$MESHOPT_REFERENCE").replace(str(ROOT), ".")
    identities = {
        "source_sha256": before, "executable_sha256": {n: sha(p) for n, p in binaries.items()},
        "rustc": text_command(["rustc", "-Vv"]), "cargo": text_command(["cargo", "-V"]),
        "cxx": text_command([compiler, "--version"]), "cxx_command": [redact(a) for a in cpp_command],
        "rust_profile": "release; default generic target; no fast math or contraction; Rust flags and compiler wrappers empty",
        "release_profile_configuration": {name: tomllib.loads((ROOT/name).read_text()).get('profile', {}).get('release', {})
                                           for name in ['Cargo.toml', 'parity/Cargo.toml', 'fuzz/Cargo.toml']},
        "math": "libm 0.2.16 sqrtf in std and no_std; exact f32 bits",
        "fp_environment": "C++ driver checks FE_TONEAREST and gradual underflow; no x87 target",
        "hardware": {k: v for k, v in platform.uname()._asdict().items() if k != "node"}, "cpu": cpu(), "logical_cpus": os.cpu_count(),
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


def message(op, positions, indices, threshold=1.05, mode=0, samples=0, variant=0):
    data = struct.pack("<4sIIIfII", b"MO01", op, len(positions) // 3, len(indices), threshold, mode, samples) + packed(positions, "f") + packed(indices, "I")

    if op >= 4:
        ratios = [0., .1, .125, .25, .5, .75, .9, 1.]
        errors = [0., .0001, .001, .01, .05, .1, .25, .5, .75, .9, 1., 1000.]
        options = [0, 32, 1, 4, 16, 64, 33, 36]
        ac = [0, 1, 3, 5, 8, 12, 13, 32][variant % 8] if op == 5 else 0
        weights = [[0., .01, .1, .5, 1., 2., 8., 10.][(variant + k) % 8] for k in range(ac)]
        if ac in {8, 12} and variant % 2 == 0:
            weights = [2.,2.,2.,1.,1.,1.,1.,1.] + ([8.]*4 if ac == 12 else [])
        attributes = []
        for i in range(len(positions)//3):
            x,y,z = positions[i*3:i*3+3]
            length = math.sqrt(x*x+y*y+z*z) or 1.
            values = [x/length,y/length,z/length, x*.125+(i%2),y*.125, 1.,0.,0., (i%3)/2.,(i%5)/4.,(i%7)/6.,1., 1. if i%2 else -1.]
            attributes.extend(values[k%13] for k in range(ac))
        flags = [([0, 0, 0, 0, 1, 2, 4, 3, 5, 6, 7][(i+variant)%11] if variant%3==0 else 0) for i in range(len(positions)//3)]
        ratio = ratios[variant%len(ratios)]
        error = errors[(variant//7)%len(errors)]
        if variant % 4 == 1:
            error = max(1. - ratio, .05)  # cooker
        elif variant % 4 == 2 and positions:
            extent = max(max(positions[k::3])-min(positions[k::3]) for k in range(3))
            error = max(1. - ratio, .05) * max(extent, 1e-6)  # editor as currently called
        data += struct.pack("<IfII", int(len(indices)*ratio), error, options[variant%len(options)], ac)
        data += packed(weights, "f") + packed(attributes, "f") + packed(flags, "I")
    return data


def response(data, count=None, samples=0):
    if count is None:
        count = struct.unpack_from("<I", data, 8)[0]
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


def sphere(side=12, split=False):
    p = []
    for y in range(side+1):
        for x in range(side):
            t=math.pi*y/side; f=2*math.pi*x/side
            p.extend((math.sin(t)*math.cos(f), math.sin(t)*math.sin(f), math.cos(t)))
    ib=[]
    for y in range(side):
        for x in range(side):
            a=y*side+x; b=y*side+(x+1)%side
            ib.extend((a,b,a+side,b,b+side,a+side))
    if split:
        p=[p[v*3+k] for v in ib for k in range(3)]
        ib=list(range(len(ib)))
    return p,ib


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
    for repeat in range(16):
        for split in [False, True]:
            p,ib=sphere(12,split)
            yield f"sphere-{'split-normal-uv-color-tangent' if split else 'shared'}-{repeat}",p,ib,1.05
    p,ib=grid(128)
    yield "nonmanifold-shared-edge",p,list(ib)+list(ib[:12])+[0,1,2],1.05


def random_mesh(rng, number):
    if number == 0:
        return (*grid(333334), 1.05, "million-indices-grid")
    if number % 100 == 0:
        return (*grid(10000 + number), 1.05, "medium-grid")
    if number % 5 == 0:
        return (*sphere(rng.randint(3,14), number%10==0),1.05,"sphere-seams")
    if number % 5 == 1:
        return (*grid(rng.randint(2,128)*2),1.05,"small-grid")
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
    archive_path = artifacts_path() / f"{args.action}-buffers.zip"
    wasm = Wasm(binaries["wasm"])
    try:
        with zipfile.ZipFile(archive_path, "w", zipfile.ZIP_DEFLATED, compresslevel=6) as archive:
            rng = random.Random(args.seed)
            cases = fixtures() if args.action == "run" else (
                (f"seed-{args.seed}-case-{i}-{kind}", p, ib, threshold)
                for i in range(args.cases_per_family)
                for p, ib, threshold, kind in [random_mesh(rng, i)])
            for variant, (case_id, p, ib, threshold) in enumerate(cases):
                for op, family in FAMILIES.items():
                    # Overdraw's documented input is the standard cache optimizer's output.
                    # The exact cache comparison occurs first on the same topology.
                    if op == 2 and len(ib):
                        cached, _ = response(execute(binaries["rust"], message(1, p, ib)), len(ib))
                        ib = array.array("I")
                        ib.frombytes(cached)
                        if sys.byteorder != "little":
                            ib.byteswap()
                    data = message(op, p, ib, threshold, variant=variant)
                    outputs = {"cpp": execute(binaries["cpp"], data), "rust": execute(binaries["rust"], data)}
                    outputs["wasm"] = wasm.execute(data)
                    for out in outputs.values():
                        response(out, len(ib) if op <= 2 else None)
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
                inventory = []
                for path in sorted((target / "upstream-fixtures").glob("*.input")):
                    data = path.read_bytes()
                    _, op, vc, ic, _, _, _ = struct.unpack_from("<4sIIIfII", data)
                    outputs = {n: execute(binaries[n], data) for n in ["cpp", "rust"]}
                    outputs["wasm"] = wasm.execute(data)
                    for out in outputs.values():
                        response(out)
                    match = len(set(outputs.values())) == 1
                    prefix = FAMILIES[op] + "/" + path.stem
                    record["cases"].append({"id": path.stem, "family": FAMILIES[op], "vertices":vc,"indices":ic,
                        "input":retain(archive,prefix+".input",data),"outputs":{n:retain(archive,prefix+"."+n,out) for n,out in outputs.items()},"match":match})
                    record["counts"][FAMILIES[op]] += 1
                    inventory.append({"id":path.stem,"family":FAMILIES[op]})
                    if not match:
                        record["mismatches"] += 1
                        raise ValueError("upstream fixture parity failure: " + path.stem)
                if {x["id"]:x["family"] for x in inventory} != UPSTREAM_FIXTURES:
                    raise ValueError("missing required upstream fixture")
                record["upstream_fixture_inventory"] = inventory
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
    import performance
    performance.run(sys.modules[__name__], args)


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
              "suites": suites, "applicability": "Six applicable simplifier calls are captured by js-fixtures.cjs and compared through Rust/native/WASM. reorderMesh uses Strip (0.1.x); only its topology is reused for standard cache. Other JS operations remain upstream sanity checks."}
    (results / "js.json").write_text(json.dumps(record, indent=2) + "\n")
    print("all five unchanged upstream JS suites passed; direct lane fixture coverage documented")


def verify_record(record, results):
    if record.get("passed") is not True:
        raise ValueError("missing passing record")
    if record.get("command") in {"run", "sweep", "benchmark"} and set(record.get("artifacts", {})) != {record["command"] + "-buffers.zip"}:
        raise ValueError("missing required buffer archive")
    for name, expected in record.get("artifacts", {}).items():
        path = artifacts_path() / name
        if path.parent != artifacts_path() or sha(path) != expected:
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
                    response(out, case["indices"] if case["family"] in {"vertex_cache", "overdraw"} else None)
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
                if record.get("schema") == 2:
                    if len(work["outputs"]) != 2 * len(work["attempts"]) or any(len(s["raw_seconds"]) < 10 for s in work["stats"].values()):
                        raise ValueError("missing benchmark samples")
                    outputs = []
                    for out in work["outputs"]:
                        data = archive.read(out["member"])
                        outputs.append(response(data[:-8], samples=struct.unpack_from("<I", data, 12)[0])[0])
                    if len(set(outputs)) != 1:
                        raise ValueError("benchmark output mismatch")
                    for attempt in work["attempts"]:
                        for invalid in attempt["invalid_clock_samples"]:
                            if "response" in invalid: verify(invalid["response"])
                elif len(work["outputs"]) != 20 or any(len(s["raw_seconds"]) != 10 for s in work["stats"].values()):
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
    if {x["id"]:x["family"] for x in records["run"]["upstream_fixture_inventory"]} != UPSTREAM_FIXTURES:
        raise ValueError("missing upstream fixture inventory")
    for name in ["run", "sweep"]:
        counts = records[name]["counts"]
        actual = {f: sum(c["family"] == f for c in records[name]["cases"]) for f in FAMILIES.values()}
        if counts != actual or any(n < (args.cases_per_family if name == "sweep" else 17) for n in counts.values()):
            raise ValueError("missing required cases")
        for family in FAMILIES.values():
            cases = [c for c in records[name]["cases"] if c["family"] == family]
            if len({c["id"] for c in cases}) != len(cases) or max(c["indices"] for c in cases) < 1000000:
                raise ValueError("missing distinct cases or large mesh coverage")
            if name == "run" and {c["id"] for c in cases} != ({case_id for case_id, _, _, _ in fixtures()} | {f["id"] for f in records[name]["upstream_fixture_inventory"] if f["family"] == family}):
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
    summary = {"schema": 1, "scope": "lane 2 geometry; Linux x86-64 and wasm32 qualification", "passed": True,
               "counts": {n: records[n]["counts"] for n in ["run", "sweep"]}, "mismatches": 0,
               "record_sha256": {n + ".json": sha(results / (n + ".json")) for n in records},
               "targets": ["linux-x86_64", "wasm32 (native Rust identity)"], "benchmark": records["benchmark"]["workloads"]}
    (ROOT / "parity/MEASURED_RESULTS.json").write_text(json.dumps(summary, indent=2) + "\n")
    lines = ["# Measured geometry parity", "", "Exact output comparison with scalar-strict C++ 1.3 and executed wasm32 Rust.", "",
             "| Function | Corpus | Seeded sweep | Mismatches |", "|---|---:|---:|---:|"]
    for f in FAMILIES.values():
        lines.append(f'| {f} | {records["run"]["counts"][f]} | {records["sweep"]["counts"][f]} | 0 |')
    lines += ["", f'Seed: {records["sweep"]["seed"]}. All five functions include 1,000,002-index cases.', "",
              f'Square-root probe: {records["run"]["math_probe"]["count"]} exact f32 results, including signed zero and subnormals.', "",
              "All five upstream JS suites passed unchanged; their applicable-input inventory is in COVERAGE.md.", "",
              "Stable fuzz smoke ran 600 seconds per module. See results/fuzz.json for executions and the instrumentation limits.", "",
              "These are Linux x86-64 and wasm32 lane records. AArch64 and the release fuzz/performance gates remain outside this lane.", "",
              "The records and external compressed input/output buffers retain SHA-256 identities. report.sh rejects stale source and missing or changed artifacts."]
    (ROOT / "parity/MEASURED_PARITY.md").write_text("\n".join(lines) + "\n")
    if records["benchmark"].get("schema") in {2, 3, 4}:
        import performance
        performance.write_report(sys.modules[__name__], records["benchmark"], results)
        print("verified source identities, required counts and retained artifacts; generated measured reports")
        return
    lines = ["# Measured geometry performance", "", "Single thread, 1,000,000 triangles; medians of ten interleaved pairs after warm-up.", "",
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
    if args.cases_per_family < 2000:
        parser.error("at least 2000 cases per family are required")
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
    if "--phase" in sys.argv and sys.argv[sys.argv.index("--phase") + 1] == "0.3":
        if sys.argv[1] == "sweep" and "--cases-per-family" in sys.argv:
            count = int(sys.argv[sys.argv.index("--cases-per-family") + 1])
            if count < 2000:
                raise SystemExit("at least 2000 cases per family are required")
        if sys.argv[1] == "benchmark":
            raise SystemExit(subprocess.call([sys.executable, str(ROOT / "parity/p03/benchmark.py"), *sys.argv[2:]], env=ENV))
        raise SystemExit(subprocess.call([sys.executable, str(ROOT / "parity/p03/runner.py"), *sys.argv[1:]], env=ENV))
    main()
