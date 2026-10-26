#!/usr/bin/env python3
"""Pre-registered median-of-three resident user-instruction campaign (no clock)."""
import hashlib, json, os, statistics, subprocess, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path('/mnt/linux-extra/moss-scratch/meshopt-029-scale')
OLD=Path('/mnt/linux-extra/moss-scratch/meshopt-028-L2')
PERF='/usr/lib/linux-tools/6.17.0-22-generic/perf'
assert os.environ.get('MOSS_HEAVY_ACTIVE')
epoch=sys.argv[1]
out=ART/('instructions-'+epoch)
out.mkdir()  # retain every campaign, including rejected probes
cases=[]
for t in json.loads((ROOT/'parity/028-L2/stages.json').read_text()):
    scales=[c for c in t['calls'] if c['operation']=='simplify-scale']
    paths=list(dict.fromkeys(c['vertex_path'] for c in scales))
    for i,p in enumerate(paths):
        cases.append(dict(case=t['case'][:4]+(f'-{i}' if len(paths)>1 else ''),path=p,stride=t['stride'],iterations=4,calls=sum(c['vertex_path']==p for c in scales),expected_input_sha256=next(c['vertex_sha256'] for c in scales if c['vertex_path']==p),expected_output_sha256=next(c['output_sha256'] for c in scales if c['vertex_path']==p)))
for name in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']:
    src=OLD/'inputs'/f'{name}.bin'
    raw=src.read_bytes();n=int.from_bytes(raw[:4],'little');packed=raw[8:8+n*12]
    for stride in [12,64]:
        p=ART/f'{name}-{stride}.raw'
        data=packed if stride==12 else b''.join(packed[i:i+12]+bytes(52) for i in range(0,len(packed),12))
        if p.exists():assert p.read_bytes()==data
        else:p.write_bytes(data)
        cases.append(dict(case=f'{name}-{stride}',path=str(p),stride=stride,iterations=10000 if name=='tiny-smooth' else 4 if name.startswith('million') else 100,calls=1,source=str(src),source_sha256=hashlib.sha256(raw).hexdigest()))
if os.environ.get('SCALE_PROBE'):
    cases=[cases[0],*cases[-2:]]
rows=[]
for case in cases:
    case['input_sha256']=hashlib.sha256(Path(case['path']).read_bytes()).hexdigest()
    if 'expected_input_sha256' in case: assert case['input_sha256']==case['expected_input_sha256']
    expected=None
    for rep in range(3):
        # Fixed rotating order; all samples retained.
        arms=['base','M7','candidate'];arms=arms[rep:]+arms[:rep]
        for arm in arms:
            binary=ART/f'driver-{arm}'
            stem=out/f"{case['case']}-{rep}-{arm}"
            cr,cw=os.pipe();ar,aw=os.pipe()
            cmd=[PERF,'stat','-e','instructions:u','-x',',','-o',str(stem.with_suffix('.perf')),'--delay=-1',f'--control=fd:{cr},{aw}','--','taskset','-c','9',str(binary),case['path'],str(case['stride'])]
            with stem.with_suffix('.stderr').open('w') as err:
                proc=subprocess.Popen(cmd,pass_fds=(cr,aw),stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=err,text=True)
                ready=proc.stdout.readline().strip()
                assert ready.startswith('READY '),(cmd,ready)
                bits=int(ready.split()[1])
                if 'expected_output_sha256' in case: assert hashlib.sha256(bits.to_bytes(4,'little')).hexdigest()==case['expected_output_sha256']
                if expected is None:expected=bits
                assert bits==expected,(case,arm,bits,expected)
                os.close(cr);os.close(aw)
                with os.fdopen(cw,'w') as control,os.fdopen(ar) as ack:
                    control.write('enable\n');control.flush();assert ack.readline().strip('\x00\n\r ')=='ack'
                    proc.stdin.write(str(case['iterations'])+'\n');proc.stdin.flush();assert proc.stdout.readline().strip()=='COUNTED'
                    control.write('disable\n');control.flush();assert ack.readline().strip('\x00\n\r ')=='ack'
                proc.communicate('stop\n',timeout=10);assert proc.returncode==0
            raw=stem.with_suffix('.perf').read_text()
            fields=next(line.split(',') for line in raw.splitlines() if ',instructions:u,' in line)
            instructions=int(fields[0]);assert float(fields[4])>=99.0,fields
            row=dict(case=case['case'],arm=arm,rep=rep,instructions=instructions,iterations=case['iterations'],per_call=instructions/case['iterations'],bits=bits,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
            rows.append(row)
            (out/'raw.json').write_text(json.dumps(dict(cases=cases,rows=rows,admission=os.environ['MOSS_HEAVY_ACTIVE']),indent=2)+'\n')
    med={arm:statistics.median(r['per_call'] for r in rows if r['case']==case['case'] and r['arm']==arm) for arm in ['base','M7','candidate']}
    print(case['case'],med,'candidate/base',med['candidate']/med['base'],'candidate/M7',med['candidate']/med['M7'],flush=True)
summary=[]
for case in cases:
    med={arm:statistics.median(r['per_call'] for r in rows if r['case']==case['case'] and r['arm']==arm) for arm in ['base','M7','candidate']}
    summary.append(dict(**case,median=med,baseline_ratio=med['candidate']/med['base'],M7_ratio=med['candidate']/med['M7']))
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
