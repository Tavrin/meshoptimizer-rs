"""One core shading campaign, followed by a deterministic UV/tangent matrix."""
from pathlib import Path
import os,sys,json,hashlib,subprocess,statistics,itertools,datetime
E=Path(os.environ["VG_EVIDENCE"]); R=E/"shading"; B=R/"vg-bench"
names=["bunny","dead_tree_trunk_02","namaqualand_boulder_02","coast_rocks_01","dragon"]
uv_names=names[1:4]
weights=[0.0,0.25,0.5,1.0]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,v):p.write_text(json.dumps(v,indent=2)+"\n")
def run(folder,name,cmd):
    with (folder/(name+".log")).open("w") as f:
        argv=list(map(str,cmd))
        if "--compile" in sys.argv and name not in ["format","format-harness"]:argv=[os.environ["VG_HEAVY_WRAPPER"],"4",*argv]
        env=dict(os.environ)
        if "--compile" in sys.argv and name not in ["format","format-harness"]:env.pop("MOSS_HEAVY_PROOF_SECONDS",None)
        else:env.pop("MOSS_HEAVY_PROOF_SECONDS",None)
        rc=subprocess.run(argv,env=env,stdout=f,stderr=subprocess.STDOUT).returncode
    with (folder/"exits.jsonl").open("a") as f:f.write(json.dumps(dict(name=name,exit=rc,command=list(map(str,cmd))))+"\n")
    print(name,rc,flush=True)
    if rc:raise RuntimeError((name,rc))
    return [json.loads(l) for l in (folder/(name+".log")).read_text().splitlines() if l.startswith("{")]
def tag(w):return str(w).replace(".","p")
def flags(w,u=None,t=None,dup=False):
    a=["--normal-error-weight",str(w)]
    if u is not None:a+=["--uv-weight",str(u)]
    if t is not None:a+=["--tangent-weight",str(t)]
    if dup:a+=["--duplicate-normals","1"]
    return a
if "--compile" in sys.argv:
    R.mkdir(exist_ok=True)
    run(R,"format",["cargo","fmt"])
    run(R,"format-harness",["cargo","fmt","--manifest-path","parity/vg-bench/Cargo.toml"])
    run(R,"harness-tests",["cargo","test","--release","--locked","--manifest-path","parity/vg-bench/Cargo.toml","shading::tests"])
    run(R,"build",["cargo","build","--release","--locked","--manifest-path","parity/vg-bench/Cargo.toml"])
    import shutil
    shutil.copy2(Path(os.environ["CARGO_TARGET_DIR"])/"release/vg-bench",B)
    snapshot={str(p):sha(p) for root in ["src","tests","parity/vg-bench/src"] for p in Path(root).rglob("*") if p.is_file()}
    snapshot.update({str(p):sha(p) for p in map(Path,["Cargo.toml","Cargo.lock","parity/vg-bench/Cargo.toml","parity/vg-bench/Cargo.lock"]) if p.is_file()})
    write(R/"compiled-source.json",snapshot)
    (R/"toolchain.txt").write_text(subprocess.check_output(["rustc","-Vv"],text=True))
    write(R/"compile-environment.json",{k:os.environ.get(k) for k in ["RUSTFLAGS","CARGO_ENCODED_RUSTFLAGS","CARGO_BUILD_TARGET","CARGO_BUILD_JOBS","RUSTUP_TOOLCHAIN","CC","CXX"]})
    write(R/"compiled-binary.json",dict(sha256=sha(B)))
    raise SystemExit(0)

