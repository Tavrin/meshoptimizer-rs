"""Render a completed instruction campaign, including wall-time disclosure."""
import json
from pathlib import Path
import sys

lane=Path(__file__).resolve().parent
campaign=sys.argv[1]
data=json.loads((lane/(campaign+'-results.json')).read_text())
lines=['# Simplifier instruction counts: '+campaign,'',
'Median of three samples per cell. Ratios are Rust/peer instructions:u.',
'Scalar Rust uses std without simd; default Rust uses std+simd. C++ scalar',
'disables explicit SIMD; default C++ enables it. All six serialized outputs',
'match exactly in each of the 32 cells. Counts include allocation and output',
'drop, use warmed workspaces and include input copies in update. Setup, input',
'parsing, attribute generation, warmup and serialization are outside counters.',
'Wall times are from the same counted loops on a shared host, disclosed only.',
'',
'| Function | Default/base real | Default/base corpus | Max case | Scalar/scalar C++ real / corpus | Default/default C++ real / corpus | Verdict |',
'|---|---:|---:|---:|---:|---:|---|']
for op,s in data['summaries'].items():
    lines.append(f"| {op} | {s['default_base']:.6f} | {s['corpus']['default_base']:.6f} | {s['max_default_base']:.6f} | {s['scalar_cpp']:.6f} / {s['corpus']['scalar_cpp']:.6f} | {s['default_cpp']:.6f} / {s['corpus']['default_cpp']:.6f} | {s['verdict']} |")
lines+=['','| Case | Function | Default/base | Scalar/base | Scalar/scalar C++ | Default/default C++ | Default instructions (base / candidate / C++) |',
'|---|---|---:|---:|---:|---:|---:|']
for c in data['cells']:
    r=c['ratios'];i=c['instructions']
    lines.append(f"| {c['case']} | {c['op']} | {r['default_base']:.6f} | {r['scalar_base']:.6f} | {r['scalar_cpp']:.6f} | {r['default_cpp']:.6f} | {i['base-default']:.0f} / {i['candidate-default']:.0f} / {i['cpp-default']:.0f} |")
lines+=['','Wall time (milliseconds/call; no timing gate):','',
'| Case | Function | Rust base default | Rust candidate default | C++ default | Rust base scalar | Rust candidate scalar | C++ scalar |',
'|---|---|---:|---:|---:|---:|---:|---:|']
for c in data['cells']:
    t=c['seconds']
    values=' | '.join(f'{t[a]*1000:.3f}' for a in ['base-default','candidate-default','cpp-default','base-scalar','candidate-scalar','cpp-scalar'])
    lines.append(f"| {c['case']} | {c['op']} | {values} |")
(lane/(campaign+'-table.md')).write_text('\n'.join(lines)+'\n')
