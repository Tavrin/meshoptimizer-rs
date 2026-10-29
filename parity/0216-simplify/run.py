"""P4 reproducible supervisor; run builds and execution under CPU admission.

P4_SCRATCH, P4_INPUTS, CARGO_TARGET_DIR and P4_PERF are supplied externally.
Stages: prepare, build ARM..., profile, measure CAMPAIGN, results CAMPAIGN.
Scalar arms use std without simd; default arms use std+simd. The peer consumer
retains owned outputs, a warmed workspace, and update's input copies in counts.
"""
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
LANE = Path(__file__).resolve().parent
S = Path(os.environ['P4_SCRATCH'])
T = Path(os.environ['CARGO_TARGET_DIR'])
PERF = os.environ['P4_PERF']
CPU = str(min(os.sched_getaffinity(0))) if 'P4_CPU' not in os.environ else os.environ['P4_CPU']
OPS = ['simplify', 'simplify-attributes', 'simplify-update', 'simplify-sloppy']
ARMS = ['base-scalar', 'base-default', 'candidate-scalar', 'candidate-default', 'cpp-scalar', 'cpp-default']
os.environ.update(LC_ALL='C', CARGO_INCREMENTAL='0', CARGO_BUILD_JOBS='2')

def sha(p):
    with Path(p).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def write(p, obj):
    Path(p).write_text(json.dumps(obj, indent=2, sort_keys=True)+'\n')

def run(name, cmd, **kw):
    d = S / 'logs'
    d.mkdir(parents=True, exist_ok=True)
    p = d / (name+'.log')
    assert not p.exists(), p
    start = time.time()
    with p.open('x') as f:
        r = subprocess.run(cmd, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT, **kw)
    write(d/(name+'.json'), dict(exit=r.returncode, elapsed=time.time()-start, log_sha256=sha(p)))
    print(name, r.returncode, flush=True)
    if r.returncode:
        print(p.read_text()[-4000:], flush=True)
        raise SystemExit(r.returncode)

def prepare():
    S.mkdir(parents=True, exist_ok=True)
    base = S/'base'
    base.mkdir()
    # Read-only archive; no index, reference or worktree state mutation.
    archive = subprocess.check_output(['git', 'archive', '1ecf69abe9f92c17e13dc38fa97be659aee44582'], cwd=ROOT)
    subprocess.run(['tar', 'xf', '-', '-C', str(base)], input=archive, check=True)
    src = Path(os.environ['P4_INPUTS'])
    shutil.copytree(src/'inputs', S/'inputs')
    shutil.copy2(src/'inputs.json', S/'inputs.json')
    for x in json.loads((S/'inputs.json').read_text()):
        assert sha(S/'inputs'/(x['case']+'.bin')) == x['sha256']
    for arm in ARMS:
        d = S/arm
        shutil.copytree(ROOT/'parity/peers', d)
        # Add the update operation; no other measured call shape changes.
        subprocess.run([sys.executable, str(ROOT/'parity/0216-qualify/additional_driver.py'), str(d/'src/main.rs')], check=True)
        manifest = (d/'Cargo.toml').read_text()
        tree = base if arm.startswith('base-') else ROOT
        manifest = manifest.replace('path = "../..", optional = true, features = ["clusterlod"]',
            'path = '+json.dumps(str(tree))+', optional = true, default-features = false, features = '+
            ('["std", "clusterlod", "simd"]' if arm.endswith('default') else '["std", "clusterlod"]'))
        manifest += '\n[profile.release.package."*"]\ndebug = 1\n'
        manifest = manifest.replace('opt-level = 3', 'opt-level = 3\ndebug = 1')
        (d/'Cargo.toml').write_text(manifest)
    write(S/'initial-source.json', {str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'src').rglob('*.rs')})

