"""Map retained perf source-line samples to broad loops; no performance gate."""
import collections
import json
import os
from pathlib import Path
import re

S=Path(os.environ['P4_SCRATCH'])
D=S/'profile'
OUT=Path(__file__).resolve().parent
OPS=['simplify','simplify-attributes','simplify-update','simplify-sloppy']

def phase(file,line,sym,arm):
    if arm=='base-scalar':
        for key,name in [('attribute_quadric','attribute quadrics'),('::quadrics','position quadrics'),('::adjacency','adjacency'),
            ('::pick','collapse picking'),('::rank','collapse ranking'),('::perform','collapse execution'),('::update','collapse update'),
            ('::vertex_ids','grid conversion'),('::triangle_count','grid triangle counts'),('::sloppy_triangles','triangle deduplication'),
            ('::solve_records','solve/finalize'),('::solve_state','solve/finalize'),('::bounds_lanes','bounds/normalization'),('::scale_values','bounds/normalization')]:
            if key in sym:return name
        if file=='cmp.rs' and line==1880:return 'position welding'
        if file=='simplify.rs':
            if 1440<=line<=1564:return 'position welding'
            if 1360<=line<=1415:return 'adjacency'
            if 1573<=line<=1700:return 'classification'
            if 2470<=line<=2719:return 'bounds/normalization'
            if 1979<=line<=2042:return 'collapse picking'
        if file=='simplify_extra.rs':
            if 1170<=line<=1223 or 1284<=line<=1357:return 'cell hashing'
            if 1450<=line<=1489 or 1700<=line<=1755 or 1760<=line<=1778:return 'position quadrics'
            if 824<=line<=1166:return 'solve/finalize'
        return 'other/unresolved'
    for key,name in [('hashLookup','position welding'),('classifyVertices','classification'),('updateEdgeAdjacency','adjacency'),
        ('fillAttributeQuadrics','attribute quadrics'),('quadricFromAttributes','attribute quadrics'),('computeVertexIds','grid conversion'),
        ('fillVertexCells','cell hashing'),('rescalePositions','bounds/normalization')]:
        if key in sym:return name
    if file=='simplifier.cpp':
        for lo,hi,name in [(52,130,'adjacency'),(131,241,'position welding'),(337,536,'classification'),(537,610,'bounds/normalization'),
            (611,689,'solve/finalize'),(690,737,'position quadrics'),(738,777,'collapse ranking'),(778,846,'position quadrics'),
            (847,928,'attribute quadrics'),(944,1033,'solve/finalize'),(1034,1207,'position quadrics'),(1208,1230,'attribute quadrics'),
            (1335,1416,'collapse picking'),(1417,1480,'collapse ranking'),(1527,1658,'collapse execution'),
            (1659,1695,'collapse update'),(1696,1831,'solve/finalize'),(1866,1886,'collapse update'),
            (2083,2102,'grid conversion'),(2103,2118,'grid triangle counts'),(2119,2164,'cell hashing'),
            (2165,2194,'position quadrics'),(2233,2249,'position quadrics'),(2278,2322,'triangle deduplication')]:
            if lo<=line<=hi:return name
    return 'other/unresolved'

allrows=[]
for case in ['real-214e','real-a082','real-b6fd']:
    for op in OPS:
        for arm in ['base-scalar','cpp-scalar']:
            path=D/(case+'-'+op+'-'+arm+'-lines.txt')
            for line in path.read_text().splitlines():
                m=re.match(r'\s*([\d.]+)%\s+(\d+)\s+(\d+)\s+(\S+):(\d+)\s+\[.\]\s+(.*)',line)
                if not m:continue
                pct,n,period,file,ln,sym=m.groups();ln=int(ln)
                ph=phase(file,ln,sym,arm)
                if ph in ['position quadrics','attribute quadrics']:ph='quadric construction/evaluation'
                allrows.append(dict(case=case,op=op,arm=arm,file=file,line=ln,samples=int(n),period=int(period),
                    phase=ph))
