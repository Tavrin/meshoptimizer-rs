#!/usr/bin/env python3
"""Verify all retained decoder evidence before build-target cleanup."""
import json
from pathlib import Path
import zipfile
from measure import ROOT,ART,TARGET,sha
from runner import sources

def main():
    current=sources();results={}
    for name in ['fixtures','malformed','sweep']:
        r=json.loads((ART/(name+'.json')).read_text());assert r['mismatches']==0 and r['sources']==current,name
        assert sha(r['archive'])==r['archive_sha256']
        with zipfile.ZipFile(r['archive']) as z:
            for case in r['cases']:
                for file in case['files'].values():assert __import__('hashlib').sha256(z.read(file['member'])).hexdigest()==file['sha256']
        results[name]={'cases':len(r['cases']),'record_sha256':sha(ART/(name+'.json'))}
    sweep=json.loads((ART/'sweep.json').read_text());assert all(int(sweep['counts'].get(str(op),0))>=2000 for op in range(1,8))
    candidate=json.loads((ART/'candidate.json').read_text());assert candidate['sources']==current
    assert candidate['registered_bar_sha256']==sha(ROOT/'parity/DECODER_BAR.md')
    assert candidate['baseline_sha256']==sha(ART/'baseline.json')
    assert all(v['minimum_bar_pass'] and v['raw_scalar_bar']['pass'] for v in candidate['verdicts'].values())
    fuzz=json.loads((ART/'fuzz.json').read_text())
    core={str(p.relative_to(ROOT)):sha(p) for p in ROOT.glob('src/**/*.rs')};assert fuzz['sources']==core and len(fuzz['targets'])==13
    for r in fuzz['targets']:assert r['exit_code']==0 and r['elapsed_seconds']>=300 and r['executions']>0 and sha(r['log'])==r['log_sha256']
    gates=json.loads((ART/'gates.json').read_text());assert len(gates)>=9 and all(r['exit_code']==0 and sha(r['log'])==r['log_sha256'] for r in gates)
    for r in json.loads((ART/'benchmark-inputs.json').read_text()):assert sha(r['path'])==r['sha256']
    results.update({'benchmark':'pass','fuzz_targets':13,'required_checks':len(gates)})
    (ART/'verification.json').write_text(json.dumps(results,indent=2));print(results)
if __name__=='__main__':main()
