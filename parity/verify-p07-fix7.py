#!/usr/bin/env python3
"""Recompute the scoped RFC verdicts and validate the paired retained controls."""
import hashlib, json, math, os, re, statistics, struct, subprocess, zipfile
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
ART = Path(os.environ['MESHOPT_ARTIFACTS'])
OLD = ART.parent / 'p07-fix6'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
read = lambda n: json.loads((ART / (n + '.json')).read_text())
gm = lambda xs: math.exp(statistics.mean(math.log(x) for x in xs))

def interval(a, b):
    assert len(a) == len(b) and len(a) >= 5
    xs = [math.log(x/y) for x, y in zip(a, b)]
    n = len(xs)
    t = [2.776,2.571,2.447,2.365,2.306,2.262,2.228,2.201,2.179,2.160,2.145,2.131,2.120,2.110,2.101,2.093][n-5] if n <= 20 else 2.045
    c = statistics.mean(xs); r = t * statistics.stdev(xs) / math.sqrt(n)
    return [math.exp(c-r), math.exp(c+r)]

def same(a, b):
    assert len(a) == len(b) and all(math.isclose(x,y,rel_tol=1e-12) for x,y in zip(a,b)), (a,b)

def family(n):
    if n.startswith('meshlet'): return 'meshlet-raw' if n.startswith('meshlet-raw') else 'meshlet'
    if n.startswith('index'): return 'sequence' if n.startswith('index-3') else 'index'
    if 'filter-' in n: return ['oct','quat','exp','color'][int(n.split('filter-')[1][0])-1]
    if 'view-' in n: return 'view-none' if 'view-none' in n else 'view-filtered'
    return 'vertex'

build = read('build'); node = read('node-build')
for n,h in build['sources'].items(): assert sha(ROOT/n) == h, n
for n,h in build['binaries'].items(): assert sha(ART/'bin'/n) == h, n
assert node['sources'] == build['sources']
for n,h in node['binaries'].items(): assert sha(ART/'node-bin'/n) == h
assert b'simd128' in (ART/'bin/wasm-simd.wasm').read_bytes()
assert b'simd128' not in (ART/'bin/wasm-scalar.wasm').read_bytes()
for meta, directory in [('control-build','control-bin'),('control-node-build','control-node-bin')]:
    control = read(meta)
    for n,h in control['binaries'].items(): assert sha(ART/directory/n) == h
    for n,h in control['sources'].items():
        data = subprocess.check_output(['git','show','c87431d:'+n],cwd=ROOT)
        assert hashlib.sha256(data).hexdigest() == h, ('round-six source',n)
assert build['sources']['parity/SIMD_BAR.md'] == read('control-build')['sources']['parity/SIMD_BAR.md']
inputs = read('inputs'); assert inputs == json.loads((OLD/'inputs.json').read_text())
for r in inputs: assert sha(ART/r['path']) == r['sha256']
proofs = {}
for label in ['fixtures','malformed','fixtures04','malformed04','benchmark-identity']:
    p = read(label); prior = json.loads((OLD/(label+'.json')).read_text())
    assert p['sources'] == build['sources'] and p['binaries'] == build['binaries'] and p['mismatches'] == 0
    assert p['cases'] == prior['cases'], (label,'frozen statuses/bytes differ from round six')
    assert sha(ART/(label+'.zip')) == p['archive_sha256']
    with zipfile.ZipFile(ART/(label+'.zip')) as z:
        assert len(z.namelist()) == 2*len(p['cases'])
        for r in p['cases']:
            assert hashlib.sha256(z.read(r['case']+'.input')).hexdigest() == r['input_sha256']
            if r['status'] == 0: assert hashlib.sha256(z.read(r['case']+'.output')).hexdigest() == r['output_sha256']
    proofs[label] = len(p['cases'])
