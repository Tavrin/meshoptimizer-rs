"""Prepare a coordinate-bounded scoring kernel with original general fallback."""
from pathlib import Path
import hashlib,json,os,shutil,subprocess
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH']);d=A/'variant-H';assert not d.exists()
shutil.copytree(A/'variant-G',d)
p=d/'src/overdraw.rs';s=p.read_text()
s=s.replace(') -> Result<[f32; 3], Error> {', ') -> Result<([f32; 3], bool), Error> {',1)
s=s.replace('    let mut centroid = [0f32; 3];','    let mut centroid = [0f32; 3];\n    let mut bounded = true;',1)
a=s.index('        if !(p[0].is_finite()');b=s.index('        for j in 0..3 {',a)
s=s[:a]+'''        if !(p[0].abs() <= 1073741824.0 & p[1].abs() <= 1073741824.0 & p[2].abs() <= 1073741824.0) {
            bounded = false;
            if !(p[0].is_finite() & p[1].is_finite() & p[2].is_finite()) {
                return Err(Error::NumericalFailure);
            }
        }
'''+s[b:]
# Comparisons need explicit parentheses around each operand of Boolean &.
s=s.replace('p[0].abs() <= 1073741824.0 & p[1].abs() <= 1073741824.0 & p[2].abs() <= 1073741824.0','(p[0].abs() <= 1073741824.0) & (p[1].abs() <= 1073741824.0) & (p[2].abs() <= 1073741824.0)')
s=s.replace('    Ok(centroid)\n}', '    Ok((centroid, bounded))\n}',1)
s=s.replace("geometry: (Positions<'_>, [f32; 3]),", "geometry: (Positions<'_>, ([f32; 3], bool)),",1)
s=s.replace('let (positions, mut mesh_centroid) = geometry;', 'let (positions, (mut mesh_centroid, bounded)) = geometry;',1)
s=s.replace('''    fn score(
''','''    fn score<const BOUNDED: bool>(
''',1)
s=s.replace('        finite(area_sum)?;','        bounded_finite::<BOUNDED>(area_sum)?;',1)
s=s.replace('*x = finite(*x * inv_area)?;', '*x = bounded_finite::<BOUNDED>(*x * inv_area)?;',1)
for i in range(3):s=s.replace(f'finite(centroid[{i}] - mesh_centroid[{i}])?',f'bounded_finite::<BOUNDED>(centroid[{i}] - mesh_centroid[{i}])?',1)
s=s.replace('        finite(cv[0] * normal[0] + cv[1] * normal[1] + cv[2] * normal[2])', '        bounded_finite::<BOUNDED>(cv[0] * normal[0] + cv[1] * normal[1] + cv[2] * normal[2])',1)
a=s.index('\n#[derive(Default)]')
s=s[:a]+'''
#[inline(always)]
fn bounded_finite<const BOUNDED: bool>(value: f32) -> Result<f32, Error> {
    if BOUNDED { Ok(value) } else { finite(value) }
}
'''+s[a:]
# Extract only the geometry phase, preserving its two original work paths.
a=s.index('    // For strided storage, dispatch once');b=s.index('\n    let mut max = 1e-3f32;',a)
body=s[a:b].replace('&mut sqrt','sqrt').replace('.score(mesh_centroid, sqrt)', '.score::<BOUNDED>(mesh_centroid, sqrt)')
helper='''
fn score_clusters<const BOUNDED: bool>(
    indices: &[u32],
    geometry: (Positions<'_>, [f32;3]),
    soft: &[u32],
    data: &mut [f32],
    work: &mut Work,
    sqrt: &mut impl FnMut(f32) -> f32,
) -> Result<(), Error> {
    let (positions, mesh_centroid) = geometry;
    let triangles = indices.as_chunks::<3>().0;
    let faces = triangles.len();
    let soft_count = soft.len();
'''+body+'''
    Ok(())
}
'''
s=s[:a]+'''    if bounded {
        score_clusters::<true>(indices, (positions, mesh_centroid), &soft[..soft_count], data, work, &mut sqrt)?;
    } else {
        score_clusters::<false>(indices, (positions, mesh_centroid), &soft[..soft_count], data, work, &mut sqrt)?;
    }
'''+s[b:]
a=s.index('\nfn kernel(');s=s[:a]+helper+s[a:]
p.write_text(s)
shutil.copytree(A/'harness-G',A/'harness-H')
p=A/'harness-H/Cargo.toml';p.write_text(p.read_text().replace('variant-G','variant-H'))
(A/'H.patch').write_text(subprocess.run(['diff','-ru',str(A/'base/src'),str(d/'src')],capture_output=True,text=True).stdout)
(A/'bounded-sources.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in (d/'src').rglob('*.rs')},indent=2)+'\n')
