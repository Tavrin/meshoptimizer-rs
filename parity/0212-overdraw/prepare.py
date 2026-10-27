import os
"""Prepare isolated attribution arms and release baseline; no builds here."""
from pathlib import Path
import hashlib, io, json, shutil, subprocess, tarfile
ROOT=Path(__file__).resolve().parents[2]
ART=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
RES=Path(os.environ['MESHOPT_RESCREEN_SCRATCH'])
assert not ART.exists()
ART.mkdir(parents=True)
commit=subprocess.check_output(['git','rev-parse','v0.2.11^{}'],text=True).strip()
for label,rev in [('base',commit),('post39','HEAD')]:
    d=ART/label;d.mkdir()
    data=subprocess.check_output(['git','archive',rev])
    with tarfile.open(fileobj=io.BytesIO(data)) as t:t.extractall(d,filter='data')
(ART/'ENTRY.json').write_text(json.dumps({'baseline':commit,'target_dir':os.environ['CARGO_TARGET_DIR'],'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()},indent=2)+'\n')
# Attribution: A removes finite guards (diagnostic only); B fixes sequential
# stride-64 validation traversal; AB combines; C removes Work accounting.
for arm in ['A','B','AB','C']:
    d=ART/f'variant-{arm}';shutil.copytree(ART/'base'/'src',d/'src');shutil.copytree(ART/'base'/'docs',d/'docs');
    for f in ['Cargo.toml','Cargo.lock']:shutil.copy2(ART/'base'/f,d/f)
    p=d/'src/overdraw.rs';s=p.read_text()
    if 'A' in arm:
        s=s.replace('''        if !p.iter().all(|x| x.is_finite()) {
            return Err(Error::NumericalFailure);
        }
''','')
        a=s.index('fn finite(');b=s.index('\n#[derive(Default)]',a)
        s=s[:a]+'''fn finite(x: f32) -> Result<f32, Error> { Ok(x) }\n'''+s[b:]
    if 'B' in arm:
        a=s.index('    positions.for_each_counted(work, |p| {');b=s.index('    Ok(centroid)',a)
        s=s[:a]+'''    let mut visit = |p: [f32;3]| {
        if !p.iter().all(|x| x.is_finite()) { return Err(Error::NumericalFailure); }
        for j in 0..3 { centroid[j] += p[j]; }
        Ok(())
    };
    if let Some(records) = positions.scale_full_records() {
        work.scan(records, &mut visit)?;
    } else {
        positions.for_each_counted(work, &mut visit)?;
    }
'''+s[b:]
        if arm=='AB':s=s.replace('        if !p.iter().all(|x| x.is_finite()) { return Err(Error::NumericalFailure); }\n','')
    p.write_text(s)
    if 'B' in arm:
        p=d/'src/input.rs';p.write_text(p.read_text().replace('fn scale_full_records(self) -> Option<impl Iterator<Item = [f32; 3]>', 'fn scale_full_records(self) -> Option<impl ExactSizeIterator<Item = [f32; 3]>'))
    if arm=='C':
        p=d/'src/workspace.rs';s=p.read_text();a=s.index('    pub(crate) fn add(&mut self,');b=s.index('\n    }',a)+6
        s=s[:a]+'''    pub(crate) fn add(&mut self, _n: usize) -> Result<(), Error> { Ok(()) }'''+s[b:]
        a=s.index('        let count = u64::try_from(values.len())',s.index('pub(crate) fn scan_units'));b=s.index('\n    /// Precharge a fixed scan',a)
        s=s[:a]+'''        let _ = units;
        for value in values { visit(value)?; }
        Ok(())
    }
'''+s[b:];p.write_text(s)
    patch=subprocess.run(['diff','-ru',str(ART/'base'/'src'),str(d/'src')],capture_output=True,text=True)
    (ART/f'{arm}.patch').write_text(patch.stdout)
for arm in ['base','A','B','AB','C','M7','X','candidate']:
    h=ART/f'harness-{arm}';shutil.copytree(RES/'parity/compare',h,ignore=shutil.ignore_patterns('target','__pycache__'))
    p=h/'Cargo.toml';s=p.read_text();source=ROOT if arm=='candidate' else ART/('base' if arm in ['base','M7','X'] else f'variant-{arm}')
    s=s.replace('version = "=0.2.10", optional = true',f'path = "{source}", optional = true')
    p.write_text(s)
# Large immutable fixtures are shared read-only; hashes freeze exact inputs.
paths=[*sorted((RES/'inputs').glob('*.bin'))]
(ART/'input-hashes.json').write_text(json.dumps({str(p):hashlib.file_digest(p.open('rb'),'sha256').hexdigest() for p in paths},indent=2)+'\n')
