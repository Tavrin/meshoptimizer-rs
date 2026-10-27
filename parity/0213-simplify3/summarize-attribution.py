"""Map sampled instruction addresses through complete DWARF inline stacks."""
from pathlib import Path
import collections
import json
import os
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
OUT = ART/'attribution-valid'
records = [json.loads(s) for s in (OUT/'records.jsonl').read_text().splitlines()]
assert len(records)==36
cpp = Path(os.environ['MESHOPT_M7_SOURCE'])/'vendor/src/simplifier.cpp'
cpp_functions = []
for n,line in enumerate(cpp.read_text().splitlines(),1):
    m = re.match(r'static .*?\b(\w+)\(',line)
    if m:
        cpp_functions.append((n,m[1]))

def category(frames,arm):
    names = ' '.join(f[0] for f in frames)
    if arm=='M7':
        # GCC sometimes attaches a caller's symbol name to an inline leaf.
        # Resolve its source line as well, retaining the complete inline chain.
        for _,source in frames:
            m = re.search(r'simplifier\.cpp:(\d+)',source)
            if m:
                line = int(m[1])
                prior = [name for n,name in cpp_functions if n<=line]
                if prior:
                    names += ' '+prior[-1]
    if arm == 'base':
        if any('run_state::{closure#14}' in name for name,_ in frames):
            return 'index-compaction'
        if any('::update' in name for name,_ in frames):
            sites = [int(m[1]) for name,source in frames if '::update' in name
                     for m in [re.search(r'simplify\.rs:(\d+)',source)] if m]
            return 'loop-remap' if sites and min(sites)>=2220 else 'quadric-update'
    else:
        if 'remapIndexBuffer' in names:
            return 'index-compaction'
        if 'remapEdgeLoops' in names:
            return 'loop-remap'
        if 'updateQuadrics' in names:
            return 'quadric-update'
    if any(n in names for n in ['bounds_impl','rescalePositions','run_state::{closure#11}']):
        return 'bounds/normalization'
    for label,needles in [
        ('welding',['position_remap','buildPositionRemap','hashLookup2','PositionHasher']),
        ('classification',['classify','has_edge','hasEdge']),
        ('adjacency',['adjacency','EdgeAdjacency']),
        ('quadrics/face',['fillFaceQuadrics','quadricFromTriangle(']),
        ('quadrics/regularize',['fillVertexQuadrics']),
        ('quadrics/edge',['fillEdgeQuadrics']),
        ('quadrics/attributes',['fillAttributeQuadrics']),
        ('picker',['::pick','pickEdgeCollapses']),
        ('ranking',['::rank','rankEdgeCollapses','quadricError']),
        ('sorting',['::sort','sortEdgeCollapses']),
        ('collapse/update',['::perform','::flips','::update','performEdgeCollapses','updateQuadrics','hasTriangleFlips']),
        ('validation',['simplify::validate']),
    ]:
        if any(n in names for n in needles):
            return label
    if '::quadrics' in names:
        if '::finite' in names or 'cache_inverse_weight' in names:
            return 'quadrics/finite-inverse'
        call_lines = []
        for name,source in frames:
            if '::quadrics' in name:
                m = re.search(r'simplify\.rs:(\d+)',source)
                if m:
                    call_lines.append(int(m[1]))
        if call_lines:
            n = min(call_lines)
            if n<1710:
                return 'quadrics/face'
            if n<1738:
                return 'quadrics/regularize'
            if n<1774:
                return 'quadrics/edge'
            if n<1790:
                return 'quadrics/attributes'
            return 'quadrics/finite-inverse'
        return 'quadrics/other'
    if any(n in names for n in ['bounds_impl','rescalePositions']):
        return 'initialization/other'
    if 'run_state' in names or 'simplifyEdge' in names:
        return 'initialization/other'
    return 'other'

addresses = {}
for row in records:
    p = OUT/f"{row['case']}-{row['arm']}.annotate"
    weights = {}
    for line in p.read_text().splitlines():
        m = re.match(r'\s*(\d+)\s*:\s*([0-9a-f]+):',line)
        if m and int(m[1]):
            assert m[2] not in weights
            weights[m[2]] = int(m[1])
    assert weights
    addresses[(row['case'],row['arm'])] = weights
frames_by_arm = {}
for arm in ['base','M7']:
    unique = sorted({a for (case,which),weights in addresses.items() if which==arm for a in weights})
    command = ['addr2line','-a','-f','-i','-C','-e',str(ART/f'driver-{arm}')]
    r = subprocess.run(command,input=''.join('0x'+a+'\n' for a in unique),capture_output=True,text=True,check=True)
    (OUT/f'inline-stacks-{arm}.txt').write_text(r.stdout)
    mapped = {};address = None;pending = None
    for line in r.stdout.splitlines():
        if re.fullmatch('0x[0-9a-f]+',line):
            address = format(int(line,16),'x');mapped[address] = [];pending = None
        elif pending is None:
            pending = line
        else:
            mapped[address].append((pending,line));pending = None
    assert set(mapped)==set(unique)
    frames_by_arm[arm] = mapped
summary = collections.defaultdict(lambda:collections.defaultdict(lambda:collections.defaultdict(float)))
function_rows = []
for row in records:
    case,arm = row['case'],row['arm']
    report = (OUT/f'{case}-{arm}.report').read_text()
    assert 'Total Lost Samples: 0' in report
    assert not (OUT/f'{case}-{arm}.sampling-check').read_text().strip()
    total = int(re.search(r'Event count \(approx\.\): (\d+)',report)[1])
    # An unthrottled stream must closely cover the counted resident call.
    assert abs(total/row['instructions']-1)<0.02,(case,arm,total,row['instructions'])
    weights = addresses[(case,arm)]
    assert sum(weights.values())<=total
    for address,period in weights.items():
        frames = frames_by_arm[arm][address]
        label = category(frames,arm)
        estimate = period/total*row['instructions']
        summary[row['family']][arm][label] += estimate
        function_rows.append(dict(case=case,arm=arm,address=address,period=period,estimate=estimate,group=label,frames=frames))
    summary[row['family']][arm]['other'] += (total-sum(weights.values()))/total*row['instructions']
public = {family:{arm:dict(groups) for arm,groups in arms.items()} for family,arms in summary.items()}
(OUT/'function-estimates.json').write_text(json.dumps(function_rows,indent=2)+'\n')
(OUT/'summary.json').write_text(json.dumps(public,indent=2)+'\n')
for family,arms in sorted(public.items()):
    print(family)
    for name in sorted(set(arms['base'])|set(arms['M7']),key=lambda g:arms['base'].get(g,0)-arms['M7'].get(g,0),reverse=True):
        a,b = arms['base'].get(name,0),arms['M7'].get(name,0)
        print(f'  {name}: Rust={a/1e9:.6f}G C++={b/1e9:.6f}G gap={(a-b)/1e9:.6f}G')
