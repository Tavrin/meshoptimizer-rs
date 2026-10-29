"""Report median instruction and disclosure-only wall ratios, without relaxing gates."""
import json,math,os,statistics
from pathlib import Path
art=Path(os.environ['DECODE_ART']);root=Path.cwd();rows=[]
for line in (art/'baseline-counts.jsonl').read_text().splitlines():rows.append(json.loads(line))
p=art/'candidate-counts.jsonl'
baseline=rows[:]
if p.exists():rows=[json.loads(line) for line in p.read_text().splitlines()]
selected=os.environ.get('DECODE_FINAL_ARM','candidate')
keys=sorted({(r['case'],r['op']) for r in rows});out=[]
for case,op in keys:
    arms=sorted({r['arm'] for r in rows if r['case']==case and r['op']==op})
    med={a:statistics.median(r['instructions_per_call'] for r in rows if (r['case'],r['op'],r['arm'])==(case,op,a)) for a in arms}
    wall={a:statistics.median(r['wall_seconds_per_call'] for r in rows if (r['case'],r['op'],r['arm'])==(case,op,a)) for a in arms}
    hashes={r['output_sha256'] for r in rows if (r['case'],r['op'])==(case,op)};assert len(hashes)==1,(case,op,hashes)
    out.append(dict(case=case,op=op,instructions=med,wall_seconds=wall,baseline_default_ratio=med['base']/med['cpp-default'],**(dict(candidate_base_ratio=med[selected]/med['base']) if 'candidate'in med else {})))
record=dict(rows=out,selected_arm=selected,method='instructions:u; three interleaved repetitions per arm; median; setup, encoding, serialization and warmup excluded',gate='tuning ships only at candidate/base geometric mean <=0.97 and every case <=1.01; NEON identity requires native CI')
(art/'results.json').write_text(json.dumps(record,indent=2)+'\n')
lines=['# Decoder default-build measurements','',record['method']+'. Wall times are disclosure only.','', '| Case | API | Rust base / C++ default instructions | Final / base instructions | Control / base | Final / C++ scalar | Final / C++ default | Rust base / C++ default wall |','|---|---|---:|---:|---:|---:|---:|---:|']
for r in out:
    ratio=r.get('candidate_base_ratio',float('nan'));w=r['wall_seconds'];lines.append(f"| {r['case']} | {r['op']} | {r['baseline_default_ratio']:.4f} | {ratio:.4f} | {r['instructions'].get('control',r['instructions']['base'])/r['instructions']['base']:.4f} | {r['instructions'].get(selected,r['instructions']['base'])/r['instructions']['cpp-scalar']:.4f} | {r['instructions'].get(selected,r['instructions']['base'])/r['instructions']['cpp-default']:.4f} | {w['base']/w['cpp-default']:.4f} |")
for op in ['vertex-decode','index-decode']:
    real=[r for r in out if r['case'].startswith('real-') and r['op']==op]
    g=lambda vals:math.exp(statistics.mean(math.log(v) for v in vals))
    print(op,'real baseline',g([r['baseline_default_ratio'] for r in real]),'worst',max(r.get('candidate_base_ratio',1) for r in out if r['op']==op))
(root/'parity/0216-decode/TABLE.md').write_text('\n'.join(lines)+'\n')