def build(arms):
    bpath = S/'build-binding.json'
    binding = json.loads(bpath.read_text()) if bpath.exists() else {}
    for arm in arms:
        env = dict(os.environ)
        if arm.startswith('cpp-'):
            env['CXXFLAGS'] = '-g -DNDEBUG -fno-fast-math -ffp-contract=off' + (' -DMESHOPTIMIZER_NO_SIMD' if arm.endswith('scalar') else '')
        target = T/arm
        run('build-'+os.environ.get('P4_BUILD_TAG','')+arm, ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', str(S/arm/'Cargo.toml'),
                           '--no-default-features', '--features', 'cpp' if arm.startswith('cpp-') else 'ours', '--target-dir', str(target)], env=env)
        old=S/('driver-'+arm)
        if old.exists():
            history=S/'binary-history';history.mkdir(exist_ok=True)
            shutil.copy2(old,history/(str(time.time_ns())+'-'+old.name))
        shutil.copy2(target/'release/meshopt-peers', old)
        tree = S/'base' if arm.startswith('base-') else ROOT
        binding[arm] = dict(binary=sha(S/('driver-'+arm)), driver=sha(S/arm/'src/main.rs'),
            sources={str(p.relative_to(tree)):sha(p) for p in (tree/'src').rglob('*.rs')},
            manifest=sha(S/arm/'Cargo.toml'),cpp_flags=env.get('CXXFLAGS',''),
            rustc=subprocess.check_output(['rustc','-Vv'],text=True),cpu=CPU)
        write(bpath, binding)

def invoke(arm, op, case, stem, n, record=False):
    cr,cw = os.pipe(); ar,aw = os.pipe()
    output = Path(str(stem)+'.out')
    cmd = [PERF, 'record' if record else 'stat', '-e', 'instructions:u', '-o', str(stem)+('.data' if record else '.perf')]
    cmd += ['-c', '200003'] if record else ['-x', ';']
    cmd += ['-D', '-1', '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', CPU, str(S/('driver-'+arm)), op,
            str(S/'inputs'/(case+'.bin')), str(output)]
    start = time.time()
    try:
        r = subprocess.run(cmd, input=f'{n}\nstop\n', env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),
                           pass_fds=(cr,cw,ar,aw), capture_output=True, text=True)
    finally:
        for fd in [cr,cw,ar,aw]: os.close(fd)
    Path(str(stem)+'.stderr').write_text(r.stderr)
    assert r.returncode == 0, (cmd, r.stderr)
    lines=r.stdout.splitlines()
    assert lines[0] == 'READY' and lines[1].startswith('COUNTED '), r.stdout
    row=dict(arm=arm,op=op,case=case,repetitions=n,output_sha256=sha(output),
             loop_seconds=float(lines[1].split()[1])/n,process_seconds=time.time()-start,exit=r.returncode)
    output.unlink()
    if not record:
        fields=next(l.split(';') for l in Path(str(stem)+'.perf').read_text().splitlines() if ';instructions:u;' in l)
        assert float(fields[4])==100.0,fields
        row['instructions_per_call']=int(fields[0])/n
    return row

def preserve_incomplete(stem):
    files=list(stem.parent.glob(stem.name+'.*'))+list(stem.parent.glob(stem.name+'-*.txt'))
    if files:
        d=stem.parent/'interrupted'/str(time.time_ns())
        d.mkdir(parents=True)
        for p in files: p.rename(d/p.name)

def profile():
    d=S/'profile'; d.mkdir(exist_ok=True)
    rows=json.loads((d/'profile.json').read_text()) if (d/'profile.json').exists() else []
    done={(v['case'],v['op'],v['arm']) for v in rows}
    for case in ['real-214e','real-a082','real-b6fd']:
        for op in OPS:
            for arm in ['base-scalar','cpp-scalar']:
                if (case,op,arm) in done: continue
                stem=d/(case+'-'+op+'-'+arm)
                preserve_incomplete(stem)
                row=invoke(arm,op,case,stem,2,True)
                for kind,args in [('report',['report','--stdio','--no-children','--inline','--show-nr-samples','--show-total-period','--percent-limit','0.1']),
                                  ('lines',['report','--stdio','--no-children','--inline','--sort','srcline,symbol','--show-nr-samples','--show-total-period','--percent-limit','0.2']),
                                  ('annotate',['annotate','--stdio','-l','--percent-limit','1'])]:
                    with Path(str(stem)+'-'+kind+'.txt').open('w') as f:
                        r=subprocess.run([PERF,*args,'-i',str(stem)+'.data'],stdout=f,stderr=subprocess.PIPE,text=True)
                    assert r.returncode==0,(kind,r.stderr)
                rows.append(row);write(d/'profile.json',rows)
                print('profile',case,op,arm,flush=True)
    shutil.copy2(d/'profile.json',LANE/'profile.json')

