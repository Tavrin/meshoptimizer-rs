#!/usr/bin/env python3
"""Apply the frozen registered protocol rules without filtering cases or rerunning."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import json, math, pathlib, statistics
ART = pathlib.Path(_public_path('<scratch>/meshopt-decode'))
def gm(xs): return math.exp(statistics.mean(map(math.log,xs)))
native=json.loads((ART/'native-timing.json').read_text())
assert len(native)==30 and all(r.get('completed') for r in native)
summary={}
for release in ['AA','M7','old']:
    for op in sorted({r['operation']for r in native if r['release']==release}):
        rs=[r for r in native if r['release']==release and r['operation']==op]
        summary[release+'/'+op]=dict(gm=gm([r['gm']for r in rs]), cases=len(rs), worst=max(r['gm']for r in rs))
floor=max(abs(v['gm']-1)for k,v in summary.items()if k.startswith('AA/'))
old=summary['old/vertex-decode']['gm']<=1.02+floor
for k,v in summary.items():
    if k.startswith('M7/'):
        v['verdict']='PASS'if v['gm']<=1 and old else'PARTIAL'
summary['floor']=floor;summary['vertex_bar']=1.02+floor;summary['vertex_verdict']='PASS'if old else'PARTIAL'
browser=json.loads((ART/'browser/data/chromium-timing.json').read_text())
assert len(browser['rows'])==42
comparisons=[('after-scalar','upstream-scalar'),('after-scalar','before-scalar'),('after-simd','before-simd')]
brows={}
for a,b in comparisons:
    rs=[]
    for row in browser['rows']:
        aa=row.get('samples',{}).get(a,[]);bb=row.get('samples',{}).get(b,[])
        item=dict(id=row['id'], mode=row['mode'], bytes=row.get('bytes',{}))
        valid=(len(aa)==len(bb)==15 and all(v>0 for v in aa+bb)
               and not row.get('excluded') and all(v=='match' for v in row['bytes'].values()))
        if valid:
            logs=[math.log(x/y)for x,y in zip(aa,bb)]
            mean=statistics.mean(logs);half=2.1447867*statistics.stdev(logs)/math.sqrt(15)
            item.update(gm=math.exp(mean),upper95=math.exp(mean+half))
        else:item.update(gm=None,upper95=None)
        rs.append(item)
    positive=[r['gm']for r in rs if r['gm'] is not None]
    valid=all(r['gm'] is not None for r in rs)
    value=gm([r['gm']for r in rs])if valid else None
    bar=1.02+floor if a=='after-simd'else 1.0
    verdict='PASS'if(valid and value<=bar and all(r['upper95']<=1.10 for r in rs))else'PARTIAL'
    brows[a+'/'+b]=dict(gm=value, positive_case_gm=gm(positive)if positive else None, positive_cases=len(positive),worst_upper95=max((r['upper95']for r in rs if r['upper95']),default=None),
                       unqualified=sum(r['gm']is None for r in rs), bar=bar, verdict=verdict,
                       modes={m:gm([r['gm']for r in rs if r['mode']==m])for m in ['ATTRIBUTES','TRIANGLES']}if valid else{}, cases=rs)
result=dict(native=summary,browser=brows,timer_resolution_ms=browser['timer_resolution_ms'])
(ART/'ANALYSIS.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(dict(native=summary,browser={k:{a:b for a,b in v.items()if a!='cases'}for k,v in brows.items()}),indent=2))
