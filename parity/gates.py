#!/usr/bin/env python3
"""Run foundation gates without changing an unborn/dirty repository's Git state."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

ROOT = Path(__file__).resolve().parent.parent
COMMANDS = [
    ["cargo", "fmt", "--check"],
    ["cargo", "clippy", "--locked", "--all-targets", "--all-features", "--", "-D", "warnings"],
    ["cargo", "test", "--locked", "--all-features"],
    ["cargo", "test", "--locked", "--no-default-features"],
    ["cargo", "build", "--target", "wasm32-unknown-unknown", "--no-default-features"],
    ["cargo", "build", "--locked", "--no-default-features"],
    ["cargo", "+1.88", "test", "--locked", "--all-features"],
    ["cargo", "+1.88", "test", "--locked", "--no-default-features"],
    ["cargo", "package", "--list"],
    ["cargo", "publish", "--dry-run", "--allow-dirty"],
    ["cargo", "doc", "--locked", "--no-deps", "--all-features"],
    ["cargo", "doc", "--locked", "--no-deps", "--no-default-features"],
    ["cargo", "clippy", "--locked", "--manifest-path", "parity/Cargo.toml", "--all-targets", "--", "-D", "warnings"],
    ["cargo", "clippy", "--locked", "--manifest-path", "fuzz/Cargo.toml", "--all-targets", "--", "-D", "warnings"],
    ["cargo", "fmt", "--manifest-path", "parity/Cargo.toml", "--check"],
    ["cargo", "fmt", "--manifest-path", "fuzz/Cargo.toml", "--check"],
]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def snapshot():
    files = [ROOT / "Cargo.toml", ROOT / "Cargo.lock", *ROOT.glob("src/*.rs"), *ROOT.glob("tests/*.rs"),
             ROOT / "parity/gates.py", ROOT / "parity/gates.sh", ROOT / ".github/workflows/ci.yml",
             *[ROOT / name for name in ["README.md", "LICENSE", "UPSTREAM.md", "CHANGELOG.md", "CONTRIBUTING.md", "SECURITY.md"]]]
    return {str(p.relative_to(ROOT)): sha(p) for p in sorted(files)}


def publish_check(env, target):
    """Exercise Cargo's real dry-run against local metadata and locked sources."""
    direct = subprocess.run(["cargo", "publish", "--dry-run", "--allow-dirty"], cwd=ROOT, env=env, capture_output=True, text=True)
    if direct.returncode == 0 or "attempting to make an HTTP request" not in direct.stderr:
        return direct, None
    with tempfile.TemporaryDirectory(prefix="publish-check-", dir=target) as temporary:
        base = Path(temporary)
        vendor = base / "vendor"
        subprocess.run(["cargo", "vendor", "--offline", "--locked", str(vendor)], cwd=ROOT, env=env,
                       check=True, capture_output=True, text=True)
        cache = base / "cargo-home"
        cache.mkdir()
        (cache / "config.toml").write_text('[source.crates-io]\nreplace-with = "locked-vendor"\n[source.locked-vendor]\ndirectory = ' + json.dumps(str(vendor)) + "\n")
        requests = []
        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                requests.append(self.path)
                if self.path == "/config.json":
                    data = json.dumps({"dl": url + "crates", "api": url.rstrip("/")}).encode()
                    self.send_response(200)
                    self.end_headers()
                    self.wfile.write(data)
                else:
                    self.send_response(404)
                    self.end_headers()

            def log_message(self, *args):
                pass
        server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        url = f"http://127.0.0.1:{server.server_port}/"
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        local_env = {**env, "CARGO_HOME": str(cache), "CARGO_NET_OFFLINE": "false", "CARGO_REGISTRY_DEFAULT": "lane1-offline",
                     "CARGO_REGISTRIES_LANE1_OFFLINE_INDEX": "sparse+" + url,
                     "CARGO_REGISTRIES_LANE1_OFFLINE_TOKEN": "dry-run-unused",
                     "CARGO_HTTP_USER_AGENT": "meshoptimizer-rs-rfc", "CARGO_HTTP_PROXY": ""}
        try:
            result = subprocess.run(["cargo", "publish", "--dry-run", "--allow-dirty"], cwd=ROOT, env=local_env, capture_output=True, text=True, timeout=120)
        finally:
            server.shutdown()
            thread.join()
            server.server_close()
        if set(requests) != {"/config.json", "/me/sh/meshoptimizer-rs"}:
            raise ValueError("unexpected local registry requests")
        adjustment = {"decision": "D9: Cargo real dry-run with a loopback sparse registry and offline-vendored locked dependencies",
                      "crates_io_offline_exit": direct.returncode, "crates_io_stderr": direct.stderr,
                      "local_requests": requests, "upload_requests": 0,
                      "qualification": "package/build validation only; no crates.io naming or registry-readiness claim",
                      "vendored_sha256": {str(p.relative_to(vendor)): sha(p) for p in sorted(vendor.rglob("*")) if p.is_file()}}
        return result, adjustment


