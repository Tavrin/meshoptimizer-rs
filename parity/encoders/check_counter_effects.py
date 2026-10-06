#!/usr/bin/env python3
"""Assert registered codegen effects, independently of elapsed qualification."""
import run as lane
import json
A=lane.ART
b=json.loads((A/'counters-before-fair.json').read_text());a=json.loads((A/'counters-after.json').read_text())
base={(r['case'],r['into']):r['per_call'] for r in b['rows'] if r['backend']=='rust-before-fair'}
rows=[]
for r in a['rows']:
 if r['backend']!='rust-after':continue
 old=base[r['case'],r['into']];ratios={k:r['per_call'][k]/old[k] for k in old};case=r['case'];ins=ratios['instructions:u'];br=ratios['branches:u']
 if case=='vertex-v1-streaming-s4':assert ins<=1.02
 elif case.startswith('vertex-'):assert ins<=.8
 elif case.startswith('index-'):assert ins<=.85
 elif case.startswith('sequence-'):assert ins<=.9
 elif case.startswith('oct-'):assert ins<=.7
 elif case.startswith('exp-'):assert ins<=.8
 elif case.startswith('quat-'):assert br<=.7
 rows.append({'case':case,'into':r['into'],'ratios':ratios})
lane.save('counter-effects',{'rows':rows,'baseline_binary':b['binary']['rust-before-fair'],'after_binary':a['binary']['rust-after'],'passed':True,'timing_qualification':False})
print('registered counter effects PASS',len(rows),flush=True)
