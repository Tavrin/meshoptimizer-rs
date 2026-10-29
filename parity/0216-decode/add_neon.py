"""Install the audited NEON parser/reconstruction extension after baseline measurement."""
from pathlib import Path
root=Path.cwd(); w=(root/'src/codec/simd/wasm32.rs').read_text()
s=w[w.index('#[inline]\n#[target_feature(enable = "simd128")]\nfn header_group'):]
s=s.replace('simd128','neon')
a=s.index('    match stride {',s.index('pub(super) fn vertex'))
b=s.index('\n}\n#[target_feature',a)
s=s[:a]+'''    // SAFETY: Baseline proves AArch64 NEON. Every load/store uses an
    // initialized fixed array behind checked slices; the parser validates
    // complete group windows and keeps scalar channel commit order.
    unsafe { vertex_kernel::<1024, 0, D>(output, count, stride, data) }'''+s[b:]
extra='''
#[inline]
#[target_feature(enable = "neon")]
fn combine<const CHANNEL: u8>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    if CHANNEL == 0 { vaddq_u8(a,b) }
    else if CHANNEL == 1 { vreinterpretq_u8_u16(vaddq_u16(vreinterpretq_u16_u8(a),vreinterpretq_u16_u8(b))) }
    else { veorq_u8(a,b) }
}
#[inline]
#[target_feature(enable = "neon")]
fn delta_prefix<const CHANNEL: u8>(mut r: uint8x16_t, previous: uint8x16_t, rot: u32) -> uint8x16_t {
    r = if CHANNEL == 0 {
        veorq_u8(vshrq_n_u8::<1>(r),vsubq_u8(vdupq_n_u8(0),vandq_u8(r,vdupq_n_u8(1))))
    } else if CHANNEL == 1 {
        let v=vreinterpretq_u16_u8(r);
        vreinterpretq_u8_u16(veorq_u16(vshrq_n_u16::<1>(v),vsubq_u16(vdupq_n_u16(0),vandq_u16(v,vdupq_n_u16(1)))))
    } else {
        let v=vreinterpretq_u32_u8(r);
        vreinterpretq_u8_u32(vorrq_u32(vshlq_u32(v,vdupq_n_s32(rot as i32)),vshlq_u32(v,vdupq_n_s32(rot as i32-32))))
    };
    let zero=vdupq_n_u8(0);
    r=combine::<CHANNEL>(r,vextq_u8::<12>(zero,r));
    r=combine::<CHANNEL>(r,vextq_u8::<8>(zero,r));
    combine::<CHANNEL>(r,previous)
}
#[inline]
#[target_feature(enable = "neon")]
fn deltas_kernel<const CHANNEL:u8,const FIXED:usize>(buffer:&[u8],target:&mut [u8],count:usize,stride:usize,last:&[u8],rot:u32) {
    let stride=if FIXED==0 {stride} else {FIXED};
    let mut previous=vreinterpretq_u8_u32(vdupq_n_u32(u32::from_le_bytes(last[..4].try_into().unwrap())));
    let full=count & !15;
    for start in (0..full).step_by(16) {
        let planes=core::array::from_fn::<_,4,_>(|c|load(buffer[c*count+start..].first_chunk::<16>().unwrap()));
        let ab0=vzip1q_u8(planes[0],planes[1]);let cd0=vzip1q_u8(planes[2],planes[3]);
        let ab1=vzip2q_u8(planes[0],planes[1]);let cd1=vzip2q_u8(planes[2],planes[3]);
        let records=[
            vreinterpretq_u8_u16(vzip1q_u16(vreinterpretq_u16_u8(ab0),vreinterpretq_u16_u8(cd0))),
            vreinterpretq_u8_u16(vzip2q_u16(vreinterpretq_u16_u8(ab0),vreinterpretq_u16_u8(cd0))),
            vreinterpretq_u8_u16(vzip1q_u16(vreinterpretq_u16_u8(ab1),vreinterpretq_u16_u8(cd1))),
            vreinterpretq_u8_u16(vzip2q_u16(vreinterpretq_u16_u8(ab1),vreinterpretq_u16_u8(cd1))),
        ];
        for (g,record) in records.into_iter().enumerate() {
            let r=delta_prefix::<CHANNEL>(record,previous,rot);
            previous=vreinterpretq_u8_u32(vdupq_laneq_u32::<3>(vreinterpretq_u32_u8(r)));
            let mut bytes=[0;16];store(&mut bytes,r);
            let dst=&mut target[(start+g*4)*stride..];
            if stride==4 {dst[..16].copy_from_slice(&bytes);}
            else {for (i,word) in bytes.as_chunks::<4>().0.iter().enumerate() {dst[i*stride..i*stride+4].copy_from_slice(word);}}
        }
    }
    if full<count {
        let mut last=[0;16];store(&mut last,previous);
        let dst=&mut target[full*stride..];
        match CHANNEL {
            0=>crate::codec::vertex::scalar_deltas::<1,false>(buffer,dst,count,stride,&last,rot,full),
            1=>crate::codec::vertex::scalar_deltas::<2,false>(buffer,dst,count,stride,&last,rot,full),
            _=>crate::codec::vertex::scalar_deltas::<4,true>(buffer,dst,count,stride,&last,rot,full),
        }
    }
}
'''
p=root/'src/codec/simd/aarch64.rs';assert 'fn delta_prefix' not in p.read_text();p.write_text(p.read_text()+extra+s)
p=root/'src/codec/simd/mod.rs';s=p.read_text();anchor='''    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    if let Some(token) = dispatch::baseline() {
        return Some(arch::vertex(token, output, count, stride, data));
    }''';assert s.count(anchor)==1
s=s.replace(anchor,anchor.replace('all(target_arch = "wasm32", target_feature = "simd128")','any(target_arch = "aarch64", all(target_arch = "wasm32", target_feature = "simd128"))'))
p.write_text(s)
