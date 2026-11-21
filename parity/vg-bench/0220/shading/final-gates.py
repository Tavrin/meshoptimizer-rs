from pathlib import Path
import os,json,subprocess,hashlib,sys
E=Path(os.environ["VG_EVIDENCE"]);R=E/"shading";D=R/"final-gates";D.mkdir(exist_ok=True)
assert json.loads((R/"campaign/status.json").read_text())["exit"]==0
checks=[("fmt",["cargo","fmt","--check"]),("clippy",["cargo","clippy","--all-targets","--all-features","--","-D","warnings"]),("release",["cargo","test","--release","--lib","--tests"]),("release-all-features",["cargo","test","--release","--all-features","--lib","--tests"]),("doc",["env","RUSTDOCFLAGS=-D warnings","cargo","doc","--no-deps","--all-features"]),("public-text",["bash","parity/tools/check_public_text.sh"]),("legacy-identity",["cargo","run","--release","--locked","--offline","--manifest-path",str(E/"legacy-identity/Cargo.toml")]),("harness-tests",["cargo","test","--release","--locked","--manifest-path","parity/vg-bench/Cargo.toml","shading::tests"])]
if "--without-doctests" not in sys.argv:
    checks.append(("all-features-doctests",["cargo","test","--release","--all-features","--doc"]))
results=[]
for name,cmd in checks:
    argv=cmd if name in ["fmt","public-text"] else [os.environ["VG_HEAVY_WRAPPER"],("9" if name == "all-features-doctests" else "4"),*cmd]
    env=dict(os.environ);env.pop("MOSS_HEAVY_PROOF_SECONDS",None)
    with (D/(name+".log")).open("w") as f:rc=subprocess.run(argv,env=env,stdout=f,stderr=subprocess.STDOUT).returncode
    results.append(dict(name=name,exit=rc,command=cmd));(D/"exits.json").write_text(json.dumps(results,indent=2)+"\n");print(name,rc,flush=True)
(D/"source.json").write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for root in ["src","tests","docs","parity/vg-bench/src"] for p in Path(root).rglob("*") if p.is_file()},indent=2)+"\n")
raise SystemExit(int(any(r["exit"] for r in results)))
