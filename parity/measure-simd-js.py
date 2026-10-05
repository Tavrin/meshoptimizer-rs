#!/usr/bin/env python3
"""Run the registered Node matrix, adding retained interruption/stage-2 telemetry.

The measured source and binaries are unchanged. The original controller and
this reporting adapter are both hashed. Existing completed keys are preserved.
"""
import json, os, sys, time
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent/'simd'))
import qualify as q
original=q.ROOT/'parity/simd/wasm_measure.py'
path=q.ART/'wasm-performance.json'
previous=json.loads(path.read_text()) if path.exists() else None
if previous:
    assert not previous.get('complete',False)
    assert previous['sources']==q.sources()
    assert previous['binaries']=={p.name:q.m.sha(p) for p in q.BIN.iterdir()}
done={(r['api'],r['case']) for r in previous['rows']} if previous else set()
controller={'source':'parity/measure-simd-js.py','sha256':q.m.sha(__file__),'original_source':'parity/simd/wasm_measure.py','original_sha256':q.m.sha(original),'resumed_rows':len(done),'poll_seconds':2}
original_sleep=time.sleep
def admission_sleep(seconds):
    original_sleep(2 if seconds==15 else seconds)
time.sleep=admission_sleep
text=original.read_text()
text=text.replace("if path.exists():raise ValueError('final wasm matrix already exists; do not repeat')",'assert not previous or not previous.get("complete",False)')
text=text.replace(";burst=time.monotonic()", "\nif previous:record['rows']=previous['rows'];record['admissions']=previous['admissions']\nrecord['controller_segments']=previous.get('controller_segments',[]) + [controller] if previous else [controller]\nburst=time.monotonic()",1)
text=text.replace("        op=int.from_bytes(b[4:8],'little')", "        if ('caller-buffer' if into else 'allocating',name) in done:continue\n        op=int.from_bytes(b[4:8],'little')")
text=text.replace("'telemetry':[],'iterations'", "'telemetry':[],'discarded':[],'iterations'")
text=text.replace("            if not after['admitted']:continue", "            if not after['admitted']:\n                row['discarded'].append({'stage':1,'result':result,'before':gate,'after':after});continue")
text=text.replace("            stage={k:[] for k in row['raw_seconds']}", "            stage={k:[] for k in row['raw_seconds']};stage_telemetry=[]")
text=text.replace("                while not admission()['admitted']:time.sleep(15)", "                while not (gate:=admission())['admitted']:time.sleep(15)")
text=text.replace("                if not admission()['admitted']:continue", "                after=admission()\n                if not after['admitted']:\n                    row['discarded'].append({'stage':2,'result':result,'before':gate,'after':after});continue\n                stage_telemetry.append({'before':gate,'after':after,'order':result['order']})")
text=text.replace("row['stage2']={'raw_seconds':stage,'interval'", "row['stage2']={'raw_seconds':stage,'telemetry':stage_telemetry,'interval'")
assert text.count("row['discarded'].append")==2
assert text.count('in done:continue')==1
assert text.count("if previous:record['rows']")==1
assert "time.sleep(10);burst=time.monotonic()" in text
executed=q.ART/'wasm-executed-controller.py'
executed.write_text(text)
controller['executed_controller_sha256']=q.m.sha(executed)
exec(compile(text,str(Path(__file__)), 'exec'),dict(q.__dict__,previous=previous,done=done,controller=controller))
assert q.m.sha(__file__)==controller['sha256'] and q.m.sha(original)==controller['original_sha256']