(OUT/'profile-lines.json').write_text(json.dumps(allrows,indent=2)+'\n')
text=['# Simplifier instruction attribution (before experiments)','',
'All four functions, both scalar arms, all three real meshes. Two warmed calls per profile,',
'`instructions:u`, period 200003. Source and binary bindings are retained in scratch.',
'`profile.json` records successful executions. Full reports, source-line reports and',
'`perf annotate -l` are retained with the raw recordings. Zero lost samples in every report.',
'',
'The table pools the three meshes and divides sampled periods by two, reporting estimated',
'millions of user instructions per call summed over the three meshes. These are sampling',
'attributions, affected by skid and inline debug locations, **not** exact phase counters.',
'Rows are the five largest resolved Rust loop groups; the corresponding C++ group is shown',
'even when it is faster or larger. Unresolved locations are not silently allocated to a cause.',
'Positional and attribute arithmetic are grouped because inline shared Q/G helpers',
'cannot always be separated by their leaf source line. Deltas are diagnostic, not',
'isolated savings forecasts; the exact whole-call counts make the shipping decision.',
'`profile-lines.json` preserves each reported source-line attribution for inspection.','',
'| Function | Cost | Rust est. M | C++ est. M | Rust - C++ est. M |',
'|---|---|---:|---:|---:|']
for op in OPS:
    groups={a:collections.Counter() for a in ['base-scalar','cpp-scalar']}
    for r in allrows:
        if r['op']==op:groups[r['arm']][r['phase']]+=r['period']/2e6
    for ph,val in [v for v in groups['base-scalar'].most_common() if v[0]!='other/unresolved'][:5]:
        peer=groups['cpp-scalar'][ph]
        text.append(f'| {op} | {ph} | {val:.1f} | {peer:.1f} | {val-peer:+.1f} |')
    text.append(f"| {op} | other/unresolved (excluded from ranking) | {groups['base-scalar']['other/unresolved']:.1f} | {groups['cpp-scalar']['other/unresolved']:.1f} | — |")
text+=['','Per-mesh leading source locations (five per arm, estimated M instructions/call):','',
'| Function / mesh | Rust source locations | C++ source locations |','|---|---|---|']
for op in OPS:
    for case in ['real-214e','real-a082','real-b6fd']:
        cells=[]
        for arm in ['base-scalar','cpp-scalar']:
            rows=sorted([r for r in allrows if r['op']==op and r['case']==case and r['line']>0],key=lambda r:r['period'],reverse=True)[:5]
            cells.append('; '.join(f"{r['file']}:{r['line']} {r['period']/2e6:.1f}" for r in rows))
        text.append(f"| {op} / {case} | {cells[0]} | {cells[1]} |")
text+=['','Source/disassembly interpretation and first experiments:','',
'- Welding: `simplify.rs:1527–1555`, C++ `simplifier.cpp:131–241`. Rust array equality',
'  at the candidate comparison emits three short-circuit `ucomiss` chains (the first',
'  comparison maps to `cmp.rs:1880`). Try eager independent XYZ comparisons; all reads',
'  have already completed, so error ordering and probe order need not change.',
'- Positional quadrics: `simplify.rs:719–832,1801–1977`, C++ `simplifier.cpp:704–718,1034–1145`.',
'  Rust accumulation already uses packed `movups/addps` for the first eight fields.',
'  The largest sampled locations are loads into dependent scatter destinations, not',
'  evidence of a missing four-field arithmetic backend. Keep sequential contributions.',
'- Attribute quadrics: `simplify.rs:1710–1800`, C++ `simplifier.cpp:847–928,1208–1230`.',
'  Gradient fields already receive packed additions. Checked disjoint records and',
'  indexed gradient writes remain costs; finite guards remain necessary for attributes.',
'- Adjacency/picking/classification: `simplify.rs:1360–1700,1979–2042`, C++',
'  `simplifier.cpp:52–130,337–536,1353–1450`. Bounds checks, remapped index gathers',
'  and ordered candidate predicates dominate these traversal loops. SIMD arithmetic',
'  does not remove their dependent accesses; unchecked indexing is not an experiment.',
'- Sloppy: `simplify_extra.rs:1238–1266`, C++ `simplifier.cpp:2070–2105`. The grid',
'  conversion expands each coordinate into add/mask/compare/correct operations;',
'  try the defined signed truncating cast on the existing positive bounded domain.',
'  Cell hashing (`:1178–1357`) and triangle counting (`:1268–1283`) retain all visits.',
'',
'No explicit SIMD extension is justified by these profiles: the quadric field adds',
'already pack, while traversal, comparisons and conversion offer scalar opportunities.',
'This decision makes no claim about other compilers or architectures.']
text += ['', (OUT/'PROFILE-NOTES.md').read_text().rstrip()]
(OUT/'PROFILE.md').write_text('\n'.join(text)+'\n')
print('\n'.join(text[:40]))
