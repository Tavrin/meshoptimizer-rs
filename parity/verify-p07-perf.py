#!/usr/bin/env python3
"""Independently verify the performance refresh without crediting release gates."""
import hashlib
import json
import math
import os
import statistics
import subprocess
import zipfile
from pathlib import Path

root = Path(__file__).resolve().parents[1]
art = Path(os.environ["MESHOPT_ARTIFACTS"])
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
read = lambda name: json.loads((art / (name + ".json")).read_text())


def identity(record):
    for name, digest in record["sources"].items():
        assert sha(root / name) == digest, name
    for name, digest in record["binaries"].items():
        assert sha(art / "bin" / name) == digest, name


def interval(values):
    n = len(values)
    t = [2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201,
         2.179, 2.160, 2.145, 2.131, 2.120, 2.110, 2.101, 2.093][n - 5] if n <= 20 else 2.045
    logs = list(map(math.log, values))
    center = statistics.mean(logs)
    radius = t * statistics.stdev(logs) / math.sqrt(n)
    return [math.exp(center - radius), math.exp(center + radius)]


assert subprocess.check_output(["git", "-C", os.environ["MESHOPT_REFERENCE"], "rev-parse", "HEAD"], text=True).strip() == "4c203430ca565cb59a468a91922c76c208169536"
assert sha(root / "parity/SIMD_BAR.md") == "b5730ff38c5b03ed0870004842a775fd286cc3545d4d354b3bd697f3074edca6"
for name in ["parity/codec/reference.cpp", "parity/codec/measure.py", "parity/codec/runner.py", "parity/codec/runner04.py"]:
    assert (root / name).read_bytes() == subprocess.check_output(["git", "show", "179ba52:" + name], cwd=root)
identity(read("build"))
inputs = read("inputs")
assert len(inputs) == 138 and len({r["case"] for r in inputs}) == 138
for row in inputs:
    assert sha(art / row["path"]) == row["sha256"]
fuzz = read("fuzz-smoke-final")
assert fuzz["exit_code"] == 0 and fuzz["wall_seconds"] >= 300
assert fuzz["cpu_seconds"] > 0 and all(x > 0 for x in fuzz["counts"][:4])
assert sha(art / "fuzz-simd") == fuzz["binary_sha256"]
assert sha(art / "fuzz-final.log") == fuzz["log_sha256"]
for name, digest in fuzz["sources"].items():
    assert sha(root / name) == digest
assert not list((art / "fuzz-crashes").iterdir())
for name, count in [("fixtures", 287), ("fixtures04", 582), ("malformed", 3637), ("malformed04", 4016), ("benchmark-identity", 138), ("sweep02", 7000), ("sweep04", 15000)]:
    record = read(name)
    identity(record)
    assert record["mismatches"] == 0 and len(record["cases"]) == count, name
    archive = art / (name + ".zip")
    assert sha(archive) == record["archive_sha256"]
    with zipfile.ZipFile(archive) as z:
        assert len(z.namelist()) == count * 2
        for case in record["cases"]:
            for suffix, key in [("input", "input_sha256"), ("output", "output_sha256")]:
                data = z.read(case["case"] + "." + suffix)
                if case[key] is not None:
                    assert hashlib.sha256(data).hexdigest() == case[key]
    print(name, count, "verified")

conformance = read("upstream-conformance")
assert conformance["exit_code"] == 0 and len(conformance["cases"]) == 87
assert conformance["script_sha256"] == sha(art / "upstream-conformance.mjs")
assert conformance["upstream_sha256"] == sha(Path(os.environ["MESHOPT_REFERENCE"]) / "js/meshopt_decoder.mjs")
assert conformance["benchmark_identity_sha256"] == sha(art / "benchmark-identity.json")
assert conformance["archive_sha256"] == sha(art / "upstream-conformance.zip")
gold = {r["case"]: r for r in read("benchmark-identity")["cases"]}
with zipfile.ZipFile(art / "upstream-conformance.zip") as z:
    for row in conformance["cases"]:
        assert row["tail_preserved"] and row["max_unit_difference"] <= 1
        assert row["expected_sha256"] == gold[row["case"]]["output_sha256"]
        assert row["input_sha256"] == gold[row["case"]]["input_sha256"]
        assert hashlib.sha256(z.read(row["case"] + ".output")).hexdigest() == row["actual_sha256"]
print("shipped JS conformance", len(conformance["cases"]), "verified")

