#!/usr/bin/env python3
"""Verify retained SIMD identity archives against this checkout and binaries."""
import hashlib,json,os,subprocess,zipfile
from pathlib import Path
root=Path(__file__).resolve().parents[1];art=Path(os.environ['MESHOPT_ARTIFACTS'])
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for name in ['parity/codec/reference.cpp','parity/codec/measure.py','parity/codec/runner.py','parity/codec/runner04.py']:
    original=subprocess.check_output(['git','show','179ba52:'+name],cwd=root)
    assert (root/name).read_bytes()==original,('preregistered harness changed',name)
print('Canonical C++ bridge and inherited harness match preregistration')
for label in ['build','fixtures','fixtures04','malformed','malformed04','benchmark-identity','sweep02','sweep04','exp-exceptional-identity','performance','wasm-performance']:
    p=art/(label+'.json');record=json.loads(p.read_text())
    for name,digest in record['sources'].items():assert sha(root/name)==digest,(label,name)
    for name,digest in record['binaries'].items():assert sha(art/'bin'/name)==digest,(label,name)
    if 'archive_sha256' in record:
        z=art/(label+'.zip');assert sha(z)==record['archive_sha256']
        with zipfile.ZipFile(z) as archive:
            for c in record['cases']:
                for suffix,key in [('input','input_sha256'),('output','output_sha256')]:
                    data=archive.read(c['case']+'.'+suffix)
                    if c[key] is not None:assert hashlib.sha256(data).hexdigest()==c[key],(label,c['case'],suffix)
        assert record['mismatches']==0
    if label.endswith('performance'):assert record['complete']
    print(label,'verified')
for backend in ['native','wasm']:
    directory=art/('exhaustive-'+backend);complete=json.loads((directory/'complete.json').read_text());assert complete['complete']
    identity=complete['identity']
    for name,digest in identity['sources'].items():assert sha(root/name)==digest,(backend,name)
    filename='meshopt-simd-qualification-native' if backend=='native' else 'meshopt_simd_qualification.wasm-wasm'
    assert sha(art/'bin'/filename)==identity['binary_sha256']
    for family,total in complete['records'].items():
        end=0
        records=sorted([json.loads(p.read_text()) for p in directory.glob(family+'-*.json')],key=lambda r:r['start'])
        for r in records:assert r['identity']==identity and r['exit_code']==0 and r['start']==end;end=r['end']
        assert end==total,(backend,family,end,total)
    print(backend,'exhaustive coverage verified')

fuzz=json.loads((art/'fuzz-smoke.json').read_text())
for name,digest in fuzz['sources'].items():
    current=root/name
    candidate=current if sha(current)==digest else art/'source-objects'/digest
    assert sha(candidate)==digest,name
assert sha(art/'fuzz-simd')==fuzz['binary_sha256'] and sha(art/'fuzz.log')==fuzz['log_sha256']
assert fuzz['exit_code']==0
for row in json.loads((art/'fuzz-version-smokes.json').read_text()):
    for name,digest in row['sources'].items():assert sha(root/name)==digest,name
    directory=art/'fuzz-version-smokes'/row['target']
    assert sha(directory/'binary')==row['binary_sha256'] and sha(directory/'smoke.log')==row['log_sha256']
    assert row['exit_code']==0 and all(n>0 for n in row['counts_lower_bounds'][:4])
seeded=json.loads((art/'wasm-seeded-1000.json').read_text())
assert seeded['complete'] and seeded['cases']==1000
for name,digest in seeded['sources'].items():assert sha(root/name)==digest,name
for name,digest in seeded['binaries'].items():assert sha(art/'bin'/name)==digest,name
print('ASan source objects, version smokes and streamed wasm smoke verified')

native=json.loads((art/'performance.json').read_text())
keys=[(r['api'],r['case']) for r in native['rows']]
assert len(keys)==276 and len(set(keys))==276
inputs=json.loads((art/'inputs.json').read_text())
with zipfile.ZipFile(art/'benchmark-identity.zip') as archive:
    for row in inputs:
        p=art/row['path'];assert sha(p)==row['sha256']
        assert p.read_bytes()==archive.read(row['case']+'.input')
