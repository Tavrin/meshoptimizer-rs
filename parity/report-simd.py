#!/usr/bin/env python3
"""Generate the final tables from the one retained native and Node matrices."""
import json, math, os, statistics,re
from pathlib import Path
art=Path(os.environ['MESHOPT_ARTIFACTS']);root=Path(__file__).resolve().parents[1]
native=json.loads((art/'performance.json').read_text());wasm=json.loads((art/'wasm-performance.json').read_text());assert native['complete'] and wasm['complete']
assessment_s4=json.loads((art/'s4-assessment.json').read_text());assert assessment_s4['complete']
gm=lambda xs:math.exp(statistics.mean([math.log(x) for x in xs]))
mean=lambda rs,key:gm([r[key] for r in rs])
def interval(values):
    logs=[math.log(x) for x in values];n=len(logs)
    critical=[2.776,2.571,2.447,2.365,2.306,2.262,2.228,2.201,2.179,2.160,2.145,2.131,2.120,2.110,2.101,2.093][n-5] if n<=20 else 2.045
    center=statistics.mean(logs);radius=critical*statistics.stdev(logs)/math.sqrt(n)
    return [math.exp(center-radius),math.exp(center+radius)]
def s3(row):
    result={}
    for ceiling in ['rust','sse2']:
        if ceiling not in row['raw_seconds']:continue
        stages={}
        for name,raw in [('stage1',row['raw_seconds']),*([('stage2',row['stage2']['raw_seconds'])] if 'stage2' in row else [])]:
            stages[name]=interval([x/y for x,y in zip(raw[ceiling],raw['scalar'])])
        failed=[ci for ci in stages.values() if ci[0]>1]
        result[ceiling]={'interval':failed[0] if failed else stages['stage1'],'pass':not failed,**stages}
    return result
summary={'native':{},'wasm':{},'s3_failures':[],'s4':{}}
lines=['# P07 individual SIMD performance','',f"Raw native evidence: `{art}/performance.json`; raw Node evidence: `{art}/wasm-performance.json`.",'','Time ratios: lower is faster. Family means use stage 1; only borderline case maxima use fresh D146 intervals. Repeated standalone filters are reported separately, outside the SIMD filter bar. See SIMD_RESULTS.md for shared-host and release limits.','', '| API | Family | Cases | Time Rust/C++ SIMD GM | Worst stage-1 | Time Rust/scalar Rust GM |', '|---|---|---:|---:|---:|---:|']
for api in ['allocating','caller-buffer']:
    rows=[r for r in native['rows'] if r['api']==api];summary['native'][api]={}
    for f in sorted(set(r['family'] for r in rows)):
        for repeated in ([False,True] if f in ['oct','quat'] else [False]):
            rs=[r for r in rows if r['family']==f and (r['case'].startswith('filter-')==repeated if f in ['oct','quat'] else True)]
            if not rs:continue
            key=f+(' (repeated)' if repeated else '');entry={'cases':len(rs),'mean':mean(rs,'time_ratio'),'worst':max(r['time_ratio'] for r in rs),'scalar_mean':mean(rs,'scalar_rust_ratio'),'case_failures':[r['case'] for r in rs if r['verdict']!='pass']};summary['native'][api][key]=entry
            lines.append(f"| {api} | {key} | {len(rs)} | {entry['mean']:.3f} | {entry['worst']:.3f} | {entry['scalar_mean']:.3f} |")
    for r in rows:
        if r['s3_applicable'] or r['case'].startswith('filter-3-'):
            for ceiling,v in s3(r).items():
                if not v['pass']:summary['s3_failures'].append({'api':api,'case':r['case'],'ceiling':ceiling,**v})
    s1=[r for r in rows if r['family'] in ['vertex','view-none','meshlet','meshlet-raw']];s2=[r for r in rows if r['family']=='view-filtered' or (r['family'] in ['oct','quat','exp','color'] and not r['case'].startswith(('filter-1-','filter-2-')))]
    summary['native'][api]['S1']={'mean':mean(s1,'time_ratio'),'pass':mean(s1,'time_ratio')<=1.1 and all(r['verdict']=='pass' for r in s1)};summary['native'][api]['S2']={'mean':mean(s2,'time_ratio'),'pass':mean(s2,'time_ratio')<=1.25 and all(r['verdict']=='pass' for r in s2)}
    s4=[r for r in rows if r['family'] in ['index','sequence']];ratios=[r['median_seconds']['rust']/r['median_seconds']['cpp-scalar'] for r in s4]
    minima={}
    for line in (root/'parity/DECODER_BAR.md').read_text().splitlines():
        cols=[c.strip() for c in line.split('|')[1:-1]]
        if len(cols)==5 and cols[0].startswith('index-'):minima[cols[0]]=float(cols[4])*1e6
    inputs={r['case']:(art/r['path']).read_bytes() for r in json.loads((art/'inputs.json').read_text())}
    failures=[]
    for r in s4:
        if r['case'] in minima:
            b=inputs[r['case']];size=int.from_bytes(b[8:12],'little')*int.from_bytes(b[12:16],'little');rate=size/r['median_seconds']['rust']
            if rate<minima[r['case']]:failures.append({'case':r['case'],'bytes_second':rate,'minimum':minima[r['case']]})
    case_failures=[r['case'] for r in assessment_s4['rows'] if r['api']==api and r['verdict']!='pass']
    summary['s4'][api]={'scalar_mean':gm(ratios),'scalar_worst':max(ratios),'frozen_minimum_failures':failures,'case_failures':case_failures,'pass':gm(ratios)<=1.25 and not case_failures and not failures}
