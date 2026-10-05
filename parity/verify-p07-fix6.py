#!/usr/bin/env python3
"""Independently verify the scoped fix-six archives and numeric verdicts."""
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
assert b'simd128' in (ART/'bin/wasm-simd.wasm').read_bytes()
assert b'simd128' not in (ART/'bin/wasm-scalar.wasm').read_bytes()
inputs = read('inputs')
old_inputs = json.loads((ART.parent/'p07r3/inputs.json').read_text())
assert inputs == old_inputs
for r in inputs: assert sha(ART/r['path']) == r['sha256']
assert sha(ROOT/'parity/SIMD_BAR.md') == json.loads((ART.parent/'p07r3/build.json').read_text())['sources']['parity/SIMD_BAR.md']
proofs = {}
for label in ['fixtures','malformed','fixtures04','malformed04','benchmark-identity']:
    rec=read(label)
    assert rec['sources']==build['sources'] and rec['binaries']==build['binaries'] and rec['mismatches']==0
    assert sha(ART/(label+'.zip')) == rec['archive_sha256']
    with zipfile.ZipFile(ART/(label+'.zip')) as z:
        assert len(z.namelist())==2*len(rec['cases'])
        for r in rec['cases']:
            assert hashlib.sha256(z.read(r['case']+'.input')).hexdigest()==r['input_sha256']
            if r['status']==0: assert hashlib.sha256(z.read(r['case']+'.output')).hexdigest()==r['output_sha256']
    proofs[label]=len(rec['cases'])
repro=read('upstream-tail-reproduction')
assert repro['sources']==build['sources'] and repro['binaries']==build['binaries']
assert hashlib.sha256(bytes.fromhex(repro['input_hex'])).hexdigest()==repro['input_sha256']
for r in repro['rows']:
    assert r['status']==(-3 if r['backend']=='cpp-simd' else 0)
    if r['status']==0: assert r['output_sha256']=='84f10ea2dbf10053d297030ad90332fba171139f972049575862fda7f5c777f7'
proofs['upstream_tail']='expected pinned upstream scalar/SIMD split; all Rust levels match scalar'
golden={r['case']:r for r in read('benchmark-identity')['cases']}
node_build=read('node-build');assert node_build['sources']==build['sources']
color_depth=read('color-depth-proof');assert color_depth['passed'] and color_depth['sources']==build['sources'] and color_depth['binaries']==build['binaries']
for stride,first,last in [(4,1,255),(8,4,65535)]:
    rs=[r for r in color_depth['rows'] if r['stride']==stride]
    assert len(rs)==7 and len({r['backend'] for r in rs})==7 and len({r['output_sha256'] for r in rs})==1
    for r in rs:assert r['status']==0 and r['alpha_range']==[first,last] and r['count']==4*(last-first+1) and sha(ART/f'color-depth-s{stride}.input')==r['input_sha256']
proofs['color_depth']='all valid alpha depths; extreme channels; native levels and actual wasm'
for name,h in node_build['binaries'].items():
    assert sha(ART/'node-bin'/name)==h and b'simd128' in (ART/'node-bin'/name).read_bytes()
preflight=read('node-preflight');assert preflight['passed']
wasm_names={r['case'] for r in inputs if struct.unpack_from('<I',(ART/r['path']).read_bytes(),4)[0] in [1,3,7]}
assert {(r['case'],r['backend'],r['into']) for r in preflight['rows']} == {(n,b,i) for n in wasm_names for b in ['simd','scalar'] for i in [0,1]}
for r in preflight['rows']: assert r['output_sha256']==golden[r['case']]['output_sha256']

native,wasm=read('performance'),read('wasm-performance')
families={'vertex','view-none','view-filtered','oct','quat','exp','meshlet','meshlet-raw'}
native_names={r['case'] for r in inputs if family(r['case']) in families}
summary={'proof_cases':proofs,'native_rows':len(native['rows']),'wasm_rows':len(wasm['rows']),'families':{},'s3_failures':[],'s3_final_failures':[],'wasm':{},'limits':'Scoped decoder/filter families; triangle index, other platforms and release qualification are not requalified.'}
old_native=json.loads((ART.parent/'p07-fix5/performance.json').read_text())
old_wasm=json.loads((ART.parent/'p07-fix5/wasm-performance.json').read_text())
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
        bars=[1.5,1.6] if is_wasm else sorted({1.5,1.3 if family(row['case']) in ['vertex','view-none','meshlet','meshlet-raw'] else 1.5})
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
            if not is_wasm and row['case'].startswith('varied-filter-'):
                for backend in ['rust','sse2']:
                    if backend in values:
                        sc=interval(values[backend],values['scalar'])
                        if sc[0]>1:
                            finding={'case':row['case'],'api':row['api'],'backend':backend,'stage':stage_no,'interval':sc}
                            summary['s3_failures'].append(finding)
                            if stage_no==(2 if 'stage2' in row else 1):summary['s3_final_failures'].append(finding)
        med={k:statistics.median(v) for k,v in raw.items()}
        assert math.isclose(row['time_ratio'],med['rust']/med[base],rel_tol=1e-12)
        if not is_wasm:
            for k,h in row['output_sha256'].items():
                if k!='simd' or family(row['case']) not in ['color','view-filtered','oct','quat']:
                    assert h==golden[row['case']]['output_sha256'],(row['case'],k)
