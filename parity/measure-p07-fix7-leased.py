#!/usr/bin/env python3
"""Checkpoint one owner-authorized queued native burst, without repeating rows."""
import inspect
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
path = q.ART / 'performance.json'
previous = json.loads(path.read_text()) if path.exists() else None
assert not previous or not previous.get('complete', False)
if previous:
    assert previous['sources'] == q.sources()
    assert previous['binaries'] == {p.name: q.m.sha(p) for p in q.BIN.iterdir()}
done = {(r['api'], r['case']) for r in previous['rows']} if previous else set()
state = {}
began = time.monotonic()
scope = sys.argv[1] if len(sys.argv) > 1 else 'touched'
assert scope in ['touched', 'full', 'fix7']
families = ['vertex', 'view-none', 'view-filtered', 'oct', 'quat', 'exp', 'meshlet', 'meshlet-raw'] if scope == 'fix7' else ['vertex', 'view-none', 'view-filtered', 'oct', 'quat', 'meshlet', 'meshlet-raw']
cases = [(name, b) for name, b in q.corpus() if scope == 'full' or q.family(name) in families]


from p07_lease import admission


class BurstEnd(Exception):
    pass


def check_budget():
    if time.monotonic() - began >= 690:
        q.save(path, state['record'])
        raise BurstEnd()


controller = {'source': 'parity/measure-p07-fix7-leased.py', 'sha256': q.m.sha(__file__),
              'original_source': 'parity/simd/qualify.py', 'original_sha256': q.m.sha(q.ROOT / 'parity/simd/qualify.py'),
              'resumed_rows': len(done), 'scope': scope, 'policy': 'owner-visible-heavy-4GB-2026-10-05', 'control_binaries':{p.name:q.m.sha(p) for p in (q.ART/'control-bin').iterdir()},
              'pair_boundary_budget_seconds': 690,'admission_sha256':q.m.sha(q.ROOT/'parity/p07_lease.py')}
q.limit = lambda name: 1.50
source = inspect.getsource(q.bench)
source = source.replace("'sse2':Rust('sse2')", "'sse2':Rust('sse2'),'old':m.Driver(ART/'control-bin/rust-simd')")
source = source.replace("row['time_ratio']=", "row['paired_new_old_interval']=interval([x/y for x,y in zip(row.get('stage2',row)['raw_seconds']['rust'],row.get('stage2',row)['raw_seconds']['old'])]);row['paired_new_old_ratio']=statistics.median([x/y for x,y in zip(row['raw_seconds']['rust'],row['raw_seconds']['old'])]);row['time_ratio']=")
source = source.replace("if path.exists() and not diagnostic:raise ValueError('final matrix already exists; do not repeat')", "assert not previous or not previous.get('complete', False)")
source = source.replace("    for api in ['allocating','caller-buffer']:",
    "    if previous:record['rows']=previous['rows'];raw=record['rows'];record['admissions']=previous['admissions']\n"
    "    record['lease_bursts']=(previous.get('lease_bursts',[]) if previous else [])\n"
    "    record['controller_segments']=(previous.get('controller_segments',[]) if previous else [])+[controller]\n"
    "    record['scope_case_names']=[name for name,_ in cases]\n"
    "    state['record']=record;state['drivers']=ds\n"
    "    for api in ['allocating','caller-buffer']:")
source = source.replace("        for name,b in cases:", "        for name,b in cases:\n            if (api,name) in done:continue\n            check_budget()")
source = source.replace("            while not gate['admitted']:", "            while not gate['admitted']:\n                check_budget()")
source = source.replace("            probe=bytearray(b)", "            state['drivers']=ds\n            probe=bytearray(b)")
source = re.sub(r"(?m)^([ \t]+)gate=admission\(\)", r"\1check_budget()\n\1gate=admission()", source)
# Restore any admitted unfinished row, including its fixed calibration and
# fresh stage-2 samples. Resumes never repeat an accepted pair.
source = source.replace("            while len(row['raw_seconds']['rust'])<", "            if previous and previous.get('current_row',{}).get('case')==name and previous['current_row']['api']==api:\n                row=previous['current_row'];struct.pack_into('<I',probe,36,row['iterations'])\n            record['current_row']=row\n            while len(row['raw_seconds']['rust'])<")
source = source.replace("stage={k:[] for k in active};stage_telemetry=[]", "pending=row.pop('stage2_partial',{'raw_seconds':{k:[] for k in active},'telemetry':[]});stage=pending['raw_seconds'];stage_telemetry=pending['telemetry'];row['stage2_partial']=pending")
source = source.replace("                row['stage2']={'raw_seconds':stage", "                row.pop('stage2_partial',None)\n                row['stage2']={'raw_seconds':stage")
source = source.replace("            raw.append(row);save(path,record)", "            record.pop('current_row',None);raw.append(row);save(path,record)")
source = source.replace("record['complete']=True;save(path,record)", "record['scope_complete']=True;record['complete']=len(raw)==276;save(path,record)")
if scope == 'fix7':
    source = source.replace("if ci[1]<=limit(name) or ci[0]>limit(name):break", "if settled(row, name):break")
    source = source.replace("if not diagnostic and row['interval'][0]<=limit(name)<row['interval'][1]:", "if not diagnostic and not settled(row, name):")
    source = source.replace("record['scope_complete']=True;record['complete']=len(raw)==276", "record['scope_complete']=True;record['complete']=len(raw)==len(cases)*2")


def settled(row, name):
    raw = row['raw_seconds']
    ci = q.interval([x/y for x,y in zip(raw['rust'],raw['simd'])])
    return ci[1] <= 1.50 or ci[0] > 1.50


executed = q.ART / ('native-leased-executed-' + str(os.getpid()) + '.py')
executed.write_text(source)
controller['executed_controller_path'] = executed.name
controller['executed_controller_sha256'] = q.m.sha(executed)
namespace = dict(q.__dict__, previous=previous, done=done, controller=controller, state=state,
                 admission=admission, check_budget=check_budget, settled=settled)
exec(compile(source, str(executed), 'exec'), namespace)
try:
    namespace['bench'](cases)
except BurstEnd:
    print('pair-boundary checkpoint: release and requeue', flush=True)
finally:
    for driver in state.get('drivers', {}).values():
        if driver.p.poll() is None:
            driver.close()
    record = state['record']
    record.setdefault('lease_bursts', []).append({'controller': controller, 'holder': admission(),
        'wall_seconds': time.monotonic()-began, 'completed_rows': len(record['rows'])})
    q.save(path, record)
assert time.monotonic()-began < 840
assert q.m.sha(__file__) == controller['sha256']
