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
        limit = row.get("worst_limit", 1.60)
        for stop in range(5,n):
            earlier = interval([x/y for x,y in zip(row['raw_seconds']['rust'][:stop],row['raw_seconds'][baseline][:stop])])
            assert earlier[0] <= limit < earlier[1], (label,row['case'],stop,'missed early stop')
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
        assert sha(root / 'parity/p07_lease.py') == segment['admission_sha256']
        if "original_source" in segment:
            assert sha(root / segment["original_source"]) == segment["original_sha256"]
        filename = segment.get("executed_controller_path", "native-executed-controller.py" if label == "performance" else "wasm-executed-controller.py")
        assert sha(art / filename) == segment["executed_controller_sha256"]
    for burst in record.get("lease_bursts", []):
        assert burst["wall_seconds"] < 840 and burst["holder"]["admitted"]
        assert burst["holder"]["lease_holder"].startswith("heavy:timeout|")
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
def gate_identity(gate):
    assert gate['admitted'] and gate['policy'] == 'owner-visible-heavy-4GB-2026-10-05'
    assert gate['lease_holder'].startswith('heavy:timeout|')
    assert gate['reserved_gb'] >= 4 and gate['queue_receipt']
    chain = gate['ancestors']
    assert int(gate['lease_holder'].split('|')[1]) in {r['pid'] for r in chain}
    wrapper = next(r for r in chain if r['pid'] == gate['heavy_pid'])
    assert '/mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 timeout 840 ' in wrapper['command']

for name in ['performance', 'wasm-performance']:
    for row in read(name)['rows']:
        for stage in [row] + ([row['stage2']] if 'stage2' in row else []):
            for pair in stage['telemetry']:
                gate_identity(pair['before']); gate_identity(pair['after'])
context = read('s4-lease-context')
assert context['wall_seconds'] < 840
for gate in [context['before'], context['after']]: gate_identity(gate)
assert context['admission_sha256'] == sha(root / 'parity/p07_lease.py')
assert context['wrapper_sha256'] == sha(root / 'parity/measure-p07-s4-leased.py')
for row in assessment['rows']:
    for pair in row['telemetry']:
        gate_identity(pair['before']); gate_identity(pair['after'])
profile = json.loads((art / 'profiles-final/identity.json').read_text())
identity(profile)
assert len(profile['cases']) == 30 and profile['wall_seconds'] < 840
assert all(row['exit_code'] == 0 for row in profile['cases'])
assert profile['controller_sha256'] == sha(art / 'profile.py')
assert profile['admission_sha256'] == sha(root / 'parity/p07_lease.py')
assert profile['base_binary_sha256'] == sha(Path('/mnt/linux-extra/meshopt-artifacts/p07perf/bin/rust-simd'))
for row in profile['cases']:
    gate_identity(row['before']); gate_identity(row['after'])
    response = art / 'profiles-final' / (row['case']+'-'+row['backend']+'-'+row['mode']+'.response')
    # The framed output contains status, output length and timed metadata.
    import sys
    sys.path.insert(0,str(root/'parity/codec'))
    import measure
    status,data,_ = measure.decode(response.read_bytes()[4:])
    assert status == 0 and hashlib.sha256(data).hexdigest() == row['output_sha256']
    assert row['input_sha256'] == sha(art / 'profiles-final' / (row['case']+'.input'))
for case in {r['case'] for r in profile['cases']}:
    assert len({r['output_sha256'] for r in profile['cases'] if r['case'] == case}) == 1
print('visible queue contexts and residual profiles verified')
for filename, count in [('miri-final.log',5),('miri-integer-final.log',3),('miri-scalar-final.log',8)]:
    assert ('test result: ok. '+str(count)+' passed; 0 failed') in (art/filename).read_text()
gates = read('gates')
for name,digest in gates['sources'].items(): assert sha(root/name) == digest
assert len(gates['commands']) == 11 and all(r['exit_code'] == 0 for r in gates['commands'])
for i,row in enumerate(gates['commands']): assert sha(art/('gate-'+str(i)+'.log')) == row['log_sha256']
checks = read('checks-final'); identity(checks)
for name,digest in checks['logs'].items(): assert sha(art/name) == digest
assert checks['extras_combined_exit_code'] == 0
print('Miri and static gates verified')
print('Round-three evidence verified; historical exhaustive filter records are not refreshed or credited here.')
