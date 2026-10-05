#!/usr/bin/env python3
"""Independently verify the scoped fix-four archives and numeric verdicts."""
import hashlib, json, math, os, re, statistics, struct, zipfile
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
ART = Path(os.environ['MESHOPT_ARTIFACTS'])
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
read = lambda name: json.loads((ART / (name + '.json')).read_text())
gm = lambda xs: math.exp(statistics.mean(math.log(x) for x in xs))

def interval(a, b):
    logs = [math.log(x / y) for x, y in zip(a, b)]
    n = len(logs)
    t = [2.776,2.571,2.447,2.365,2.306,2.262,2.228,2.201,2.179,2.160,2.145,2.131,2.120,2.110,2.101,2.093][n-5] if n <= 20 else 2.045
    radius = t * statistics.stdev(logs) / math.sqrt(n)
    center = statistics.mean(logs)
    return [math.exp(center-radius), math.exp(center+radius)]

def same(a, b):
    assert len(a) == len(b) and all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(a,b)), (a,b)

def family(name):
    if name.startswith('meshlet'): return 'meshlet-raw' if name.startswith('meshlet-raw') else 'meshlet'
    if name.startswith('index'): return 'sequence' if name.startswith('index-3') else 'index'
    if 'filter-' in name: return ['oct','quat','exp','color'][int(name.split('filter-')[1][0])-1]
    if 'view-' in name: return 'view-none' if 'view-none' in name else 'view-filtered'
    return 'vertex'

build = read('build')
for name,h in build['sources'].items(): assert sha(ROOT/name) == h, name
for name,h in build['binaries'].items(): assert sha(ART/'bin'/name) == h, name
inputs = read('inputs')
old_inputs = json.loads((ART.parent/'p07r3/inputs.json').read_text())
assert inputs == old_inputs
for r in inputs: assert sha(ART/r['path']) == r['sha256']
assert sha(ROOT/'parity/SIMD_BAR.md') == json.loads((ART.parent/'p07r3/build.json').read_text())['sources']['parity/SIMD_BAR.md']
proofs = {}
for label in ['fixtures','malformed','fixtures04','malformed04','benchmark-identity','sweep02']:
    rec=read(label)
    assert rec['sources']==build['sources'] and rec['binaries']==build['binaries'] and rec['mismatches']==0
    assert sha(ART/(label+'.zip')) == rec['archive_sha256']
    with zipfile.ZipFile(ART/(label+'.zip')) as z:
        assert len(z.namelist())==2*len(rec['cases'])
        for r in rec['cases']:
            assert hashlib.sha256(z.read(r['case']+'.input')).hexdigest()==r['input_sha256']
            if r['status']==0: assert hashlib.sha256(z.read(r['case']+'.output')).hexdigest()==r['output_sha256']
    proofs[label]=len(rec['cases'])
failure=read('sweep04-failure')
assert failure['sources']==build['sources'] and failure['binaries']==build['binaries']
assert failure['status']=='failed-expanded-cpp-simd-conformance'
assert sha(ART/'sweep04.zip')==failure['archive_sha256'] and sha(ART/'sweep.log')==failure['log_sha256']
repro=read('seed-failure-reproduction');assert repro==failure['reproduction']
assert sha(ART/(repro['case']+'.input'))==repro['input_sha256']
rr={r['backend']:r for r in repro['rows']}
assert rr['cpp-simd']['status']==-3 and rr['cpp-scalar']['status']==0
assert all(r['status']==0 and r['output_sha256']==rr['cpp-scalar']['output_sha256'] for k,r in rr.items() if k!='cpp-simd')
proofs['expanded_sweep04']={'verdict':'FAIL; pre-existing upstream scalar/SIMD odd-tail status disagreement','completed_before_failure':failure['completed_cases_before_failure'],'first_failed_case':failure['first_failed_case']}
golden={r['case']:r for r in read('benchmark-identity')['cases']}
node_build=read('node-build')
for name,h in node_build['binaries'].items(): assert sha(ART/'node-bin'/name)==h
preflight=read('node-preflight');assert preflight['passed']
wasm_names={r['case'] for r in inputs if struct.unpack_from('<I',(ART/r['path']).read_bytes(),4)[0] in [1,3,7]}
assert {(r['case'],r['backend'],r['into']) for r in preflight['rows']} == {(n,b,i) for n in wasm_names for b in ['simd','scalar'] for i in [0,1]}
for r in preflight['rows']: assert r['output_sha256']==golden[r['case']]['output_sha256']

