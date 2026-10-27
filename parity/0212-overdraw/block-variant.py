"""Batch only fully padded eight-record groups; keep exact per-vertex fallbacks."""
from pathlib import Path
import hashlib,json,os,shutil,subprocess
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH']);d=A/'variant-J';assert not d.exists()
shutil.copytree(A/'variant-I',d)
p=d/'src/input.rs';s=p.read_text().replace('Option<impl ExactSizeIterator<Item = [f32; 3]> + Clone + \'a>', 'Option<impl Iterator<Item = [f32; 3]> + Clone + \'a>',1)
a=s.index('    /// Original packed coordinates and an optional sparse source mapping.')
s=s[:a]+'''    /// Eight complete cooker records and their tail, borrowing only XYZ.
    pub(crate) fn overdraw_record_blocks(self) -> Option<(
        impl ExactSizeIterator<Item = [[f32; 3]; 8]> + 'a,
        impl ExactSizeIterator<Item = [f32; 3]> + 'a,
    )> {
        if self.1.is_some() { return None; }
        let PositionStorage::View(View {
            storage: Storage::Bytes(data, ByteOrder::LittleEndian),
            count, stride: 64, offset, ..
        }) = self.0 else { return None; };
        let records = data[offset..].get(..count.checked_mul(64)?)?.as_chunks::<64>().0;
        let (blocks, tail) = records.as_chunks::<8>();
        Some((
            blocks.iter().map(|block| core::array::from_fn(|i| {
                position_bytes::<false>(block[i].first_chunk().unwrap())
            })),
            tail.iter().map(|record| position_bytes::<false>(record.first_chunk().unwrap())),
        ))
    }
'''+s[a:];p.write_text(s)
p=d/'src/overdraw.rs';s=p.read_text();a=s.index('    let mut centroid = [0f32; 3];',s.index('fn validate('));b=s.index('\n    Ok((centroid, bounded))',a)
s=s[:a]+'''    #[inline(always)]
    fn visit(p: [f32;3], centroid: &mut [f32;3], bounded: &mut bool) -> Result<(), Error> {
        if !((p[0].abs() <= 1073741824.0) & (p[1].abs() <= 1073741824.0) & (p[2].abs() <= 1073741824.0)) {
            *bounded = false;
            if !(p[0].is_finite() & p[1].is_finite() & p[2].is_finite()) { return Err(Error::NumericalFailure); }
        }
        for j in 0..3 { centroid[j] += p[j]; }
        Ok(())
    }
    let mut centroid = [0f32;3];
    let mut bounded = true;
    if let Some((blocks, tail)) = positions.overdraw_record_blocks() {
        for block in blocks {
            let admitted = block.as_flattened().iter().fold(true, |all, &x| {
                all & ((x.to_bits() & 0x7fff_ffff) <= 1073741824.0f32.to_bits())
            });
            if admitted {
                work.scan(block.into_iter(), |p| {
                    for j in 0..3 { centroid[j] += p[j]; }
                    Ok(())
                })?;
            } else {
                work.scan(block.into_iter(), |p| visit(p, &mut centroid, &mut bounded))?;
            }
        }
        work.scan(tail, |p| visit(p, &mut centroid, &mut bounded))?;
    } else {
        positions.for_each_counted(work, |p| visit(p, &mut centroid, &mut bounded))?;
    }
'''+s[b:];p.write_text(s)
shutil.copytree(A/'harness-I',A/'harness-J')
p=A/'harness-J/Cargo.toml';p.write_text(p.read_text().replace('variant-I','variant-J'))
(A/'J.patch').write_text(subprocess.run(['diff','-ru',str(A/'base/src'),str(d/'src')],capture_output=True,text=True).stdout)
(A/'block-sources.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in (d/'src').rglob('*.rs')},indent=2)+'\n')
