"""One byte-preserving speed milestone; run inside the admitted CPU scope.

VG_EVIDENCE is the original campaign directory. Keep its candidate-vg-bench,
final artifacts and rows. VG_CONTINUATION is a fresh directory containing the
new candidate-vg-bench. Quality rows are reused only after complete file identity.
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess

old = Path(os.environ["VG_EVIDENCE"])
root = Path(os.environ["VG_CONTINUATION"])
out = root / "final"
out.mkdir(exist_ok=False)
candidate = root / "candidate-vg-bench"
previous = old / "candidate-vg-bench"
cores = sorted(os.sched_getaffinity(0))
assert len(cores) >= 8
meshes = ["bunny", "dead_tree_trunk_02", "namaqualand_boulder_02", "coast_rocks_01", "dragon"]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(name, command):
    with (out / (name + ".log")).open("w") as log:
        rc = subprocess.run(list(map(str, command)), stdout=log, stderr=subprocess.STDOUT).returncode
    with (out / "exits.jsonl").open("a") as log:
        log.write(json.dumps(dict(name=name, exit=rc, command=list(map(str, command)))) + "\n")
    assert rc == 0, (name, rc)
    return [json.loads(line) for line in (out / (name + ".log")).read_text().splitlines() if line.startswith("{")]


(out / "environment.json").write_text(json.dumps(dict(
    cores=cores, repeats=5,
    binaries={"candidate": sha(candidate), "previous": sha(previous)},
    reference="8cc060f01c3d4d5920781124cc8c32013bb47730",
), indent=2) + "\n")
for mesh in meshes:
    rows = []
    for repeat in range(5):
        for threads, cpus in [(1, str(cores[0])), (8, ",".join(map(str, cores[:8])))]:
            arms = [("ours", candidate, "ours-v2"), ("baseline", previous, "ours-v2"), ("bevy", candidate, "bevy")]
            shift = repeat % 3
            arms = arms[shift:] + arms[:shift]
            if repeat % 2:
                arms.reverse()
            for arm, binary, policy in arms:
                artifact = out / f"{mesh}.{arm}.{threads}T.bin"
                command = ["taskset", "-c", cpus, "/usr/bin/time", "-v", binary, "build", policy, old / (mesh + ".vgm"), "--threads", threads]
                if repeat == 0:
                    command += ["--out", artifact]
                for row in run(f"build-{mesh}-{arm}-{threads}T-{repeat + 1}", command):
                    row.update(builder=arm, repeat=repeat + 1, rss_kib=row["vmhwm_kib"])
                    if repeat == 0:
                        row["artifact_sha256"] = sha(artifact)
                    rows.append(row)
    for threads in [1, 8]:
        expected = sha(old / "final" / f"{mesh}.ours.{threads}T.bin")
        for arm in ["ours", "baseline"]:
            assert sha(out / f"{mesh}.{arm}.{threads}T.bin") == expected, (mesh, threads, arm, "byte identity")
    # Same complete MDAG 2 files: cuts, error claims, shape and sizes are exact.
    original = [json.loads(line) for line in (old / "final" / (mesh + ".jsonl")).read_text().splitlines()]
    rows.extend(row for row in original if row["kind"] != "build")
    (out / (mesh + ".jsonl")).write_text("".join(json.dumps(row) + "\n" for row in rows))
    for row in run("limits-" + mesh, ["taskset", "-c", ",".join(map(str, cores[:8])), candidate, "limits", "ours", old / (mesh + ".vgm"), "--suggested", "--jobs", "8"]):
        assert row["pass"], (mesh, row)
    print("complete", mesh, flush=True)