assert sha(B)==json.loads((R/"compiled-binary.json").read_text())["sha256"]
for p,h in json.loads((R/"compiled-source.json").read_text()).items():assert sha(Path(p))==h,p
verified=json.loads((E/"continuation2/verified-inputs.json").read_text())
for name,h in verified["vgm_inputs"].items():assert sha(E/name)==h,name
cores=sorted(os.sched_getaffinity(0));assert len(cores)>=8
cpus={1:str(cores[0]),8:",".join(map(str,cores[:8]))}
out=R/"campaign";out.mkdir(exist_ok=False)
write(out/"protocol.json",dict(started_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),driver_sha256=sha(Path(__file__)),configuration="default_config(128), grid -22, dedicated simplifier normal weight 0.5",repeats=5,cores=cores,pinned_cpus=cpus,weights=weights,dirs=16,spans=[64,128,256,512,1024,2048],threshold_px=1,samples_each_direction=16384,
    sampling="Fixed area-stratified source probes reused for all variants; symmetric target probes; exact float32 pooled quantiles weighted by view multiplicity.",
    default_criterion="Smallest tested weight with dragon p95 <= 1.10*weight1 p95 and mean triangles <= 2*weight0 mean triangles. No relaxation if none qualifies.",
    attribute_study="Existing attributes only; normal via dedicated input (fixed simplifier weight 0.5). UV/tangent 3x3 grid plus duplicated-normal controls. One 8T build per matrix cell, no performance claim from those single observations.",
    tangent_provenance="Authored UVs on three scan meshes; tangents derived from area-weighted UV gradients with Gram-Schmidt and handedness. Not authored or Mikk-compatible qualification.",
    binary_sha256=sha(B),source_hashes=json.loads((R/"compiled-source.json").read_text()),input_hashes=verified["vgm_inputs"]))
core=[]
for mesh in names:
    build_rows=[]
    for repeat in range(5):
        arms=weights[repeat%4:]+weights[:repeat%4]
        if repeat%2:arms=arms[::-1]
        for threads in [1,8]:
            for w in arms:
                artifact=out/f"{mesh}.normal-{tag(w)}.{threads}T.bin"
                cmd=["taskset","-c",cpus[threads],"/usr/bin/time","-v",B,"build","ours-v2",E/(mesh+".vgm"),"--threads",threads,*flags(w)]
                if repeat==0:cmd+=["--out",artifact]
                rs=run(out,f"build-{mesh}-w{tag(w)}-{threads}T-{repeat+1}",cmd)
                row=next(x for x in rs if x["kind"]=="build")
                assert row["normal_error_weight"]==w and not row["duplicate_normals"]
                row.update(repeat=repeat+1,weight=w)
                if repeat==0:row["artifact_sha256"]=sha(artifact)
                build_rows.append(row)
    for w in weights:
        assert sha(out/f"{mesh}.normal-{tag(w)}.1T.bin")==sha(out/f"{mesh}.normal-{tag(w)}.8T.bin"),(mesh,w,"worker identity")
    for threads in [1,8]:
        assert sha(out/f"{mesh}.normal-0p0.{threads}T.bin")==sha(E/"final"/f"{mesh}.ours.{threads}T.bin"),(mesh,"zero disables exactly")
    eval_rows=run(out,"eval-"+mesh,["taskset","-c",cpus[8],B,"shade-eval",E/(mesh+".vgm"),*[out/f"{mesh}.normal-{tag(w)}.8T.bin" for w in weights],"--samples",16384,"--dirs",16,"--jobs",8,"--with-tangents",0])
    assert len(eval_rows)==4 and all(x["cut_oracle_pass"] for x in eval_rows)
    for w,quality in zip(weights,eval_rows):
        times={}
        for threads in [1,8]:
            rows=[x for x in build_rows if x["weight"]==w and x["threads"]==threads]
            v=[x["build_ms"]+x["serialise_ms"] for x in rows]
            times[str(threads)]=dict(median=statistics.median(v),p95=max(v),noise=max(v)-min(v),
                accounted_bytes=max(x["accounted_bytes"] for x in rows),work=max(x["work"] for x in rows))
        core.append(dict(mesh=mesh,weight=w,quality=quality,timing=times))
    write(out/(mesh+"-builds.json"),build_rows);write(out/"core.json",core)
