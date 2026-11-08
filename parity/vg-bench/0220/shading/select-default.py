from pathlib import Path
import os,json,math,re
E=Path(os.environ['VG_EVIDENCE']);R=E/'shading';C=R/'campaign'
assert json.loads((C/'status.json').read_text())['exit']==0
selection=json.loads((C/'default-selection.json').read_text());weight=selection['chosen_default']
core=json.loads((C/'core.json').read_text());rows=[r for r in core if r['weight']==weight]
triangles={r['mesh']:json.loads((C/(r['mesh']+'-builds.json')).read_text())[0]['triangles'] for r in rows}
def allowance(key,minimum):return max(minimum,2**math.ceil(math.log2(max(2*r['timing']['1'][key]/triangles[r['mesh']] for r in rows))))
byte_factor=allowance('accounted_bytes',512);work_factor=allowance('work',4096)
p=Path('src/clusterlod/v2.rs');s=p.read_text();s=re.sub(r'DEFAULT_NORMAL_ERROR_WEIGHT: f32 = [0-9.]+;',f'DEFAULT_NORMAL_ERROR_WEIGHT: f32 = {weight:.2f};',s)
s=s.replace('    /// Only applies when normals are supplied. Must be finite and nonnegative.',f'    /// Defaults to {weight:g}. Only applies when normals are supplied. Must be finite and nonnegative.')
a=s.index('pub fn suggested_limits(');b=s.index('/// Owned flat DAG',a)
section=s[a:b].replace('.checked_mul(512)',f'.checked_mul({byte_factor})').replace('.checked_mul(4096)',f'.checked_mul({work_factor})')
s=s[:a]+section+s[b:]
s=s.replace('16 MiB plus 512 bytes per triangle',f'16 MiB plus {byte_factor} bytes per triangle').replace('2^24 plus 4096 visits per triangle',f'2^24 plus {work_factor} visits per triangle')
p.write_text(s)
p=Path('CHANGELOG.md');s=p.read_text();entry=f'''- Price sampled interpolated-normal loss in MDAG 2 original-source group errors,
  using a local-edge normal-chord displacement and monotone inheritance. The new
  `BuildMesh::normal_error_weight` defaults to {weight:g}; zero disables it and absent
  normals leave geometry unchanged. Builder revision 8 binds the weight in cook
  keys. Legacy builders and container layouts are unchanged.
''';assert entry not in s;s=s.replace('''- Add an opt-in sampled normal-loss displacement to MDAG 2 original-source
  group errors through `BuildMesh::normal_error_weight`, with geometric floors
  and monotone inheritance retained. Builder revision 8 binds this weight in
  cook keys; the container layout is unchanged. Validation and default selection
  remain pending; the candidate default is disabled (zero).
''','');s=s.replace('### Changed\n\n','### Changed\n\n'+entry,1);p.write_text(s)
p=Path('docs/DAG_FORMAT.md');s=p.read_text().replace('Builder revision is 5 (local source-region ownership requires a recook).','Builder revision is 8 (sampled normal-loss pricing requires a recook).')
s=s.replace('16 MiB + 512T bytes',f'16 MiB + {byte_factor}T bytes').replace('2^24 + 4096T visits',f'2^24 + {work_factor}T visits')
s+='''

## Sampled shading error

`BuildMesh::normal_error_weight` prices source normal variation discarded between
surviving vertices. For an original-source region, take the maximum sampled
angle between source and nearest replacement interpolated normals in both
surface directions. With regional maximum source/replacement triangle edge L,
its displacement proxy is `2 * weight * L * sin(angle / 2)`. This unit-normal
chord is monotone through 180 degrees and is a conservative local lever-arm
choice. Maximize it with every positional floor and inherit it through parents.
This is a sampled shading proxy, not a continuous normal, BRDF or texture bound.
Undefined interior normal interpolants contribute 180 degrees on this new private
path; the public quality-statistics API retains its strict numerical refusal.

The default weight is WEIGHT. Zero disables this term; absent normals also
leave the geometric builder unchanged. Weights must be finite and nonnegative.
The angular pass shares the existing region samples and exact nearest queries.
Repeated corners are deduplicated. Constant-normal regions, and regions whose
geometric floor already exceeds the largest possible chord displacement, keep
the positional-only path. Topology and attribute references are unchanged by
this weight; only stored errors and resulting runtime cuts change.

Set normals only through `BuildMesh::normals`; their simplifier weight remains
0.5. Pack UV and tangent components through the existing `attributes` and
`attribute_weights` fields. Packing normals there as well weights them a second
time. `KeyInputs::normal_error_weight` must match the build input when computing
a cook key. Its constructor uses the same default; builder revision 8 invalidates
prior keys without changing the MDAG 2 container or page layouts. Legacy entry
points retain their earlier semantics and byte output.

The five-mesh weight comparison, default decision, and UV/tangent recommendation
are recorded in the [sampled shading table](../parity/vg-bench/0220/shading/TABLE.md).
'''.replace('WEIGHT',f'{weight:g}')
p.write_text(s)
(R/'default-and-budget-selection.json').write_text(json.dumps(dict(default_weight=weight,byte_factor=byte_factor,work_factor=work_factor,source='twice measured unlimited 1T maxima, rounded upward to powers of two; finite validation pending',selection=selection),indent=2)+'\n')
print(weight,byte_factor,work_factor)