for api in ['allocating','caller-buffer']:
    summary['families'][api]={}
    for f in sorted(families):
        rows=[r for r in native['rows'] if r['api']==api and family(r['case'])==f]
        strict_gm=1.10 if f in ['vertex','view-none','meshlet','meshlet-raw'] else 1.25
        strict_max=1.30 if strict_gm==1.10 else 1.50
        mean=gm(r['time_ratio'] for r in rows)
        summary['families'][api][f]={'before_epoch':'p07-fix5','before':gm(old_n[api,r['case']]['time_ratio'] for r in rows),'after':mean,'worst_stage1_ratio':max(r['time_ratio'] for r in rows),'registered_pass':mean<=strict_gm and all(r['interval'][1]<=strict_max for r in rows),'brief_pass':mean<=1.25 and all(r['interval'][1]<=1.50 for r in rows),'failed_registered_cases':[r['case'] for r in rows if r['interval'][1]>strict_max]}
    wr=[r for r in wasm['rows'] if r['api']==api];mean=gm(r['time_ratio'] for r in wr)
    summary['wasm'][api]={'before':gm(old_w[api,r['case']]['time_ratio'] for r in wr),'after':mean,'worst_stage1_ratio':max(r['time_ratio'] for r in wr),'registered_pass':mean<=1.25 and all(r['interval'][1]<=1.6 for r in wr),'brief_pass':mean<=1.25 and all(r['interval'][1]<=1.5 for r in wr),'failed_registered_cases':[r['case'] for r in wr if r['interval'][1]>1.6]}
summary['named_exp_s3']=[]
for name in ['varied-filter-3-tiny-s32','varied-filter-3-resident-s32']:
    prior=old_n['caller-buffer',name]
    current=next(r for r in native['rows'] if r['api']=='caller-buffer' and r['case']==name)
    before_raw=prior.get('stage2',prior)['raw_seconds'];after_raw=current.get('stage2',current)['raw_seconds']
    before_ci=interval(before_raw['rust'],before_raw['scalar']);after_ci=interval(after_raw['rust'],after_raw['scalar'])
    assert before_ci[0]>1
    summary['named_exp_s3'].append({'api':'caller-buffer','case':name,'before_interval':before_ci,'after_interval':after_ci,'current_significant_slowdown':after_ci[0]>1})
for api in ['allocating','caller-buffer']:
    for f,r in summary['families'][api].items():
        failures=[x for x in summary['s3_final_failures'] if x['api']==api and family(x['case'])==f]
        r['s3_failures']=failures
        r['registered_ratio_pass']=r['registered_pass'];r['brief_ratio_pass']=r['brief_pass']
        r['registered_pass'] &= not failures;r['brief_pass'] &= not failures
summary['s3_final_significant_comparisons']=len(summary['s3_final_failures'])
summary['s3_registered_varied_failures']=[r for r in summary['s3_final_failures'] if r['case'].startswith('varied-')]
summary['s3_significant_comparisons']=len({(r['api'],r['case'],r['backend']) for r in summary['s3_failures']})
bursts=read('timing-bursts')
assert len(bursts['rows'])==len(native['controller_segments'])+len(wasm['controller_segments'])
for receipt in bursts['rows']:
    log=ART/receipt['log'];assert sha(log)==receipt['log_sha256']
    match=re.search(r"gpu-lease: 'heavy:timeout' released after (\d+)m(\d+)s \(exit 0\)",log.read_text());assert match
    seconds=int(match[1])*60+int(match[2]);assert seconds==receipt['wrapper_reported_seconds'] and seconds+1<840
    assert 'moss-heavy.sh' not in receipt.get('nested_wrapper','')
    if receipt['kind']=='native':record=native
    else:record=wasm
    assert receipt['executed_controller_path'] in {x['executed_controller_path'] for x in record['controller_segments']}
summary['timing_bursts']=bursts
checks=read('checks');assert len(checks)==7 and all(r['exit_code']==0 and sha(ART/(r['name']+'.log'))==r['log_sha256'] for r in checks)
counter=read('counters');assert counter['complete'] and counter['sources']==build['sources']
for r in counter['rows']:
    if r['backend']=='after':assert r['binary_sha256']==build['binaries']['rust-simd']
assert all(r['effective_encoded_rustflags']==r['environment_overrides'].get('RUSTFLAGS','').replace(' ', '\x1f') for r in checks)
for r in read('portability-checks')['commands']: assert r['exit_code']==0 and sha(ART/r['log'])==r['log_sha256']
assert read('safety-gates')['passed']
summary['aggregate_bars']={}
for label,fs,bar_mean,bar_max in [('s1',{'vertex','view-none','meshlet','meshlet-raw'},1.10,1.30),('s2',{'view-filtered','oct','quat','exp'},1.25,1.50)]:
    summary[label]={};summary['aggregate_bars'][label]={}
    for api in ['allocating','caller-buffer']:
        rows=[r for r in native['rows'] if r['api']==api and family(r['case']) in fs]
        mean=gm(r['time_ratio'] for r in rows);failed=[r['case'] for r in rows if r['interval'][1]>bar_max]
        summary[label][api]=mean<=bar_mean and not failed
        summary['aggregate_bars'][label][api]={'geomean':mean,'maximum_failures':failed,'pass':summary[label][api]}
arithmetic=read('arithmetic-proof');assert arithmetic['passed'] and arithmetic['sources']==build['sources']
for row in arithmetic['rows']:assert row['exit_code']==0 and sha(ART/row['log'])==row['log_sha256']
summary['arithmetic_proof']=arithmetic
summary['verified']=True
(ART/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