def measure(campaign):
    d=S/campaign;d.mkdir(exist_ok=True)
    counts=d/'counts.jsonl'
    rows=[json.loads(s) for s in counts.read_text().splitlines()] if counts.exists() else []
    keys={(r['case'],r['op'],r['arm'],r['rep']) for r in rows}
    binding=json.loads((S/'build-binding.json').read_text())
    for a in ARMS: assert sha(S/('driver-'+a))==binding[a]['binary']
    write(d/'build-binding.json',binding)
    for x in json.loads((S/'inputs.json').read_text()):
        for op in OPS:
            for rep in range(3):
                for arm in ARMS if rep!=1 else list(reversed(ARMS)):
                    key=(x['case'],op,arm,rep)
                    if key in keys: continue
                    stem=d/('-'.join(map(str,key)))
                    preserve_incomplete(stem)
                    n=1000 if x['case']=='tiny-smooth' else 3 if x['case'].startswith('medium') else 1
                    row=invoke(arm,op,x['case'],stem,n)
                    row['rep']=rep
                    with counts.open('a') as f:
                        f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
                    rows.append(row)
                cell=[r for r in rows if r['case']==x['case'] and r['op']==op]
                assert len({r['output_sha256'] for r in cell})==1,(x['case'],op,'output mismatch')
            print('measure',x['case'],op,flush=True)
    shutil.copy2(counts,LANE/(campaign+'-counts.jsonl'))

def results(campaign):
    rows=[json.loads(s) for s in (S/campaign/'counts.jsonl').read_text().splitlines()]
    cells=[]
    for x in json.loads((S/'inputs.json').read_text()):
        for op in OPS:
            cell=[r for r in rows if r['case']==x['case'] and r['op']==op]
            assert len(cell)==18 and len({r['output_sha256'] for r in cell})==1
            med={a:statistics.median(r['instructions_per_call'] for r in cell if r['arm']==a) for a in ARMS}
            sec={a:statistics.median(r['loop_seconds'] for r in cell if r['arm']==a) for a in ARMS}
            cells.append(dict(case=x['case'],op=op,instructions=med,seconds=sec,
                ratios=dict(default_base=med['candidate-default']/med['base-default'],scalar_base=med['candidate-scalar']/med['base-scalar'],
                            scalar_cpp=med['candidate-scalar']/med['cpp-scalar'],default_cpp=med['candidate-default']/med['cpp-default'])))
    summaries={}
    gm=lambda v:math.exp(statistics.mean(math.log(x) for x in v))
    for op in OPS:
        real=[c for c in cells if c['op']==op and c['case'].startswith('real')]
        corpus=[c for c in cells if c['op']==op and not c['case'].startswith('real')]
        allc=real+corpus
        r={k:gm([c['ratios'][k] for c in real]) for k in real[0]['ratios']}
        r['corpus']={k:gm([c['ratios'][k] for c in corpus]) for k in real[0]['ratios']}
        r['max_default_base']=max(c['ratios']['default_base'] for c in allc)
        r['verdict']='SHIPS' if r['default_base']<=.95 and r['max_default_base']<=1.01 else 'NO-GAIN'
        summaries[op]=r
    write(LANE/(campaign+'-results.json'),dict(cells=cells,summaries=summaries,samples=len(rows)))
    print(json.dumps(summaries,indent=2))

if __name__=='__main__':
    stage=sys.argv[1]
    if stage=='prepare':prepare()
    elif stage=='build':build(sys.argv[2:])
    elif stage=='profile':profile()
    elif stage=='measure':measure(sys.argv[2])
    elif stage=='results':results(sys.argv[2])
