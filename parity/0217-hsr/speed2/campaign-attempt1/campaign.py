"""Same-host interleaved resident timings, pinned reference and exact indices."""
from support import *
import platform,statistics,sys
# Preserve oracle build receipts before the timing arms reuse these labels.
oracle_logs=RECEIPT/'oracle-builds';oracle_logs.mkdir(exist_ok=True)
for name in ['build-before.log','build-prior.log','build-after.log','build-none.log','build-parallel.log','oracle-build.log']:
    path=RECEIPT/name
    if path.exists():
        assert not (oracle_logs/name).exists()
        shutil.move(path,oracle_logs/name)
quick='--quick' in sys.argv
arms=[('before',reference(),['std','simd']),('serial',ROOT,['std','simd'])]
if not quick:arms += [('before-parallel',reference(),['std','simd','parallel']),('parallel',ROOT,['std','simd','parallel']),('none',ROOT,[])]
binaries={arm:build(arm,source,features) for arm,source,features in arms}
cpus=sorted(os.sched_getaffinity(0));serial_cpu=cpus[0];pool_cpus=cpus[:4]
rows=[];identity=[]
for scene in ['breakfast_room','conference','living_room']:
    mesh=SCENES/(scene+'.bin')
    meta=next(v for v in json.loads((ROOT/'parity/0217-hsr/eval/scenes.json').read_text()) if v['name']==scene)
    assert sha(mesh)==meta['mesh_sha256']
    peers={};expected=None
    for arm,binary in binaries.items():
        out=SCRATCH/(scene+'-'+arm+'.out')
        affinity=','.join(map(str,pool_cpus)) if 'parallel' in arm else str(serial_cpu)
        p=subprocess.Popen(['taskset','-c',affinity,str(binary),str(mesh),str(out)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,env=dict(os.environ,RAYON_NUM_THREADS='4',HSR_PIXEL_SIZE='0'))
        line=p.stdout.readline()
        if not line:raise RuntimeError((scene,arm,p.wait(),p.stderr.read()))
        data=json.loads(line);print(scene,arm,'warm',data,flush=True)
        digest=sha(out)
        if expected is None:expected=digest
        assert digest==expected,(scene,arm,digest,expected)
        peers[arm]=(p,out)
        identity.append(dict(scene=scene,arm=arm,phase='warm',sha256=digest,report=data))
    for rep in range(1 if quick else 3):
        order=list(peers)
        if rep==1:order.reverse()
        for arm in order:
            p,out=peers[arm];p.stdin.write('measured\n');p.stdin.flush()
            line=p.stdout.readline()
            if not line:raise RuntimeError((scene,arm,p.wait(),p.stderr.read()))
            data=json.loads(line);digest=sha(out);assert digest==expected
            rows.append(dict(scene=scene,arm=arm,rep=rep,output_sha256=digest,**data))
            print(scene,arm,rep,data['seconds'],flush=True)
            (RECEIPT/('quick-samples.json' if quick else 'samples.json')).write_text(json.dumps(rows,indent=2)+'\n')
    for arm,(p,out) in peers.items():
        stdout,stderr=p.communicate('stop\n');(RECEIPT/(scene+'-'+arm+'.stderr')).write_text(stderr)
        with (RECEIPT/'exits.tsv').open('a') as f:f.write(f'{scene}-{arm}\t{p.returncode}\n')
        assert p.returncode==0,(scene,arm,stderr)
summary=[dict(scene=scene,arm=arm,median=statistics.median(r['seconds'] for r in rows if r['scene']==scene and r['arm']==arm)) for scene in ['breakfast_room','conference','living_room'] for arm in binaries]
(RECEIPT/('quick-summary.json' if quick else 'summary.json')).write_text(json.dumps(summary,indent=2)+'\n')
(RECEIPT/'identity.json').write_text(json.dumps(identity,indent=2)+'\n')
(RECEIPT/'environment.json').write_text(json.dumps(dict(cpu=serial_cpu,pool_cpus=pool_cpus,rayon_threads=4,platform=platform.platform(),rustc=subprocess.check_output(['rustc','-vV'],text=True),binary_sha256={k:sha(v) for k,v in binaries.items()},source_sha256={str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'src').rglob('*.rs')},timing='one warmup, three resident calls; interleaved arms with reversed middle repetition; parsing and serialization excluded; shared-host wall time; overflow checks enabled',reference='f9859cc'),indent=2)+'\n')
print(summary,flush=True)
# Retain the evaluation's maximum-grid/extreme-coordinate overflow check.
run('hidden-overflow',['cargo','test','--locked','--release','--all-features','--test','hidden'],env=dict(os.environ,CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS='true'))
