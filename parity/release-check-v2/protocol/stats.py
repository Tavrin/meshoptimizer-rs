import numpy as np, math,statistics

def analyze(workloads):
 rng=np.random.default_rng(20261025);cases={};boots={};families={}
 for name,w in workloads.items():
  assert w['warmup_pairs']==1 and len(w['samples'])==12
  arm=w['arm'];base=w['base']
  assert all(p['order']==([arm,base] if i%2==0 else [base,arm]) for i,p in enumerate(w['samples']))
  x=np.array([p['ratio'] for p in w['samples']]);b=np.median(x[rng.integers(0,12,size=(10000,12))],axis=1)
  cases[name]={'median':float(np.median(x)),'interval':np.quantile(b,[.025,.975]).tolist(),'family':w['family']};boots[name]=b
 for f in sorted({c['family'] for c in cases.values()}):
  ns=[n for n,c in cases.items() if c['family']==f];gm=math.exp(statistics.mean(math.log(cases[n]['median']) for n in ns))
  b=np.exp(np.mean([np.log(boots[n]) for n in ns],axis=0));families[f]={'cases':len(ns),'gm':gm,'interval':np.quantile(b,[.025,.975]).tolist(),'regresses':gm>1.02}
 return {'cases':cases,'families':families}
