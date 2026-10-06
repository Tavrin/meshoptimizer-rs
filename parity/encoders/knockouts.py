#!/usr/bin/env python3
"""Disposable baseline copies only; diagnostic edits never enter the package."""
from pathlib import Path
import shutil,sys,os,json,subprocess
sys.path.insert(0,str(Path(__file__).resolve().parent));import run as lane
R,A,T=lane.ROOT,lane.ART,lane.TARGET
assert os.environ.get('MOSS_HEAVY_ACTIVE')
for name in ['bounds','validation']:
 assert shutil.disk_usage(A).free>=25*1024**3,'build floor'
 dst=A/('knockout-'+name)
 if dst.exists():shutil.rmtree(dst)
 for f in ['Cargo.toml','Cargo.lock','LICENSE','README.md']:
  shutil.copy2(R/f,dst/f) if dst.exists() else (dst.mkdir(),shutil.copy2(R/f,dst/f))
 shutil.copytree(R/'src',dst/'src');shutil.copytree(R/'parity/codec',dst/'parity/codec',ignore=shutil.ignore_patterns('__pycache__'))
 if name=='bounds':
  p=dst/'src/codec/simd/mod.rs'
  p.write_text(p.read_text()+'''
// Disposable valid-input diagnostic only. Not package/release code.
#[inline(always)]
pub(crate) fn diag_get<T>(s: &[T], i: usize) -> &T {
    debug_assert!(i < s.len());
    // SAFETY: the diagnostic uses only previously validated valid fixtures.
    unsafe { s.get_unchecked(i) }
}
#[inline(always)]
pub(crate) fn diag_get_mut<T>(s: &mut [T], i: usize) -> &mut T {
    debug_assert!(i < s.len());
    // SAFETY: packed-group n is at most 23 under its typed 16-lane contract.
    unsafe { s.get_unchecked_mut(i) }
}
#[inline(always)]
pub(crate) fn diag_span(s: &mut [u8], start: usize, end: usize) -> &mut [u8] {
    debug_assert!(start <= end && end <= s.len());
    // SAFETY: every diagnostic fixture uses a bound-sized destination.
    unsafe { s.get_unchecked_mut(start..end) }
}
''')
  p=dst/'src/codec/vertex_encode.rs';s=p.read_text()
  s=s.replace('u32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])','u32::from_le_bytes(core::array::from_fn(|i| *super::simd::diag_get(data, at + i)))')
  s=s.replace('dst[n] = v;', '*super::simd::diag_get_mut(dst, n) = v;')
  s=s.replace('(&mut w.out[w.pos..w.pos + DECODE_LIMIT])', '(super::simd::diag_span(w.out, w.pos, w.pos + DECODE_LIMIT))')
  p.write_text(s)
  p=dst/'src/codec/index_encode.rs';s=p.read_text().replace('&mut out[*pos..*pos + 5]', 'super::simd::diag_span(out, *pos, *pos + 5)').replace('(&mut out[pos..pos + 5])', '(super::simd::diag_span(out, pos, pos + 5))');p.write_text(s)
 elif name=='validation':
  p=dst/'src/codec/encode.rs';s=p.read_text()
  # Fixed setup/accounting only; hot numerical validity checks stay enabled.
  s=s.replace('workspace.account_codec(bound)?;', 'let _ = bound;').replace('workspace.account_codec(0)?;','').replace('workspace.account_codec(bytes)?;','')
  s=s.replace('work.add(input)?;', 'let _ = input;').replace('work.add(bound)','Ok(())').replace('work.add(bound.min(capacity))?;','let _ = capacity;').replace('work.add(data.len())?;','').replace('work.add(bytes / 4)?;','')
  s=s.replace('if vertices.len() != vertex_layout(count, stride)? {','if false {')
  p.write_text(s)
  p=dst/'src/codec/filter_encode.rs';s=p.read_text().replace('valid &= (-2147483648.0..2147483648.0).contains(&f);','let _ = f;');p.write_text(s)
 lane.run(['cargo','build','--offline','--locked','--release','--features','simd','--manifest-path',dst/'parity/codec/Cargo.toml'])
 shutil.copy2(T/'release/codec-driver',A/('rust-'+name))
 lane.save('build-'+name,{'source':{str(p.relative_to(dst)):lane.sha(p) for p in dst.rglob('*.rs')},'binary':lane.sha(A/('rust-'+name)),'disposable':True,'admission':os.environ['MOSS_HEAVY_ACTIVE']})
 print('knockout built',name,flush=True)
