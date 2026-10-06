#!/usr/bin/env python3
"""One queued, checkpointed Node burst of the registered final matrix."""
import json
import os
import re
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent / 'simd'))
import qualify as q

assert os.environ.get('MOSS_GPU_LEASE') == '1'
assert os.environ.get('MOSS_GPU_LEASE_LABEL') == 'heavy:timeout'
path = q.ART / 'wasm-performance.json'
previous = json.loads(path.read_text()) if path.exists() else None
assert not previous or not previous.get('complete', False)
if previous:
    assert previous['sources'] == q.sources()
    assert previous['binaries'] == {p.name:q.m.sha(p) for p in q.BIN.iterdir()}
done = {(r['api'],r['case']) for r in previous['rows']} if previous else set()
state = {}
began = time.monotonic()


from p07_lease import admission as lease_admission


class BurstEnd(Exception):
    pass


def check_budget():
    if time.monotonic()-began >= 690:
        q.save(path,state['record'])
        raise BurstEnd()


original = q.ROOT/'parity/simd/wasm_measure.py'
controller = {'source':'parity/measure-p07-fix8-wasm-leased.py','sha256':q.m.sha(__file__),
              'original_source':'parity/simd/wasm_measure.py','original_sha256':q.m.sha(original),
              'resumed_rows':len(done),'policy':'owner-visible-heavy-4GB-2026-10-05','pair_boundary_budget_seconds':690,'admission_sha256':q.m.sha(q.ROOT/'parity/p07_lease.py')}