golden = {r['case']:r['output_sha256'] for r in read('benchmark-identity')['cases']}
pre = read('node-preflight'); assert pre['passed']
wn = {r['case'] for r in inputs if struct.unpack_from('<I',(ART/r['path']).read_bytes(),4)[0] in [1,3,7]}
assert {(r['case'],r['backend'],r['into']) for r in pre['rows']} == {(n,b,i) for n in wn for b in ['simd','scalar','old'] for i in [0,1]}
for r in pre['rows']: assert r['output_sha256'] == golden[r['case']]
for label in ['color-depth-proof','upstream-tail-reproduction','arithmetic-proof']:
    p = read(label); assert p['sources'] == build['sources']
    if label != 'arithmetic-proof': assert p['binaries'] == build['binaries']
    if label == 'upstream-tail-reproduction':
        for r in p['rows']:
            assert r['status'] == (-3 if r['backend'] == 'cpp-simd' else 0)
            if r['status'] == 0: assert r['output_sha256'] == '84f10ea2dbf10053d297030ad90332fba171139f972049575862fda7f5c777f7'
    else: assert p['passed']
    if label == 'color-depth-proof':
        for stride,first,last in [(4,1,255),(8,4,65535)]:
            rows = [r for r in p['rows'] if r['stride'] == stride]
            assert len(rows) == 7 and len({r['output_sha256'] for r in rows}) == 1
            for r in rows: assert r['status'] == 0 and r['count'] == 4*(last-first+1) and r['alpha_range'] == [first,last]
    if label == 'arithmetic-proof':
        for r in p['rows']:
            assert r['exit_code'] == 0 and sha(ART/r['log']) == r['log_sha256']
            b = ART/('dev-bin/final-arithmetic' if r['backend'] == 'native' else 'node-bin/arithmetic-simd.wasm')
            assert sha(b) == r['binary_sha256']
checks = read('checks'); assert len(checks) == 7
for r in checks:
    assert r['exit_code'] == 0 and sha(ART/(r['name']+'.log')) == r['log_sha256']
    if r.get('retained_before_test_only_change'):
        assert r['name'] == 'miri-simd' and r['source_head'] == '3ed50bb'
        for n,h in r['source_hashes'].items():
            if n != 'src/codec/simd/mod.rs': assert build['sources'][n] == h, ('retained Miri source',n)
        old_module = subprocess.check_output(['git','show','3ed50bb:src/codec/simd/mod.rs'],cwd=ROOT).decode()
        now_module = (ROOT/'src/codec/simd/mod.rs').read_text()
        assert old_module.split('\n#[cfg(test)]')[0] == now_module.split('\n#[cfg(test)]')[0]

for label in ['portability-checks','safety-gates']:
    for r in read(label)['commands']: assert r['exit_code'] == 0 and sha(ART/r['log']) == r['log_sha256']
assert read('safety-gates')['passed']
counts = read('final-counters'); assert counts['complete'] and counts['sources'] == build['sources']
for r in counts['rows']:
    assert r['binary_sha256'] == build['binaries']['rust-simd']
    for run in r['runs']: assert run['exit_code'] == 0 and run['output_sha256'] == golden[r['case']]

