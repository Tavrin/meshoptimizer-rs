#!/usr/bin/env python3
"""Lane C adapters; reuse the registered RFC113 statistics and admission code."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import datetime
import hashlib
import json
import os
from pathlib import Path
import statistics
import struct
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
ART = Path(_public_path('<scratch>/meshopt-cluster-c'))
TARGET = Path(_public_path('<target-dir>'))
os.environ['CARGO_TARGET_DIR'] = str(TARGET)
os.environ['MESHOPT_RFC113_ART_DIR'] = str(ART)
sys.path.insert(0, str(HERE.parent))
import run as clod
import timing as protocol


def write(path, data):
    with path.open('x') as f:
        json.dump(data, f, indent=2)
        f.write('\n')


def event(kind, **kw):
    with (ART/'campaign.jsonl').open('a') as f:
        f.write(json.dumps(dict(event=kind, utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), **kw))+'\n')


def paths(profile):
    return {'cpp': TARGET/'cluster-c-cpp', 'rust': TARGET/profile/'rfc113-cluster-c'}


def prepare():
    manifest = json.loads(Path(_public_path('<scratch>/meshopt-cluster-dag/MANIFEST.json')).read_text())
    for p, h in manifest['pins'].items():
        assert clod.sha(p) == h, p
    inputs = ART/'inputs'
    inputs.mkdir(exist_ok=True)
    cases = []
    for mesh in sorted(clod.MESHES.glob('*.mesh')):
        payload, meta = clod.prepare(mesh, 48)
        payload = payload[:16] + struct.pack('<II', 0, 0) + payload[16:]
        case = dict(set='S2', name=mesh.stem, source=str(mesh), source_sha256=clod.sha(mesh), **meta)
        case['input'] = str(inputs/('S2-'+mesh.stem+'.bin'))
        Path(case['input']).write_bytes(payload)
        case['input_sha256'] = hashlib.sha256(payload).hexdigest()
        cases.append(case)
    assert len(cases) == 9
    s1 = []
    for row in manifest['files']:
        b = Path(row['source']).read_bytes()
        assert hashlib.sha256(b).hexdigest() == row['source_sha256']
        version, ll = struct.unpack_from('<IH', b, 4)
        layout = b[10:10+ll].decode()
        stride = {'pos3-norm3-uv2-tangent4':48, 'pos3-norm3-uv2-tangent4-color4':64}[layout]
        at = 10+ll
        nv, ni = struct.unpack_from('<II', b, at)
        at += 8 + (4 if version >= 3 else 0)
        vb, = struct.unpack_from('<Q', b, at)
        at += 8
        assert vb == nv*stride
        vertices = b[at:at+vb]
        at += vb
        ib, = struct.unpack_from('<Q', b, at)
        at += 8
        assert ib == ni*4
        payload = struct.pack('<4sIIIII', b'R113', nv, ni, stride, 1, int(row['dilated'])) + vertices + b[at:at+ib] + bytes(nv)
        case = dict(set='S1', name=row['mesh'], source=row['source'], source_sha256=row['source_sha256'], vertices=nv, indices=ni, stride=stride, dilated=row['dilated'], oracle=row['oracle'], oracle_sha256=row['oracle_sha256'], input=str(inputs/('S1-'+row['mesh']+'.bin')), source_size=len(b), input_sha256=hashlib.sha256(payload).hexdigest())
        Path(case['input']).write_bytes(payload)
        s1.append(case)
    assert len(s1) == 41
    milestone = min((r for r in s1 if not r['dilated']), key=lambda r:r['source_size'])
    s1.sort(key=lambda r:(r['name'] != milestone['name'], r['name']))
    cases = s1 + cases
    write(ART/'cases.json', cases)
    event('INPUTS_FROZEN', count=len(cases), milestone=milestone['name'])


def preflight():
    rows = []
    for profile in ('consumer', 'release'):
        pool = {side:clod.Driver(path, min(os.sched_getaffinity(0)-{0,1})) for side,path in paths(profile).items()}
        try:
            for case in json.loads((ART/'cases.json').read_text()):
                data = Path(case['input']).read_bytes()
                outputs = {side:driver.run(data)[0] for side,driver in pool.items()}
                assert outputs['cpp'] == outputs['rust'], (profile, case['set'], case['name'], clod.first_mismatch(outputs['cpp'],outputs['cpp'],outputs['rust'],'cluster_c preflight'))
                if case['set'] == 'S1':
                    oracle = Path(case['oracle']).read_bytes()
                    assert hashlib.sha256(oracle).hexdigest() == case['oracle_sha256']
                    assert outputs['rust'] == oracle, case['name']
                rows.append(dict(profile=profile,set=case['set'],name=case['name'],exact=True,sha256=hashlib.sha256(outputs['rust']).hexdigest()))
                print('PREFLIGHT', profile, case['set'], case['name'], 'MATCH', flush=True)
        finally:
            for d in pool.values(): d.close()
    write(ART/'preflight.json', rows)


def profile_instructions():
    cases = json.loads((ART/'cases.json').read_text())
    case = cases[0]
    records = []
    perf = '/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf'
    for profile in ('consumer', 'release'):
        for side, binary in paths(profile).items():
            payload = Path(case['input']).read_bytes()
            framed = struct.pack('<I', len(payload))+payload
            # Untimed requests only; counts are iteration evidence, never verdict samples.
            data = framed * 100
            stem = ART/f'instructions-counts-{profile}-{side}'
            with stem.with_suffix('.stderr').open('x') as err:
                r = subprocess.run([perf,'stat','-x',';','-e','instructions:u','-o',str(stem)+'.stat','--',str(binary)],input=data,stdout=subprocess.DEVNULL,stderr=err)
            record = dict(profile=profile,side=side,case=case['name'],requests=100,exit=r.returncode,binary_sha256=clod.sha(binary),input_sha256=case['input_sha256'])
            record['sampling_status'] = 'v1/v2 raw captures retained; postprocessing stalled. Total instructions are the available iteration evidence.'
            records.append(record)
    write(ART/'instructions.json', records)


def train_blockers():
    blockers=[]
    coord=Path(_public_path('<local>/downstream-coord'))
    for filename in ('heavy.waiting','heavy.reservations'):
        for line in (coord/filename).read_text().splitlines():
            words=line.split()
            if not words:continue
            try:
                pid=int(words[0]);proc=Path('/proc')/str(pid)
                cmd=(proc/'cmdline').read_bytes().replace(b'\0',b' ').decode(errors='replace')
                cwd=str((proc/'cwd').resolve())
                if '/downstream-lane-worktrees/train-' in cmd or '/downstream-lane-worktrees/train-' in cwd or any(w.startswith('train') for w in words[1:]):
                    blockers.append(dict(queue=filename,line=line,cwd=cwd,cmd=cmd))
            except (OSError,ValueError):pass
    return blockers


def train_wait():
    while True:
        blockers=train_blockers()
        if not blockers:return
        event('PAUSE',reason='train-first between cases',blockers=blockers)
        time.sleep(30)


def timed_case(profile, index):
    assert (ART/'OWNER-ADMISSION-AMENDMENT.json').is_file(), 'owner admission amendment must be recorded'
    frozen=json.loads((ART/'READY.json').read_text())
    assert all(clod.sha(ROOT/p)==h for p,h in frozen['sources'].items()), 'source identity drift'
    assert clod.sha(ART/'REGISTRATION.md')==frozen['registration_sha256']
    cases=json.loads((ART/'cases.json').read_text())
    case=cases[index]
    protocol.profile=profile;protocol.case=case['set']+'/'+case['name']
    binaries=paths(profile)
    identities={s:clod.sha(p) for s,p in binaries.items()}
    data=Path(case['input']).read_bytes();assert clod.sha(case['input'])==case['input_sha256']
    payload=b'R11T'+data[4:]
    pool=None;burst=None;bursts=[];stage2=False;fixed=None;selection=None
    def close():
        nonlocal pool,burst
        if pool:
            for d in pool.values():d.close()
            end=clod.cpu_snapshot()
            burst['duration_s']=time.monotonic()-burst.pop('_start')
            snap=burst.pop('_snapshot')
            burst['load_after']=os.getloadavg()
            burst['sibling_utilization']={str(c):clod.utilization(snap,end,c) for c in selection['siblings']}
            bursts.append(burst);pool=None;burst=None
    def ready():
        nonlocal pool,burst,selection,fixed
        if burst and time.monotonic()-burst['_start']>=840:
            close();event('PAUSE',reason='840-second burst cooldown');protocol.idle_wait(60)
        # Owner amendment: only ops/train admission may block this campaign.
        # Heavy admission governs ops; the controller yields to trains between cases.
        if pool is None:
            selection=fixed if stage2 and fixed else protocol.choose_core()
            if stage2:fixed=selection
            pool={s:clod.Driver(p,selection['core']) for s,p in binaries.items()}
            burst=dict(selection=selection,load_before=os.getloadavg(),_start=time.monotonic(),_snapshot=clod.cpu_snapshot())
    expected=None
    def pair(i,stage):
        nonlocal expected
        ready();row={};outputs={}
        event('PAIR_START',profile=profile,set=case['set'],name=case['name'],pair=i+1,stage=stage,core=selection['core'])
        for side in (('cpp','rust') if i%2==0 else ('rust','cpp')):
            outputs[side],row[side]=pool[side].run(payload)
        assert outputs['cpp']==outputs['rust'], 'timed parity mismatch'
        digest=hashlib.sha256(outputs['rust']).hexdigest()
        if expected is None:expected=digest
        assert digest==expected,'nondeterministic output'
        event('PAIR_COMPLETE',profile=profile,set=case['set'],name=case['name'],pair=i+1,stage=stage,times_s=row,output_sha256=digest)
        return row
    try:
        ready()
        outputs={s:d.run(data)[0] for s,d in pool.items()}
        assert outputs['cpp']==outputs['rust']
        if case['set']=='S1':assert clod.sha(case['oracle'])==hashlib.sha256(outputs['rust']).hexdigest()
        if index == 0:
            aa=[];second=None;aa_core=None;aa_load=os.getloadavg()
            try:
                for i in range(20):
                    ready()
                    if second is None or aa_core != selection['core']:
                        if second is not None:second.close()
                        aa_core=selection['core'];second=clod.Driver(binaries['cpp'],aa_core)
                        assert second.run(data)[0]==outputs['cpp']
                    drivers={'cpp':pool['cpp'],'rust':second};row={}
                    for side in (('cpp','rust') if i%2==0 else ('rust','cpp')):
                        out,row[side]=drivers[side].run(payload)
                        assert out==outputs['cpp']
                    aa.append(row)
                    event('AA_PAIR',profile=profile,case=case['name'],pair=i+1,core=aa_core,times_s=row,load_average=os.getloadavg())
            finally:
                if second is not None:second.close()
            ci=protocol.interval(aa)
            write(ART/('AA-'+profile+'.json'),dict(profile=profile,case=case['name'],samples=aa,interval=ci,median_ratio=statistics.median(r['rust'] for r in aa)/statistics.median(r['cpp'] for r in aa),floor=max(abs(ci['lower']-1),abs(ci['upper']-1)),load_before=aa_load,load_after=os.getloadavg(),diagnostic_only=True,binary_sha256=identities['cpp']))
        samples=[];extension=[]
        for i in range(20):
            samples.append(pair(i,1))
            if len(samples)>=5:
                decision=protocol.interval(samples)
                if decision['verdict']!='BORDERLINE':break
        ext_decision=None
        if decision['verdict']=='BORDERLINE':
            close();stage2=True
            for i in range(30):extension.append(pair(i,2))
            ext_decision=protocol.interval(extension)
            if ext_decision['verdict']=='BORDERLINE':ext_decision['verdict']='INCONCLUSIVE'
        close()
        medians={s:statistics.median(r[s] for r in samples) for s in binaries}
        result=dict(profile=profile,case=case,medians_s=medians,ratio=medians['rust']/medians['cpp'],samples=samples,decision=decision,extension=extension,extension_decision=ext_decision,bursts=bursts,binary_sha256=identities,output_sha256=expected,head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip())
        assert identities=={s:clod.sha(p) for s,p in binaries.items()}
        destination=ART/'timing'/f'{index:02}-{profile}.json';destination.parent.mkdir(exist_ok=True)
        write(destination,result)
        print('CASE_COMPLETE',profile,case['set'],case['name'],result['ratio'],flush=True)
    finally:close()


if __name__=='__main__':
    action=sys.argv[1]
    if action=='prepare':prepare()
    elif action=='preflight':preflight()
    elif action=='instructions':profile_instructions()
    elif action=='train-wait':train_wait()
    elif action=='case':timed_case(sys.argv[2],int(sys.argv[3]))
