"""Independently inventory width-aware declarations in generated rustdoc JSON."""
import json,os
from pathlib import Path
doc=Path(os.environ['CARGO_TARGET_DIR'])/'doc/meshoptimizer_rs.json'
d=json.loads(doc.read_text());index=d['index']
modules={str(i) for v in index.values() if v['crate_id']==0 and 'module' in v['inner'] for i in v['inner']['module']['items']}
functions=[v for v in index.values() if v['crate_id']==0 and (v.get('name') or '').endswith('_generic') and 'function' in v['inner']]
free=sorted(v['name'] for v in functions if str(v['id']) in modules)
owners={str(i):v['inner']['impl']['for']['resolved_path']['path'] for v in index.values() if v['crate_id']==0 and 'impl' in v['inner'] and 'resolved_path' in v['inner']['impl']['for'] for i in v['inner']['impl']['items']}
methods=sorted(owners[str(v['id'])]+'::'+v['name'] for v in functions if str(v['id']) not in modules)
aliases=sorted(v['inner']['use']['name'] for v in index.values() if v['crate_id']==0 and 'use' in v['inner'] and v['inner']['use']['name'].endswith('_generic') and v['inner']['use']['name']!=index[str(v['inner']['use']['id'])]['name'])
assert (len(free),len(aliases),len(methods))==(135,2,6),(len(free),aliases,methods)
Path('parity/0217-u16c/api-rustdoc.json').write_text(json.dumps({'free':free,'aliases':aliases,'methods':methods,'entrypoints':len(free)+len(aliases)+len(methods)},indent=2)+'\n')
print(len(free)+len(aliases)+len(methods),'width-aware entry points independently counted in rustdoc')
