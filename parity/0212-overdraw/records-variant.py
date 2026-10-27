"""Prepare a safe shared-bounds triangle reader on top of predicate D."""
from pathlib import Path
import hashlib,json,os,shutil,subprocess
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
d=A/'variant-F';assert not d.exists()
shutil.copytree(A/'variant-D',d)
p=d/'src/input.rs';s=p.read_text()
a=s.index('        // Full 64-byte records let a single checked record lookup')
s=s[:a]+'''        #[inline(always)]
        fn scan_records<const BIG_ENDIAN: bool>(
            triangles: &[[u32; 3]],
            records: &[[u8; 64]],
            work: &mut crate::workspace::Work,
            mut visit: impl FnMut([[f32; 3]; 3]) -> Result<(), Error>,
        ) -> Result<(), Error> {
            work.scan(triangles.iter(), |tri| {
                let indices = tri.map(|i| i as usize);
                let max = indices[0].max(indices[1]).max(indices[2]);
                if max >= records.len() {
                    return Err(Error::IndexOutOfBounds);
                }
                visit(indices.map(|i| {
                    position_bytes::<BIG_ENDIAN>(records[i].first_chunk().unwrap())
                }))
            })
        }
'''+s[a:]
a=s.index('                    return match order {',s.index('fn scan_records'))
b=s.index('\n                }',a)
s=s[:a]+'''                    return match order {
                        ByteOrder::LittleEndian => scan_records::<false>(triangles, records, work, visit),
                        ByteOrder::BigEndian => scan_records::<true>(triangles, records, work, visit),
                    };'''+s[b:]
p.write_text(s)
shutil.copytree(A/'harness-D',A/'harness-F')
p=A/'harness-F/Cargo.toml';p.write_text(p.read_text().replace('variant-D','variant-F'))
(A/'F.patch').write_text(subprocess.run(['diff','-ru',str(A/'base/src'),str(d/'src')],capture_output=True,text=True).stdout)
(A/'records-sources.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in (d/'src').rglob('*.rs')},indent=2)+'\n')