native,wasm=read('performance'),read('wasm-performance')
families={'vertex','view-none','view-filtered','color','meshlet','meshlet-raw','sequence'}
native_names={r['case'] for r in inputs if family(r['case']) in families}
summary={'proof_cases':proofs,'native_rows':len(native['rows']),'wasm_rows':len(wasm['rows']),'families':{},'s3_failures':[],'sequence_matched_before':[],'s4':{},'wasm':{},'limits':'Scoped touched families; unchanged triangle index and standalone Oct/Quat/Exp timing is not requalified.'}
old_native=json.loads((ART.parent/'p07r3/performance.json').read_text())
old_wasm=json.loads((ART.parent/'p07r3/wasm-performance.json').read_text())
old_n={(r['api'],r['case']):r for r in old_native['rows']}
old_w={(r['api'],r['case']):r for r in old_wasm['rows']}
for record,names,is_wasm in [(native,native_names,False),(wasm,wasm_names,True)]:
    assert record['complete'] and record['sources']==build['sources'] and record['binaries']==build['binaries']
    keys=[(r['api'],r['case']) for r in record['rows']]
    assert len(keys)==len(set(keys))==2*len(names) and set(keys)=={(a,n) for a in ['allocating','caller-buffer'] for n in names}
    if is_wasm: assert record['timing_binaries']==node_build['binaries']
    for segment in record['controller_segments']:
        assert sha(ART/segment['executed_controller_path'])==segment['executed_controller_sha256']
        assert sha(ROOT/segment['source'])==segment['sha256']
    for burst in record['lease_bursts']: assert burst['wall_seconds']<840
    for row in record['rows']:
        raw=row['raw_seconds'];n=len(raw['rust']);assert 5<=n<=20 and all(len(x)==n for x in raw.values())
        base='cpp' if is_wasm else 'simd';ci=interval(raw['rust'],raw[base])
        same(ci,row.get('stage1_interval',row['interval']))
        bars=[1.5,1.6] if is_wasm else [1.5,1.3 if family(row['case']) in ['vertex','view-none','meshlet','meshlet-raw'] else 1.5]
        borderline=any(ci[0]<=bar<ci[1] for bar in bars)
        if not is_wasm and family(row['case'])=='sequence':
            scalar_ci=interval(raw['rust'],raw['cpp-scalar']);borderline |= scalar_ci[0]<=1.5<scalar_ci[1]
        assert ('stage2' in row)==borderline and (n==20 or not borderline)
        stages=[(1,raw,row['telemetry'])]
        if 'stage2' in row:
            stage=row['stage2'];assert all(len(x)==30 for x in stage['raw_seconds'].values())
            same(interval(stage['raw_seconds']['rust'],stage['raw_seconds'][base]),stage['interval'])
            stages.append((2,stage['raw_seconds'],stage['telemetry']))
        # Early stopping must not have been possible on a prior accepted prefix.
        for stop in range(5,n):
            prefix={k:v[:stop] for k,v in raw.items()};prior=interval(prefix['rust'],prefix[base])
            unsettled=any(prior[0]<=bar<prior[1] for bar in bars)
            if not is_wasm and family(row['case'])=='sequence':
                sc=interval(prefix['rust'],prefix['cpp-scalar']);unsettled |= sc[0]<=1.5<sc[1]
            assert unsettled,(row['case'],stop,'early stopping')
        for stage_no,values,telemetry in stages:
            assert len(telemetry)==len(values['rust'])
            for t in telemetry:
                for side in ['before','after']:
                    receipt=t[side];assert receipt['admitted'] and receipt['queue_receipt'] and not receipt['measuring']
                    assert receipt['declared_gb']==4 and receipt['lease_holder'].startswith('heavy:timeout|')
                    pid=int(receipt['lease_holder'].split('|')[1]);assert pid in {v['pid'] for v in receipt['ancestors']}
            if not is_wasm and family(row['case'])!='sequence':
                for backend in ['rust','sse2']:
                    if backend in values:
                        sc=interval(values[backend],values['scalar'])
                        if sc[0]>1:summary['s3_failures'].append({'case':row['case'],'api':row['api'],'backend':backend,'stage':stage_no,'interval':sc})
        med={k:statistics.median(v) for k,v in raw.items()}
        assert math.isclose(row['time_ratio'],med['rust']/med[base],rel_tol=1e-12)
        if not is_wasm:
            for k,h in row['output_sha256'].items():
                if k!='simd' or family(row['case']) not in ['color','view-filtered']:
                    assert h==golden[row['case']]['output_sha256'],(row['case'],k)
        if not is_wasm and family(row['case'])=='sequence':
            assert 'before' in raw
            summary['sequence_matched_before'].append({'case':row['case'],'api':row['api'],'before_cpp_ratio':med['before']/med['simd'],'after_cpp_ratio':row['time_ratio'],'after_before_ratio':med['rust']/med['before'],'before_cpp_interval':interval(raw['before'],raw['simd'])})
