#!/usr/bin/env python3
"""Continue the one native matrix without repeating completed API/case rows.

Only the admission polling delay changes (15 seconds to 2). Original source,
inputs, executable identities, bars, paired sampling and D146 remain unchanged.
"""
import hashlib,inspect,json,os,runpy,sys,time
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent/'simd'))
import qualify as q
original_sleep=time.sleep
def admission_sleep(seconds):
    original_sleep(2 if seconds==15 else seconds)
time.sleep=admission_sleep
path=q.ART/'performance.json';previous=json.loads(path.read_text());assert not previous.get('complete',False)
assert previous['sources']==q.sources()
assert previous['binaries']=={f.name:q.m.sha(f) for f in q.BIN.iterdir()}
done={(r['api'],r['case']) for r in previous['rows']}
controller={'source':str(Path(__file__).relative_to(q.ROOT)),'sha256':q.m.sha(__file__),'resumed_rows':len(done),'poll_seconds':2}
text=inspect.getsource(q.bench)
text=text.replace("if path.exists() and not diagnostic:raise ValueError('final matrix already exists; do not repeat')","assert not diagnostic")
text=text.replace('raw=[];before=sources();ids=', "raw=previous['rows'];before=sources();ids=")
text=text.replace("    for api in ['allocating','caller-buffer']:", "    record['admissions']=previous['admissions']\n    record['controller_segments']=previous.get('controller_segments',[{'source':'parity/simd/qualify.py','sha256':m.sha(ROOT/'parity/simd/qualify.py'),'poll_seconds':15}])+[controller]\n    for api in ['allocating','caller-buffer']:")
text=text.replace('        for name,b in cases:\n', '        for name,b in cases:\n            if (api,name) in done:continue\n')
assert text.count('if (api,name) in done:continue')==1
namespace=dict(q.__dict__,previous=previous,done=done,controller=controller)
exec(compile(text,str(Path(__file__)), 'exec'),namespace)
namespace['bench'](q.corpus())
assert q.m.sha(__file__)==controller['sha256']
# The Node matrix also uses the faster admission poll. Its code stays unchanged.
runpy.run_path(str(q.ROOT/'parity/simd/wasm_measure.py'),run_name='__main__')
