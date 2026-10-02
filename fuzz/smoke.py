#!/usr/bin/env python3
"""Stable bounded mutation smoke, with pre/post source and binary identities."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import signal
import subprocess
import time

ROOT = Path(__file__).resolve().parent.parent
ENV = {**os.environ, "CARGO_NET_OFFLINE": "true", "RUSTFLAGS": "", "CARGO_ENCODED_RUSTFLAGS": "", "RUSTC_WRAPPER": "", "RUSTC_WORKSPACE_WRAPPER": ""}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def snapshot():
    paths = [ROOT / "Cargo.toml", ROOT / "Cargo.lock", ROOT / "fuzz/Cargo.toml", ROOT / "fuzz/Cargo.lock",
             *ROOT.glob("src/*.rs"), *ROOT.glob("fuzz/src/**/*.rs"), ROOT / "fuzz/smoke.py"]
    return {str(p.relative_to(ROOT)): sha(p) for p in sorted(paths)}


def dependency_snapshot():
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--offline", "--locked", "--format-version", "1"], env=ENV, cwd=ROOT, text=True))
    dependencies = {}
    for package in metadata["packages"]:
        if package["source"] is not None:
            directory = Path(package["manifest_path"]).parent
            dependencies.update({f'{package["name"]}-{package["version"]}/{p.relative_to(directory)}': sha(p)
                                 for p in sorted(directory.rglob("*")) if p.is_file()})
    return dependencies


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--seconds", type=int, default=600)
    parser.add_argument("--seed", type=int, default=20261002)
    args = parser.parse_args()
    if args.seconds < 1 or args.seed < 1:
        parser.error("positive elapsed budget and nonzero seed required")
    target = Path(os.environ["CARGO_TARGET_DIR"])
    (target / "tmp").mkdir(parents=True, exist_ok=True)
    ENV["TMPDIR"] = str(target / "tmp")
    before_build = snapshot()
    subprocess.run(["cargo", "build", "--offline", "--locked", "--release", "--manifest-path", str(ROOT / "fuzz/Cargo.toml")], check=True, env=ENV)
    if before_build != snapshot():
        raise ValueError("source changed during fuzz build")
    before = snapshot()
    binaries = {name: target / "release" / name for name in ["vertex_cache", "overdraw"]}
    identities = {n: sha(b) for n, b in binaries.items()}
    dependencies = dependency_snapshot()
    record = {"profile": "stable-seeded-mutation; no sanitizer or coverage instrumentation",
              "source_sha256": before, "dependency_sha256": dependencies, "executable_sha256": identities,
              "rustc": subprocess.check_output(["rustc", "-Vv"], text=True),
              "cargo": subprocess.check_output(["cargo", "-V"], text=True).strip(),
              "hardware": platform.uname()._asdict(), "started_unix": time.time(),
              "budget_seconds": args.seconds, "seed": args.seed, "rustflags": ""}
    processes = {}
    for name, binary in binaries.items():
        usage = target / f"fuzz-{name}-usage.json"
        processes[name] = (subprocess.Popen(["/usr/bin/time", "-o", str(usage), "-f",
                              '{"user_seconds":%U,"system_seconds":%S,"elapsed_seconds":%e,"max_rss_kib":%M}',
                              str(binary), str(args.seconds), str(args.seed)],
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=ENV, start_new_session=True), usage)
    results = []
    for name, (process, usage) in processes.items():
        try:
            stdout, stderr = process.communicate(timeout=args.seconds + 60)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            stdout, stderr = process.communicate()
            stderr += "\nfuzz time budget exceeded\n"
        result = {"target": name, "exit": process.returncode, "stdout": stdout, "stderr": stderr}
        if process.returncode == 0:
            result["statistics"] = json.loads(stdout)
            result["usage"] = json.loads(usage.read_text())
        results.append(result)
    record["runs"] = results
    record["finished_unix"] = time.time()
    record["identities_unchanged"] = before == snapshot() and dependencies == dependency_snapshot() and identities == {n: sha(b) for n, b in binaries.items()}
    destination = Path(os.environ.get("MESHOPT_RESULTS", ROOT / "parity/results"))
    destination.mkdir(parents=True, exist_ok=True)
    (destination / "fuzz.json").write_text(json.dumps(record, indent=2) + "\n")
    if not record["identities_unchanged"] or any(r["exit"] for r in results):
        raise SystemExit("fuzz gate failed; see retained fuzz.json")
    for result in results:
        print(result["stdout"].strip())


if __name__ == "__main__":
    main()