for api in ['allocating','caller-buffer']:
    summary['families'][api]={}
    for f in sorted(families):
        rows=[r for r in native['rows'] if r['api']==api and family(r['case'])==f]
        strict_gm=1.10 if f in ['vertex','view-none','meshlet','meshlet-raw'] else 1.25
        strict_max=1.30 if strict_gm==1.10 else 1.50
        mean=gm(r['time_ratio'] for r in rows)
        summary['families'][api][f]={'before':gm(old_n[api,r['case']]['time_ratio'] for r in rows),'after':mean,'worst_stage1_ratio':max(r['time_ratio'] for r in rows),'registered_pass':mean<=strict_gm and all(r['interval'][1]<=strict_max for r in rows),'brief_pass':mean<=1.25 and all(r['interval'][1]<=1.50 for r in rows),'failed_registered_cases':[r['case'] for r in rows if r['interval'][1]>strict_max]}
    seq=[r for r in native['rows'] if r['api']==api and family(r['case'])=='sequence']
    minima={m.group(1):float(m.group(2)) for m in re.finditer(r'^\| (index-3-\S+) \|[^\n]*\| ([\d.]+) \|$',(ROOT/'parity/DECODER_BAR.md').read_text(),re.M)}
    minimum_failures=[];max_failures=[]
    for r in seq:
        b=next((ART/x['path']).read_bytes() for x in inputs if x['case']==r['case']);count,stride=struct.unpack_from('<II',b,8)
        if count*stride/statistics.median(r['raw_seconds']['rust'])/1e6<minima[r['case']]:minimum_failures.append(r['case'])
        raw=r['stage2']['raw_seconds'] if 'stage2' in r else r['raw_seconds']
        if interval(raw['rust'],raw['cpp-scalar'])[1]>1.5:max_failures.append(r['case'])
    scalar_gm=gm(statistics.median(r['raw_seconds']['rust'])/statistics.median(r['raw_seconds']['cpp-scalar']) for r in seq)
    summary['s4'][api]={'scope':'sequence only','scalar_cpp_geomean':scalar_gm,'maximum_failures':max_failures,'minimum_failures':minimum_failures,'pass':scalar_gm<=1.25 and not max_failures and not minimum_failures}
    wr=[r for r in wasm['rows'] if r['api']==api];mean=gm(r['time_ratio'] for r in wr)
    summary['wasm'][api]={'before':gm(old_w[api,r['case']]['time_ratio'] for r in wr),'after':mean,'worst_stage1_ratio':max(r['time_ratio'] for r in wr),'registered_pass':mean<=1.25 and all(r['interval'][1]<=1.6 for r in wr),'brief_pass':mean<=1.25 and all(r['interval'][1]<=1.5 for r in wr),'failed_registered_cases':[r['case'] for r in wr if r['interval'][1]>1.6]}
summary['s3_significant_comparisons']=len({(r['api'],r['case'],r['backend']) for r in summary['s3_failures']})
checks=read('checks');assert len(checks)==7 and all(r['exit_code']==0 and sha(ART/(r['name']+'.log'))==r['log_sha256'] for r in checks)
assert read('counters')['complete']
summary['verified']=True
(ART/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