source = original.read_text()
source = source.replace("time.sleep(15)","check_budget();time.sleep(2)")
source = source.replace("ROOT/'parity/simd/wasm_bench.mjs'","ROOT/'parity/wasm-p07-fix7-bench.mjs'")
source = source.replace("str(REF/'js/meshopt_decoder.mjs')","str(REF/'js/meshopt_decoder.mjs'),str(ART/'control-node-bin/arithmetic-simd.wasm')")
source = source.replace("'cpp':[],'scalar':[]","'cpp':[],'scalar':[],'old':[]")
source = source.replace("row['time_ratio']=","row['paired_new_old_interval']=interval([x/y for x,y in zip(row.get('stage2',row)['raw_seconds']['rust'],row.get('stage2',row)['raw_seconds']['old'])]);row['paired_new_old_ratio']=statistics.median([x/y for x,y in zip(row['raw_seconds']['rust'],row['raw_seconds']['old'])]);row['time_ratio']=")
node_bin=q.ART/'node-bin'
node_ids={p.name:q.m.sha(p) for p in node_bin.iterdir()}
assert node_ids==__import__('json').loads((q.ART/'node-build.json').read_text())['binaries']
if previous and previous['rows']:assert previous['timing_binaries']==node_ids
controller['timing_binaries']=node_ids
source=source.replace("str(BIN/'arithmetic-","str(node_bin/'arithmetic-")
source = source.replace("if path.exists():raise ValueError('final wasm matrix already exists; do not repeat')", "assert not previous or not previous.get('complete',False)")
source = source.replace("before=sources();", "admission=lease_admission\nbefore=sources();", 1)
source = source.replace(";burst=time.monotonic()", "\nif previous:record['rows']=previous['rows'];record['admissions']=previous['admissions']\nrecord['timing_binaries']=node_ids\nrecord['lease_bursts']=(previous.get('lease_bursts',[]) if previous else [])\nrecord['controller_segments']=(previous.get('controller_segments',[]) if previous else [])+[controller]\nstate['record']=record;state['process']=proc\nburst=time.monotonic()",1)
source = source.replace("        op=int.from_bytes(b[4:8],'little')", "        if ('caller-buffer' if into else 'allocating',name) in done:continue\n        check_budget()\n        op=int.from_bytes(b[4:8],'little')")
source = source.replace("'telemetry':[],'iterations'", "'telemetry':[],'discarded':[],'iterations'")
source = source.replace("        while len(row['raw_seconds']['rust'])<20:", "        if previous and previous.get('current_row',{}).get('case')==name and previous['current_row']['api']==row['api']:\n            row=previous['current_row'];q['iterations']=row['iterations']\n        record['current_row']=row;ci=row.get('interval')\n        while len(row['raw_seconds']['rust'])<20:\n            check_budget()")
source = source.replace("            if not after['admitted']:continue", "            if not after['admitted']:\n                row['discarded'].append({'stage':1,'result':result,'before':gate,'after':after});continue")
source = source.replace("            stage={k:[] for k in row['raw_seconds']}", "            pending=row.pop('stage2_partial',{'raw_seconds':{k:[] for k in row['raw_seconds']},'telemetry':[]});stage=pending['raw_seconds'];stage_telemetry=pending['telemetry'];row['stage2_partial']=pending")
source = source.replace("            while len(stage['rust'])<30:", "            while len(stage['rust'])<30:\n                check_budget()")
source = source.replace("                while not admission()['admitted']:time.sleep(15)", "                while not (gate:=admission())['admitted']:time.sleep(15)")
source = source.replace("                if not admission()['admitted']:continue", "                after=admission()\n                if not after['admitted']:\n                    row['discarded'].append({'stage':2,'result':result,'before':gate,'after':after});continue\n                stage_telemetry.append({'before':gate,'after':after,'order':result['order']})")
source = source.replace("            row['stage2']={'raw_seconds':stage,'interval'", "            row.pop('stage2_partial',None)\n            row['stage2']={'raw_seconds':stage,'telemetry':stage_telemetry,'interval'")
source = source.replace("        record['rows'].append(row);save(path,record)", "        record.pop('current_row',None);record['rows'].append(row);save(path,record)")
# A completed stage-1 row can resume directly inside its fresh second stage.
source = source.replace("        row['interval']=ci", "        ci=row.get('interval',ci) if len(row['raw_seconds']['rust'])==20 else ci\n        row['interval']=ci",1)
scope = sys.argv[1] if len(sys.argv) > 1 else 'full'
assert scope in ['full', 'fix8']
controller['scope'] = scope
controller['node_controller_sha256']=q.m.sha(q.ROOT/'parity/wasm-p07-fix7-bench.mjs')
controller['control_binaries']={p.name:q.m.sha(p) for p in (q.ART/'control-node-bin').iterdir()}
if scope == 'fix8':
    source = source.replace("if op not in [1,2,3,7]:continue", "if op not in [1,7]:continue")
    source = source.replace("if ci[1]<=1.60 or ci[0]>1.60:break", "if all(ci[1]<=bar or ci[0]>bar for bar in [1.50]):break")
    source = source.replace("if ci[0]<=1.60<ci[1]:", "if any(ci[0]<=bar<ci[1] for bar in [1.50]):")
    source = source.replace("row['interval'][1]<=1.60","row['interval'][1]<=1.50")

executed = q.ART/('wasm-leased-executed-'+str(os.getpid())+'.py')
executed.write_text(source)
controller['executed_controller_path']=executed.name
controller['executed_controller_sha256']=q.m.sha(executed)
namespace = dict(q.__dict__,previous=previous,done=done,controller=controller,state=state,
                 lease_admission=lease_admission,check_budget=check_budget,node_bin=node_bin,node_ids=node_ids)
try:
    exec(compile(source,str(executed),'exec'),namespace)
except BurstEnd:
    print('pair-boundary checkpoint: release and requeue',flush=True)
finally:
    process=state.get('process')
    if process and process.poll() is None:
        process.stdin.close()
        assert process.wait()==0
    record=state['record']
    record.setdefault('lease_bursts',[]).append({'controller':controller,'holder':lease_admission(),
        'wall_seconds':time.monotonic()-began,'completed_rows':len(record['rows'])})
    q.save(path,record)
assert record['sources']==q.sources() and record['binaries']=={p.name:q.m.sha(p) for p in q.BIN.iterdir()}
assert time.monotonic()-began < 840
assert q.m.sha(__file__)==controller['sha256']