assert set(keys)=={(api,row['case']) for api in ['allocating','caller-buffer'] for row in inputs}
for segment in native['controller_segments']:assert sha(root/segment['source'])==segment['sha256']
original=json.loads((art/'performance-before-resume.json').read_text())
by_key={(r['api'],r['case']):r for r in native['rows']}
for row in original['rows']:assert by_key[(row['api'],row['case'])]==row
print('Completed matrix has unique rows and preserves all pre-resume samples')

wasm=json.loads((art/'wasm-performance.json').read_text())
for segment in wasm['controller_segments']:
    assert sha(root/segment['source'])==segment['sha256']
    assert sha(root/segment['original_source'])==segment['original_sha256']
    assert sha(art/'wasm-executed-controller.py')==segment['executed_controller_sha256']
keys=[(r['api'],r['case']) for r in wasm['rows']]
for record in [native,wasm]:
    for row in record['rows']:
        n=len(row['raw_seconds']['rust']);assert 5<=n<=20
        assert all(len(values)==n and all(v>0 for v in values) for values in row['raw_seconds'].values())
        assert len(row['telemetry'])==n
        stages=[row]
        if 'stage2' in row:
            assert n==20 and all(len(v)==30 for v in row['stage2']['raw_seconds'].values())
            assert len(row['stage2']['telemetry'])==30
            stages.append(row['stage2'])
        for stage in stages:
            assert all(t['before']['admitted'] and t['after']['admitted'] for t in stage['telemetry'])
eligible={r['case'] for r in json.loads((art/'inputs.json').read_text())
          if int.from_bytes((art/r['path']).read_bytes()[4:8],'little') in [1,2,3,7]}
assert set(keys)=={(api,name) for api in ['allocating','caller-buffer'] for name in eligible}
assert len(keys)==len(set(keys))
upstream=json.loads((art/'wasm-upstream-identity.json').read_text())
assert upstream['sha256_before']==upstream['sha256_after']
assert sha(Path(upstream['path']))==upstream['sha256_before']
assert sha(art/'upstream-meshopt-decoder.mjs')==upstream['sha256_before']
assert upstream['reference']=='4c203430ca565cb59a468a91922c76c208169536'
assert upstream['shipped_simd_detector']=='true'
conformance=json.loads((art/'wasm-upstream-conformance.json').read_text())
assert conformance['complete'] and conformance['mismatches']==0
assert conformance['script_sha256']==sha(root/'parity/check-simd-js.py')
assert conformance['upstream_sha256']==upstream['sha256_before']
assert conformance['archive_sha256']==sha(art/'benchmark-identity.zip')
assert conformance['output_archive_sha256']==sha(art/'wasm-upstream-conformance.zip')
with zipfile.ZipFile(art/'wasm-upstream-conformance.zip') as outputs:
    for row in conformance['cases']:
        assert hashlib.sha256(outputs.read(row['case']+'.output')).hexdigest()==row['output_sha256']
assert {r['case'] for r in conformance['cases']}==eligible
assert all(r['tail_preserved'] and r['max_unit_difference']<=1 for r in conformance['cases'])
print('Node matrix coverage and shipped upstream SIMD identity verified')

assessment=json.loads((art/'s4-assessment.json').read_text());assert assessment['complete']
assert assessment['native_sha256']==sha(art/'performance.json')
assert assessment['controller_sha256']==sha(root/'parity/measure-simd-s4.py')
assert assessment['sources']==native['sources'] and assessment['binaries']==native['binaries']
by_key={(r['api'],r['case']):r for r in native['rows'] if r['family'] in ['index','sequence']}
assert {(r['api'],r['case']) for r in assessment['rows']}==set(by_key)
for row in assessment['rows']:
    original=by_key[(row['api'],row['case'])]
    assert row['original_pairs']==len(original['raw_seconds']['rust'])
    additional=len(row['additional_raw_seconds']['rust'])
    assert row['original_pairs']+additional<=20
    assert len(row['additional_raw_seconds']['cpp-scalar'])==additional
    if additional:assert row['original_interval'][0]<=1.5<row['original_interval'][1]
    n2=30 if 'stage2' in row else 0
    if n2:assert row['original_pairs']+additional==20 and all(len(v)==30 for v in row['stage2']['raw_seconds'].values())
    assert len(row['telemetry'])==additional+n2
    assert all(t['before']['admitted'] and t['after']['admitted'] for t in row['telemetry'])
print('S4 preserves the matrix and supplements only borderline scalar intervals')