for label, expected in [("performance", {r["case"] for r in inputs}), ("wasm-performance", {r["case"] for r in inputs if int.from_bytes((art / r["path"]).read_bytes()[4:8], "little") in [1, 2, 3, 7]})]:
    record = read(label)
    identity(record)
    assert record["complete"]
    keys = [(r["api"], r["case"]) for r in record["rows"]]
    assert len(keys) == len(set(keys)) == len(expected) * 2
    assert set(keys) == {(api, name) for api in ["allocating", "caller-buffer"] for name in expected}
    for row in record["rows"]:
        n = len(row["raw_seconds"]["rust"])
        assert 5 <= n <= 20
        baseline = "simd" if label == "performance" else "cpp"
        ci = interval([x / y for x, y in zip(row["raw_seconds"]["rust"], row["raw_seconds"][baseline])])
        assert row.get("stage1_interval", row["interval"]) == ci
        if "stage2" in row:
            limit = row.get("worst_limit", 1.60)
            assert n == 20 and ci[0] <= limit < ci[1]
        for stage in [row] + ([row["stage2"]] if "stage2" in row else []):
            expected_count = n if stage is row else 30
            assert len(stage["telemetry"]) == expected_count
            assert all(len(v) == expected_count and all(math.isfinite(x) and x > 0 for x in v) for v in stage["raw_seconds"].values())
            assert all(t["before"]["admitted"] and t["after"]["admitted"] for t in stage["telemetry"])
        ratio = statistics.median(row["raw_seconds"]["rust"]) / statistics.median(row["raw_seconds"][baseline])
        assert row["time_ratio"] == ratio
    for segment in record["controller_segments"]:
        assert sha(root / segment["source"]) == segment["sha256"]
        if "original_source" in segment:
            assert sha(root / segment["original_source"]) == segment["original_sha256"]
        filename = segment.get("executed_controller_path", "native-executed-controller.py" if label == "performance" else "wasm-executed-controller.py")
        assert sha(art / filename) == segment["executed_controller_sha256"]
    for burst in record.get("lease_bursts", []):
        assert burst["wall_seconds"] < 840 and burst["holder"]["admitted"]
        assert burst["holder"]["lease_holder"].startswith("meshopt-timing:p07|")
    print(label, len(keys), "unique rows and admitted samples verified")

assessment = read("s4-assessment")
identity(assessment)
assert assessment["complete"] and assessment["native_sha256"] == sha(art / "performance.json")
assert assessment["controller_sha256"] == sha(root / "parity/measure-simd-s4.py")
native = read("performance")
matrix = {(r["api"], r["case"]): r for r in native["rows"]}
expected_s4 = {key for key, row in matrix.items() if row["family"] in ["index", "sequence"]}
assert {(row["api"], row["case"]) for row in assessment["rows"]} == expected_s4
assert len(assessment["rows"]) == len(expected_s4)
for row in assessment["rows"]:
    original = matrix[(row["api"], row["case"])]
    raw = {k: original["raw_seconds"][k] + row["additional_raw_seconds"][k]
           for k in ["rust", "cpp-scalar"]}
    original_ci = interval([x / y for x, y in zip(original["raw_seconds"]["rust"], original["raw_seconds"]["cpp-scalar"])])
    assert row["original_interval"] == original_ci
    if row["additional_raw_seconds"]["rust"]:
        assert original_ci[0] <= 1.5 < original_ci[1]
    assert len(raw["rust"]) <= 20 and len(raw["rust"]) == len(raw["cpp-scalar"])
    ci = interval([x / y for x, y in zip(raw["rust"], raw["cpp-scalar"])])
    if "stage2" in row:
        assert len(raw["rust"]) == 20 and ci[0] <= 1.5 < ci[1]
        stage = row["stage2"]["raw_seconds"]
        assert all(len(v) == 30 for v in stage.values())
        ci = interval([x / y for x, y in zip(stage["rust"], stage["cpp-scalar"])])
    assert row["interval"] == ci
    assert all(t["before"]["admitted"] and t["after"]["admitted"] for t in row["telemetry"])
for label in ["s4", "profile-native", "profile-wasm", "profile-s4"]:
    context = read(label + "-lease-context")
    assert context["wall_seconds"] < 840
    for gate in [context["before"], context["after"]]:
        assert gate["admitted"] and gate["policy"] == "owner-queued-lease-2026-10-05"
        assert gate["lease_holder"].startswith("meshopt-timing:p07|")
    assert context["admission_sha256"] == sha(root / "parity/p07_lease.py")
    wrapper = root / ("parity/measure-p07-s4-leased.py" if label == "s4" else "parity/profile-p07-leased.py")
    archived_wrapper = art / "profile-p07-leased-pre-s4.py"
    assert context["wrapper_sha256"] == sha(wrapper) or (label in ["profile-native", "profile-wasm"] and context["wrapper_sha256"] == sha(archived_wrapper))
for folder, expected_captures in [("profiles-final", 44), ("profiles-s4-final", 8), ("profiles-wasm-isolated-final", 6)]:
    record = json.loads((art / folder / "identity.json").read_text())
    identity(record)
    assert len(record["cases"]) == expected_captures and all(row["exit_code"] == 0 for row in record["cases"])
    if folder == "profiles-s4-final":
        assert {row["case"] for row in record["cases"]} == {"index-2-v1-streaming-s2", "index-2-v1-streaming-s4"}
print("queued contexts and residual profiles verified")
for backend in ["native", "wasm"]:
    directory = art / ("exhaustive-" + backend)
    complete = json.loads((directory / "complete.json").read_text())
    assert complete["complete"]
    binary = "meshopt-simd-qualification-native" if backend == "native" else "meshopt_simd_qualification.wasm-wasm"
    assert sha(art / "bin" / binary) == complete["identity"]["binary_sha256"]
    for name, digest in complete["identity"]["sources"].items():
        assert sha(root / name) == digest
    for family, total in complete["records"].items():
        end = 0
        for row in sorted([json.loads(p.read_text()) for p in directory.glob(family + "-*.json")], key=lambda r: r["start"]):
            assert row["start"] == end and row["exit_code"] == 0 and row["identity"] == complete["identity"]
            end = row["end"]
        assert end == total
    print(backend, "exhaustive ranges verified")
print("Evidence verified; failed bars and unrun release/platform gates remain explicit.")
