#!/usr/bin/env python3
"""Resume diagnostic instruction/cycle captures under an owner-queued lease."""
import json
import sys
import time
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parent/'simd'))
import qualify as q
from p07_lease import admission

before=admission();began=time.monotonic();q.admission=admission
name=sys.argv[1]
assert name in ['native','wasm','s4']
original=q.ART/('profile-final.py' if name=='native' else 'profile-s4-final.py' if name=='s4' else 'profile-wasm-isolated-final.py')
source=original.read_text()
directory=q.ART/('profiles-final' if name=='native' else 'profiles-s4-final' if name=='s4' else 'profiles-wasm-isolated-final')
prior=json.loads((directory/'identity.json').read_text()) if (directory/'identity.json').exists() else None
if prior:
    assert prior['sources']==q.sources()
    assert prior['binaries']=={p.name:q.m.sha(p) for p in q.BIN.iterdir()}
source=source.replace('inputs=dict(q.corpus())',"if prior:identity=prior\ninputs=dict(q.corpus())") if name in ['native','s4'] else source.replace('for label in [',"if prior:identity=prior\nfor label in [",1)
if name in ['native','s4']:
    done={(r['case'],r['backend'],r['mode']) for r in prior['cases']} if prior else set()
    source=source.replace('        for mode in ["stat","record"]:', '        for mode in ["stat","record"]:\n            if (name,backend,mode) in done:continue')
else:
    done={(r['case'],r['backend']) for r in prior['cases']} if prior else set()
    source=source.replace('    name,backend=label.rsplit("--",1)', '    name,backend=label.rsplit("--",1)\n    if (name,backend) in done:continue')
executed=q.ART/('profile-'+name+'-leased-executed.py');executed.write_text(source)
exec(compile(source,str(executed),'exec'),{'__file__':str(original),'__name__':'__main__','prior':prior,'done':done})
q.save(q.ART/('profile-'+name+'-lease-context.json'),{'before':before,'after':admission(),
    'wall_seconds':time.monotonic()-began,'wrapper_sha256':q.m.sha(__file__),
    'admission_sha256':q.m.sha(q.ROOT/'parity/p07_lease.py'),'original_sha256':q.m.sha(original),
    'executed_sha256':q.m.sha(executed)})
assert time.monotonic()-began < 840