dragon={x["weight"]:x for x in core if x["mesh"]=="dragon"}
target=dragon[1.0]["quality"]["normal_p95_deg"]*1.10
budget=dragon[0.0]["quality"]["mean_triangles"]*2.0
eligible=[w for w in weights if dragon[w]["quality"]["normal_p95_deg"]<=target and dragon[w]["quality"]["mean_triangles"]<=budget]
chosen=min(eligible) if eligible else 0.0
selection=dict(eligible=eligible,chosen_default=chosen,qualified=bool(eligible),dragon_normal_p95_limit=target,dragon_triangle_limit=budget,
    reason="Smallest weight satisfying both requested conditions." if eligible else "No tested weight satisfies both requested conditions; disabled default retained.")
write(out/"default-selection.json",selection)
# When no default fits, the attribute comparison is explicitly an opt-in quality mode.
attribute_weight=chosen if eligible else 1.0
all_attributes=[]
for mesh in uv_names:
    cases=[("consumer-current",0.0,1.0,1.0,True),("duplicate-control",attribute_weight,1.0,1.0,True)]
    cases += [(f"u{tag(u)}-t{tag(t)}",attribute_weight,u,t,False) for u,t in itertools.product([.25,.5,1.0],repeat=2)]
    artifacts=[]
    for label,w,u,t,dup in cases:
        artifact=out/f"{mesh}.{label}.8T.bin";artifacts.append(artifact)
        run(out,f"attr-build-{mesh}-{label}",["taskset","-c",cpus[8],B,"build","ours-v2",E/(mesh+".vgm"),"--threads",8,*flags(w,u,t,dup),"--out",artifact])
    measured=run(out,"attr-eval-"+mesh,["taskset","-c",cpus[8],B,"shade-eval",E/(mesh+".vgm"),*artifacts,"--samples",16384,"--dirs",16,"--jobs",8,"--with-tangents",1])
    assert len(measured)==len(cases)
    for case,quality,artifact in zip(cases,measured,artifacts):
        label,w,u,t,dup=case
        all_attributes.append(dict(mesh=mesh,label=label,normal_error_weight=w,uv_weight=u,tangent_weight=t,duplicate_normals=dup,quality=quality,artifact_sha256=sha(artifact)))
    write(out/"attributes.json",all_attributes)
# One shared weight set: preserve each mesh's normal, UV and tangent p95 within
# ten percent of correctly packed 1/1, minimizing geometric-mean triangle ratio.
scores=[]
for u,t in itertools.product([.25,.5,1.0],repeat=2):
    ratios=[];good=True
    for mesh in uv_names:
        ref=next(x for x in all_attributes if x["mesh"]==mesh and x["label"]=="u1p0-t1p0")
        row=next(x for x in all_attributes if x["mesh"]==mesh and not x["duplicate_normals"] and x["uv_weight"]==u and x["tangent_weight"]==t)
        for metric in ["normal_p95_deg","uv_p95","tangent_p95_deg","tangent_sign_mismatch_fraction"]:
            good &= row["quality"][metric]<=ref["quality"][metric]*1.10
        ratios.append(row["quality"]["mean_triangles"]/ref["quality"]["mean_triangles"])
    if good:scores.append((statistics.geometric_mean(ratios),-u,-t,u,t))
assert scores,"correctly packed 1/1 must remain a valid recommendation"
_,_,_,u,t=min(scores)
write(out/"attribute-selection.json",dict(uv_weight=u,tangent_weight=t,normal_error_weight=attribute_weight,
    simplifier_normal_weight=0.5,duplicate_normals=False,
    criterion="All three meshes: normal, UV, tangent p95 and tangent handedness mismatch fraction <= 1.10 of correctly packed 1/1. Minimize geometric mean triangle ratio; ties prefer larger weights.",
    tangent_provenance="Derived from authored UVs; no authored tangent qualification."))
for mesh in uv_names:
    label=f"u{tag(u)}-t{tag(t)}";artifact=out/f"{mesh}.{label}.1T.bin"
    run(out,"selected-worker-identity-"+mesh,["taskset","-c",cpus[1],B,"build","ours-v2",E/(mesh+".vgm"),"--threads",1,*flags(attribute_weight,u,t),"--out",artifact])
    assert sha(artifact)==sha(out/f"{mesh}.{label}.8T.bin")
write(out/"status.json",dict(exit=0,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),core_complete=True,attribute_complete=True,default_selection=selection))
