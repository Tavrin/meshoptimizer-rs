from pathlib import Path
import os,json
E=Path(os.environ['VG_EVIDENCE']);R=E/'shading'
assert json.loads((R/'selected-validation/status.json').read_text())['exit']==0
rows=json.loads((R/'selected-validation/limits.json').read_text());choice=json.loads((R/'default-and-budget-selection.json').read_text())
selected='''

Calibration of the selected default normal-error weight WEIGHT, with fresh
finite suggested budgets, normals via the dedicated field, no UV/tangent
attributes, `default_config(128)` and grid -22:

| Mesh | Source triangles | Peak accounted bytes | Counted visits | Bytes/source triangle | Visits/source triangle |
| --- | ---: | ---: | ---: | ---: | ---: |
'''.replace('WEIGHT',f"{choice['default_weight']:g}")
for r in rows:
 t=r['source_triangles'];selected+=f"| {r['mesh']} | {t:,} | {r['peak_bytes']:,} | {r['work']:,} | {r['peak_bytes']/t:,.1f} | {r['work']/t:,.1f} |\n"
selected+='''
Every selected-default finite build succeeds with at least twice its measured
accounted peak and counted work available. The helper uses fixed overhead plus
power-of-two per-triangle factors, chosen above twice the initial successful
1T observations and then validated with finite budgets. These are empirical
allowances for this corpus and input policy. Additional attributes, unused
vertices, material duplication, locks or different topology require profiling.
The UV/tangent comparison does not qualify these budgets for those channels.
'''
p=Path('docs/DAG_FORMAT.md');s=p.read_text();s+=selected;p.write_text(s)
p=Path('src/clusterlod/v2.rs');s=p.read_text();a='/// Calibration with normal-error weight zero and fresh finite workspaces,'
assert a in s
# Retain the old zero-weight table as a labeled historical baseline. Add the new
# finite observations before the shared sizing advice so both scopes are explicit.
marker='/// [`suggested_limits`] supplies an empirical starting budget for this range.'
assert s.count(marker)==1
short=f"/// Calibration of the selected default normal-error weight {choice['default_weight']:g}, with fresh finite\n/// suggested budgets, the same configuration, dedicated normals and no extra attributes:\n///\n/// | Source triangles | Peak accounted bytes | Counted visits |\n/// | ---: | ---: | ---: |\n"
for r in rows:short+=f"/// | {r['source_triangles']:,} | {r['peak_bytes']:,} | {r['work']:,} |\n"
short+='///\n/// Each suggested budget exceeds twice the selected-default observation.\n/// Additional UV/tangent channels are outside this calibration.\n///\n'
s=s.replace(marker,short+marker)
s=s.replace('/// calibration (69,451 to 871,414 triangles, `default_config(128)`, normals at','/// calibration (69,451 to 871,414 triangles, `default_config(128)`, normals at')
s=s.replace('/// weight 0.5, grid -22). It is not a worst-case bound:',f"/// simplifier weight 0.5, normal-error weight {choice['default_weight']:g}, grid -22). It is not a worst-case bound:")
p.write_text(s)
print('Selected-default finite calibration recorded.')