families = {'vertex','view-none','view-filtered','oct','quat','exp','meshlet','meshlet-raw'}
summary = {'verified':True,'bar':'Owner RFC only: family/API geomean <=1.25, final maximum interval upper <=1.50; native varied-filter S3 has no significant slowdown.', 'proof_cases':proofs,'native':{},'wasm':{},'s3_failures':[],'prior_maximum_failures':read('prior-failures')}
receipts = read('timing-bursts')['rows']
for kind,record,names,base in [('native',read('performance'),{r['case'] for r in inputs if family(r['case']) in families},'simd'),('wasm',read('wasm-performance'),wn,'cpp')]:
    assert record['complete'] and record['sources'] == build['sources'] and record['binaries'] == build['binaries']
    if kind == 'wasm': assert record['timing_binaries'] == node['binaries']
    keys = [(r['api'],r['case']) for r in record['rows']]
    assert len(keys) == len(set(keys)) == 2*len(names)
    assert set(keys) == {(a,n) for a in ['allocating','caller-buffer'] for n in names}
    for seg in record['controller_segments']:
        assert sha(ROOT/seg['source']) == seg['sha256']
        assert sha(ART/seg['executed_controller_path']) == seg['executed_controller_sha256']
        expected = read('control-build' if kind == 'native' else 'control-node-build')['binaries']
        assert seg['control_binaries'] == expected
        if kind == 'wasm': assert sha(ROOT/'parity/wasm-p07-fix7-bench.mjs') == seg['node_controller_sha256']
    for burst in record['lease_bursts']: assert burst['wall_seconds'] < 840
    for row in record['rows']:
        raw = row['raw_seconds']; n = len(raw['rust'])
        assert 5 <= n <= 20 and 'old' in raw and all(len(v) == n for v in raw.values())
        ci = interval(raw['rust'],raw[base]); same(ci,row.get('stage1_interval',row['interval']))
        borderline = ci[0] <= 1.5 < ci[1]
        assert ('stage2' in row) == borderline and (n == 20 or not borderline)
        for prefix in range(5,n):
            pc = interval(raw['rust'][:prefix],raw[base][:prefix]); assert pc[0] <= 1.5 < pc[1], (row['case'],prefix,'early stop')
        stages = [(raw,row['telemetry'])]
        if 'stage2' in row:
            stage = row['stage2']; assert all(len(v) == 30 for v in stage['raw_seconds'].values())
            same(interval(stage['raw_seconds']['rust'],stage['raw_seconds'][base]),stage['interval'])
            stages.append((stage['raw_seconds'],stage['telemetry']))
        final = stages[-1][0]
        same(interval(final['rust'],final['old']),row['paired_new_old_interval'])
        assert math.isclose(row['paired_new_old_ratio'],statistics.median(x/y for x,y in zip(raw['rust'],raw['old'])),rel_tol=1e-12)
        assert math.isclose(row['time_ratio'],statistics.median(raw['rust'])/statistics.median(raw[base]),rel_tol=1e-12)
        for values,telemetry in stages:
            assert len(telemetry) == len(values['rust'])
            for t in telemetry:
                assert set(t['order']) == set(values)
                for side in ['before','after']:
                    receipt = t[side]; assert receipt['admitted'] and not receipt['measuring']
                    assert receipt['declared_gb'] == 4 and receipt['lease_holder'].startswith('heavy:timeout|')
                    assert int(receipt['lease_holder'].split('|')[1]) in {v['pid'] for v in receipt['ancestors']}
                    assert receipt['queue_receipt'].split()[0] == str(receipt['heavy_pid'])
        if kind == 'native':
            for k,h in row['output_sha256'].items():
                if k != 'simd' or family(row['case']) not in ['oct','quat','view-filtered']: assert h == golden[row['case']], (row['case'],k)
            if row['case'].startswith('varied-filter-'):
                sc = interval(final['rust'],final['scalar'])
                if sc[0] > 1: summary['s3_failures'].append({'api':row['api'],'case':row['case'],'interval':sc})
    prior = json.loads((OLD/('performance.json' if kind=='native' else 'wasm-performance.json')).read_text())
    prior = {(r['api'],r['case']):r for r in prior['rows']}
    for api in ['allocating','caller-buffer']:
        summary[kind][api] = {}
        for f in sorted({family(r['case']) for r in record['rows']}):
            rows = [r for r in record['rows'] if r['api'] == api and family(r['case']) == f]
            mean = gm(r['time_ratio'] for r in rows)
            fail = [r['case'] for r in rows if r['interval'][1] > 1.5]
            s3 = [r for r in summary['s3_failures'] if r['api'] == api and family(r['case']) == f] if kind == 'native' else []
            summary[kind][api][f] = {'before':gm(prior[(api,r['case'])]['time_ratio'] for r in rows),'after':mean,'paired_new_old':gm(r['paired_new_old_ratio'] for r in rows),'current_control_cpp':gm(statistics.median(r['raw_seconds']['old'])/statistics.median(r['raw_seconds'][base]) for r in rows),'maximum_failures':fail,'s3_failures':s3,'pass':mean <= 1.25 and not fail and not s3,'rows':len(rows)}
    rr = [r for r in receipts if r['kind'] == kind]
    assert len(rr) == len(record['controller_segments'])
    for r in rr:
        p = ART/r['log']; assert sha(p) == r['log_sha256']
        match = re.search(r"gpu-lease: 'heavy:timeout' released after (\d+)m(\d+)s \(exit 0\)",p.read_text()); assert match
        seconds = int(match[1])*60 + int(match[2]); assert seconds < 840 and seconds == r['wrapper_reported_seconds']
summary['timing_bursts'] = receipts
summary['complete_rfc_pass'] = all(v['pass'] for k in ['native','wasm'] for a in summary[k].values() for v in a.values())
(ART/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