def main():
    target = Path(os.environ["CARGO_TARGET_DIR"]).resolve()
    if target == ROOT or ROOT in target.parents:
        raise ValueError("use an isolated build target")
    (target / "tmp").mkdir(parents=True, exist_ok=True)
    env = {**os.environ, "CARGO_NET_OFFLINE": "true", "GIT_OPTIONAL_LOCKS": "0", "RUSTC_WRAPPER": "",
           "RUSTC_WORKSPACE_WRAPPER": "", "RUSTFLAGS": "", "CARGO_ENCODED_RUSTFLAGS": "", "TMPDIR": str(target / "tmp")}
    destination = Path(os.environ.get("MESHOPT_RESULTS", ROOT / "parity/results"))
    destination.mkdir(parents=True, exist_ok=True)
    def redact(text):
        return text.replace(str(target), "$CARGO_TARGET_DIR").replace(str(ROOT), ".").replace(str(Path.home()), "$USER_HOME")
    record = {"schema": 1, "started_unix": time.time(), "source_sha256": snapshot(),
              "environment": {k: env[k] for k in ["CARGO_NET_OFFLINE", "GIT_OPTIONAL_LOCKS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTFLAGS"]},
              "commands": [], "adjustments": [], "passed": False}
    for cmd in COMMANDS:
        cmd_env = dict(env)
        if "doc" in cmd:
            cmd_env["RUSTDOCFLAGS"] = "-D warnings"
        cwd = "working tree"
        if cmd == ["cargo", "package", "--list"]:
            # The user prohibits Git changes. Cargo refuses even listing a dirty
            # package; export identical files without Git metadata for this gate.
            direct = subprocess.run(cmd, cwd=ROOT, env=cmd_env, capture_output=True, text=True)
            if direct.returncode:
                record["adjustments"].append({"command": "cargo package --list", "working_tree_exit": direct.returncode,
                    "stderr": redact(direct.stderr), "decision": "D8: exact command in source-identical export; no Git metadata changes"})
                with tempfile.TemporaryDirectory(prefix="package-export-", dir=target) as temporary:
                    export = Path(temporary) / "crate"
                    shutil.copytree(ROOT, export, ignore=shutil.ignore_patterns(".git", ".agents", ".codex", "target", "__pycache__"))
                    if any(sha(export / name) != expected for name, expected in record["source_sha256"].items()):
                        raise ValueError("package export differs from source")
                    result = subprocess.run(cmd, cwd=export, env=cmd_env, capture_output=True, text=True)
                    allowed = subprocess.run([*cmd, "--allow-dirty"], cwd=ROOT, env=cmd_env, capture_output=True, text=True)
                    if allowed.returncode or result.stdout != allowed.stdout:
                        raise ValueError("export and working-tree package inventories differ")
                    for name in result.stdout.splitlines():
                        if name.startswith(("parity/", "fuzz/", ".github/")):
                            raise ValueError("unpublished tooling in package")
                        original = ROOT / name
                        if original.is_file() and sha(original) != sha(export / name):
                            raise ValueError("published file identity differs")
                    cwd = "source-identical temporary export (inventory matched working tree with --allow-dirty)"
            else:
                result = direct
        elif cmd == ["cargo", "publish", "--dry-run", "--allow-dirty"]:
            result, adjustment = publish_check(cmd_env, target)
            if adjustment:
                record["adjustments"].append(adjustment)
                cwd = "working tree; loopback sparse registry and locked vendored dependency sources (D9)"
        else:
            result = subprocess.run(cmd, cwd=ROOT, env=cmd_env, capture_output=True, text=True)
        record["commands"].append({"command": " ".join(cmd), "cwd": cwd, "exit": result.returncode,
                                   "stdout": redact(result.stdout), "stderr": redact(result.stderr)})
        print(" ".join(cmd), "exit", result.returncode, flush=True)
        if result.returncode:
            print(redact(result.stderr), flush=True)
            break
    record["identities_unchanged"] = record["source_sha256"] == snapshot()
    record["passed"] = len(record["commands"]) == len(COMMANDS) and all(c["exit"] == 0 for c in record["commands"]) and record["identities_unchanged"]
    record["finished_unix"] = time.time()
    package = target / "package/meshoptimizer-rs-0.1.0.crate"
    if record["passed"]:
        shutil.copyfile(package, destination / package.name)
        record["package_artifact"] = {"name": package.name, "sha256": sha(package)}
    (destination / "gates.json").write_text(json.dumps(record, indent=2) + "\n")
    if not record["passed"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