lines+=['','## Native individual cases','', '| API | Case | Rust MB/s | C++ SIMD MB/s | Time Rust/C++ SIMD | Time Rust/scalar | Pairs | Final interval | Case bar | S3 |','|---|---|---:|---:|---:|---:|---:|---|---|---|']
inputs={r['case']:(art/r['path']).read_bytes() for r in json.loads((art/'inputs.json').read_text())}
for r in native['rows']:
    b=inputs[r['case']];op=int.from_bytes(b[4:8],'little');count=int.from_bytes(b[8:12],'little');stride=int.from_bytes(b[12:16],'little');size=count*stride
    if op in [20,21]:size+=int.from_bytes(b[24:28],'little')*(4 if op==21 else int.from_bytes(b[28:32],'little'))
    n=len(r['raw_seconds']['rust']);n=str(n)+('+30' if 'stage2' in r else '');ci=r['interval'];s3_verdict='n/a' if not (r['s3_applicable'] or r['case'].startswith('filter-3-')) else 'pass' if all(v['pass'] for v in s3(r).values()) else 'FAIL'
    lines.append(f"| {r['api']} | {r['case']} | {size/r['median_seconds']['rust']/1e6:.3f} | {size/r['median_seconds']['simd']/1e6:.3f} | {r['time_ratio']:.3f} | {r['scalar_rust_ratio']:.3f} | {n} | {ci[0]:.3f}–{ci[1]:.3f} | {r['verdict']} | {s3_verdict} |")
lines+=['','## S4 scalar C++ case intervals','', 'The original family means and medians stay fixed. Only scalar-baseline borderline cases receive additional pairs, up to twenty total; remaining borderline cases alone receive thirty fresh D146 pairs. No clear failure is retried.','', '| API | Case | Original pairs | Additional stage-1 pairs | Fresh stage-2 pairs | Final interval | S4 maximum |', '|---|---|---:|---:|---:|---|---|']
for r in assessment_s4['rows']:
    ci=r['interval'];n2=len(r['stage2']['raw_seconds']['rust']) if 'stage2' in r else 0
    lines.append(f"| {r['api']} | {r['case']} | {r['original_pairs']} | {len(r['additional_raw_seconds']['rust'])} | {n2} | {ci[0]:.3f}–{ci[1]:.3f} | {r['verdict']} |")
lines+=['','## Node shipped SIMD decoder','', '| API | Family | Cases | Time Rust/upstream SIMD GM | Worst | Time Rust/scalar GM |','|---|---|---:|---:|---:|---:|']
def family(n):
    if n.startswith('view-') or n.startswith('varied-view-'):return 'view-none' if 'view-none' in n else 'view-filtered'
    if n.startswith('index-'):return 'sequence' if n.startswith('index-3') else 'index'
    return 'vertex'
for api in ['allocating','caller-buffer']:
    rows=[r for r in wasm['rows'] if r['api']==api];summary['wasm'][api]={}
    for f in sorted(set(family(r['case']) for r in rows)):
        rs=[r for r in rows if family(r['case'])==f];entry={'mean':mean(rs,'time_ratio'),'worst':max(r['time_ratio'] for r in rs),'scalar_mean':mean(rs,'scalar_time_ratio'),'cases':len(rs)};summary['wasm'][api][f]=entry;lines.append(f"| {api} | {f} | {len(rs)} | {entry['mean']:.3f} | {entry['worst']:.3f} | {entry['scalar_mean']:.3f} |")
    summary['wasm'][api]['S5']={'mean':mean(rows,'time_ratio'),'worst':max(r['time_ratio'] for r in rows),'pass':mean(rows,'time_ratio')<=1.25 and all(r['verdict']=='pass' for r in rows)}
lines+=['','| API | Case | Time Rust/upstream SIMD | Time Rust/scalar | Pairs | Final interval | Case bar |','|---|---|---:|---:|---:|---|---|']
for r in wasm['rows']:
    ci=r['interval'];n=str(len(r['raw_seconds']['rust']))+('+30' if 'stage2' in r else '');lines.append(f"| {r['api']} | {r['case']} | {r['time_ratio']:.3f} | {r['scalar_time_ratio']:.3f} | {n} | {ci[0]:.3f}–{ci[1]:.3f} | {r['verdict']} |")
(root/'parity/SIMD_PERFORMANCE.md').write_text('\n'.join(lines)+'\n');(art/'performance-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
