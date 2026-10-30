"""Verify retained receipts, run public-text gate, optionally remove owned target.

Run through CPU admission after the campaign and release report are complete.
Sized below 60 CPU seconds. No benchmark or proof stream is repeated.
"""
import hashlib, json, os, resource, shutil, subprocess, sys, time
from pathlib import Path
import run as q

assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU')=='0'
before=resource.getrusage(resource.RUSAGE_CHILDREN)
own_before=resource.getrusage(resource.RUSAGE_SELF)
start=time.monotonic()
with (q.A/'job-sizing.jsonl').open('a') as f:
    f.write(json.dumps(dict(stage='finish',upper_cpu_seconds_estimate=60,basis='hash and text verification only'))+'\n')
code=0
try:
    results=json.loads((q.A/'release-results.json').read_text())
    assert not results['regressions'], results['regressions']
    assert results['campaign_samples']==3504 and results['samples']==3594
    initial=json.loads((q.A/'start.json').read_text())
    current={str(p.relative_to(q.R)):q.sha(p) for p in (q.R/'src').rglob('*') if p.is_file()}
    assert current==initial['sources'], 'production source changed'
    jobs=[json.loads(l) for l in (q.A/'jobs.jsonl').read_text().splitlines()]
    last={x['stage']:x for x in jobs}
    assert all(x['exit']==0 for stage,x in last.items() if stage!='finish'),last
    assert all(x.get('total_cpu_seconds',x['child_cpu_seconds'])<=7200 for x in jobs)
    binaries={}
    peers=json.loads((q.A/'peer-bindings.json').read_text())
    for arm,b in peers.items():
        assert q.sha(q.T/'bin'/arm)==b['binary'],arm
    for mode in ['all','none']:
        binding=json.loads((q.A/f'exact-binding-{mode}.json').read_text())
        for name,digest in binding['binaries'].items():
            assert q.sha(q.T/'bin'/f'{name}-{mode}')==digest,(name,mode)
    for p in (q.T/'bin').iterdir():
        if p.is_file():binaries[str(p.relative_to(q.T))]=q.sha(p)
    binaries['stream-cpp']=q.sha(q.T/'stream-cpp')
    q.write(q.A/'final-binaries.json',binaries)
    for x in json.loads((q.A/'inputs.json').read_text()):
        assert q.sha(q.T/'inputs'/(x['case']+'.bin'))==x['sha256']
    rows=[json.loads(l) for l in (q.A/'counts.jsonl').read_text().splitlines()]
    assert len(rows)==3594
    for row in rows:
        prefix=q.A/row['sample_prefix']
        text=Path(str(prefix)+'.perf').read_text()
        fields=next(l.split(';') for l in text.splitlines() if ';instructions:u;' in l)
        assert int(fields[0])==row['instructions'] and float(fields[4])==100.,row
        assert row['exit']==0 and row['instructions']>0
    manifests={}
    locks=q.A/'locks';locks.mkdir(exist_ok=True)
    for p in q.T.rglob('Cargo.toml'):
        manifests[str(p.relative_to(q.T))]=q.clean(p.read_text())
        lock=p.with_name('Cargo.lock')
        if lock.exists():
            name=str(lock.relative_to(q.T)).replace('/','--')
            shutil.copy2(lock,locks/name)
    q.write(q.A/'final-manifests.json',manifests)
    q.run('final-source-diff',['git','diff','--exit-code','--','src','Cargo.toml','Cargo.lock'],estimate=1)
    q.run('final-reference',['bash','parity/check-reference.sh'],estimate=1)
    q.run('final-public-text',['bash','parity/tools/check_public_text.sh'],estimate=10)
    q.run('final-whitespace',['git','diff','--check'],estimate=1)
    q.write(q.A/'final-verification.json',dict(exit=0,source_unchanged=True,
        campaign_samples=3504,cost_samples=90,wall_samples=30,
        retained_perf_files_verified=len(rows),binaries=binaries,
        max_job_cpu_seconds=max(x.get('total_cpu_seconds',x['child_cpu_seconds']) for x in jobs),
        prior_failed_attempts=[x for x in jobs if x['exit']!=0]))
    if '--cleanup' in sys.argv:
        assert q.T.is_absolute() and q.T.name=='meshopt-qualify-0216' and q.T.is_dir() and not q.T.is_symlink()
        shutil.rmtree(q.T)
        q.write(q.A/'cleanup-final.json',dict(exit=0,target_removed=not q.T.exists(),retained_binaries=len(binaries)))
    print('FINAL VERIFICATION PASS',flush=True)
except BaseException as e:
    code=e.code if isinstance(e,SystemExit) and isinstance(e.code,int) else 1
    raise
finally:
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    own_after=resource.getrusage(resource.RUSAGE_SELF)
    cpu=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime
    own=own_after.ru_utime+own_after.ru_stime-own_before.ru_utime-own_before.ru_stime
    with (q.A/'jobs.jsonl').open('a') as f:
        f.write(json.dumps(dict(stage='finish',exit=code,child_cpu_seconds=cpu,supervisor_cpu_seconds=own,
            total_cpu_seconds=cpu+own,elapsed_seconds=time.monotonic()-start,runner_sha256=q.sha(__file__)))+'\n')
    assert cpu+own<=60,(cpu,own)
