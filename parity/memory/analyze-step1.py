#!/usr/bin/python3
from pathlib import Path
import json,re
s=Path(__file__).parent
rows=[]; reference=None
for arm in ['reuse','clear-mesh','cpp13','clear-call-diagnostic']:
 d=s/'step1'/arm
 doc=json.loads((d/'result'/'jungle-scoreboard-214e85c568a64e8a-entity.json').read_text())
 stable={k:v for k,v in doc.items() if k!='instrumented_processing_ns'}
 if reference is None:reference=stable
 assert stable==reference,arm
 events=[json.loads(l.split('HARNESS_DIAG ',1)[1]) for l in (d/'stderr.log').read_text().splitlines() if 'HARNESS_DIAG ' in l]
 tm=(d/'time.txt').read_text();peak=int(re.search(r'Maximum resident set size \(kbytes\): (\d+)',tm)[1])
 rust=arm!='cpp13'
 rows.append({'arm':arm,'peak_rss_kib':peak,'peak_rss_gib':peak/2**20,'peak_tracked_bytes':max(e['detail'].get('usage_bytes',e['detail'].get('cpp_peak',0)) for e in events),'max_retained_on_return':max((e['detail']['retained'][0] if rust else e['detail']['cpp_retained']) for e in events),'boundary':[e['detail'] for e in events if e['stage']=='mesh_boundary'],'exact':True,'algorithm_calls':sum(e['stage'] in ['scale_raw','simplify','cache','overdraw'] for e in events)})
summary={'rows':rows,'clear_mesh_saved_gib':(rows[0]['peak_rss_kib']-rows[1]['peak_rss_kib'])/2**20,'clear_call_saved_gib':(rows[0]['peak_rss_kib']-rows[3]['peak_rss_kib'])/2**20}
summary['verdict']='no library change needed' if summary['clear_mesh_saved_gib']<.5 else 'continue to registered check'
(s/'STEP1.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
