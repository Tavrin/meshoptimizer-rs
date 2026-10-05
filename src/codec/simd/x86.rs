// meshoptimizer 1.3 SSSE3 byte-group algorithm, MIT, Arseny Kapoulkine.
use super::dispatch::{Baseline, Sse41, Ssse3};
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
fn load(bytes: &[u8; 16]) -> __m128i {
    // SAFETY: the borrowed initialized array supplies exactly 16 readable bytes;
    // loadu has no alignment requirement and retains no pointer.
    unsafe { _mm_loadu_si128(bytes.as_ptr().cast()) }
}
#[inline]
#[target_feature(enable = "sse2")]
fn store(bytes: &mut [u8; 16], value: __m128i) {
    // SAFETY: the exclusive array supplies exactly 16 writable bytes; storeu
    // requires no alignment, and the pointer is not retained.
    unsafe { _mm_storeu_si128(bytes.as_mut_ptr().cast(), value) }
}
#[inline]
#[target_feature(enable = "sse2")]
fn fload(v: [f32; 4]) -> __m128 {
    _mm_setr_ps(v[0], v[1], v[2], v[3])
}
#[inline]
#[target_feature(enable = "sse2")]
fn fout(v: __m128) -> [f32; 4] {
    let mut b = [0; 16];
    store(&mut b, _mm_castps_si128(v));
    core::array::from_fn(|i| {
        f32::from_bits(u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
    })
}
#[inline]
#[target_feature(enable = "sse2")]
fn trunc(v: __m128) -> [i32; 4] {
    let mut b = [0; 16];
    store(&mut b, _mm_cvttps_epi32(v));
    core::array::from_fn(|i| i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
}
#[inline]
#[target_feature(enable = "sse2")]
fn splat(v: f32) -> __m128 {
    _mm_set1_ps(v)
}
#[inline]
#[target_feature(enable = "sse2")]
fn add(a: __m128, b: __m128) -> __m128 {
    _mm_add_ps(a, b)
}
#[inline]
#[target_feature(enable = "sse2")]
fn sub(a: __m128, b: __m128) -> __m128 {
    _mm_sub_ps(a, b)
}
#[inline]
#[target_feature(enable = "sse2")]
fn mul(a: __m128, b: __m128) -> __m128 {
    _mm_mul_ps(a, b)
}
#[inline]
#[target_feature(enable = "sse2")]
fn div(a: __m128, b: __m128) -> __m128 {
    _mm_div_ps(a, b)
}
#[inline]
#[target_feature(enable = "sse2")]
fn sqrt(a: __m128) -> __m128 {
    select(ge(a, splat(0.0)), _mm_sqrt_ps(a), splat(f32::NAN))
}
#[inline]
#[target_feature(enable = "sse2")]
fn abs(a: __m128) -> __m128 {
    _mm_andnot_ps(_mm_set1_ps(-0.0), a)
}
#[inline]
#[target_feature(enable = "sse2")]
fn neg(a: __m128) -> __m128 {
    _mm_xor_ps(_mm_set1_ps(-0.0), a)
}
#[inline]
#[target_feature(enable = "sse2")]
fn ge(a: __m128, b: __m128) -> __m128 {
    _mm_cmpge_ps(a, b)
}
#[inline]
#[target_feature(enable = "sse2")]
fn select(mask: __m128, a: __m128, b: __m128) -> __m128 {
    _mm_or_ps(_mm_and_ps(mask, a), _mm_andnot_ps(mask, b))
}
filter_kernel!("sse2");
pub(super) fn filter(
    _token: Baseline,
    kind: u8,
    data: &mut [u8],
    stride: usize,
) -> Result<(), crate::Error> {
    // SAFETY: Baseline is issued only for detected x86-64 SSE2; x86-64 makes
    // SSE2 mandatory. All slice bounds are checked by the safe kernel.
    unsafe { filter_kernel(kind, data, stride) }
}

pub(super) fn group(_token: Ssse3, data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    // SAFETY: Ssse3 is constructed only after SSSE3 AND POPCNT detection (or
    // compile-time features in no_std); the kernel's complete input is typed.
    unsafe { group_kernel(data, out, bits) }
}
#[target_feature(enable = "ssse3,popcnt")]
fn group_kernel(data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    if bits == 0 {
        out.fill(0);
        return 0;
    }
    if bits == 8 {
        out.copy_from_slice(&data[..16]);
        return 16;
    }
    let (rep, even, odd, escape) = match bits {
        1 => (
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            [0, 1, 64, 0, 16, 0, 4, 0, 0, 1, 64, 0, 16, 0, 4, 0],
            [128, 0, 32, 0, 8, 0, 2, 0, 128, 0, 32, 0, 8, 0, 2, 0],
            1,
        ),
        2 => (
            [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3],
            [4, 0, 64, 0, 4, 0, 64, 0, 4, 0, 64, 0, 4, 0, 64, 0],
            [16, 0, 0, 1, 16, 0, 0, 1, 16, 0, 0, 1, 16, 0, 0, 1],
            3,
        ),
        4 => (
            [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7],
            [16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0],
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            15,
        ),
        _ => unreachable!("validated byte-group selector"),
    };
    let input = load(data.first_chunk::<16>().unwrap());
    let words = _mm_shuffle_epi8(input, load(&rep));
    let fields = _mm_or_si128(
        _mm_mulhi_epu16(words, load(&even)),
        _mm_slli_epi16::<8>(_mm_mulhi_epu16(words, load(&odd))),
    );
    let sent = _mm_set1_epi8(escape);
    let sel = _mm_and_si128(fields, sent);
    let mask = _mm_cmpeq_epi8(sel, sent);
    let m = _mm_movemask_epi8(mask) as usize;
    let lo = m & 255;
    let hi = m >> 8;
    let count = lo.count_ones() as u8;
    let mut shuf = super::MASKS[lo];
    for i in 0..8 {
        shuf[8 + i] = super::MASKS[hi][i] + count;
    }
    let skip = (bits * 2) as usize;
    let rest = load(data[skip..].first_chunk::<16>().unwrap());
    let result = _mm_or_si128(
        _mm_shuffle_epi8(rest, load(&shuf)),
        _mm_andnot_si128(mask, sel),
    );
    store(out, result);
    skip + m.count_ones() as usize
}

pub(super) fn meshlet(_token: Sse41, data: &[u8; 16], code: u8, last: u32) -> ([u32; 4], usize) {
    // SAFETY: Sse41 is issued only after SSSE3, POPCNT and SSE4.1 are detected;
    // the kernel uses SSSE3/SSE4.1 on a complete initialized byte array.
    unsafe { meshlet_kernel(data, code, last) }
}
#[target_feature(enable = "ssse3,sse4.1")]
fn meshlet_kernel(data: &[u8; 16], code: u8, last: u32) -> ([u32; 4], usize) {
    let (mask, used) = super::meshlet_mask(code);
    let v = _mm_shuffle_epi8(load(data), load(&mask));
    let d = _mm_xor_si128(
        _mm_srli_epi32::<1>(v),
        _mm_sub_epi32(_mm_setzero_si128(), _mm_and_si128(v, _mm_set1_epi32(1))),
    );
    let mut r = _mm_add_epi32(d, _mm_set1_epi32(1));
    r = _mm_add_epi32(r, _mm_slli_si128::<4>(r));
    r = _mm_add_epi32(r, _mm_slli_si128::<8>(r));
    r = _mm_add_epi32(r, _mm_set1_epi32(last as i32));
    let mut out = [0; 16];
    store(&mut out, r);
    (
        core::array::from_fn(|i| u32::from_le_bytes(out[i * 4..i * 4 + 4].try_into().unwrap())),
        used,
    )
}

pub(super) fn deltas8(
    _token: Baseline,
    buffer: &[u8],
    target: &mut [u8],
    count: usize,
    stride: usize,
    last: &[u8],
) {
    // SAFETY: Baseline proves SSE2. The kernel slices only validated component
    // planes, stages short vectors, and scatters via checked destination slices.
    unsafe { deltas8_kernel(buffer, target, count, stride, last) }
}
#[target_feature(enable = "sse2")]
fn deltas8_kernel(buffer: &[u8], target: &mut [u8], count: usize, stride: usize, last: &[u8]) {
    let mut previous = [last[0], last[1], last[2], last[3]];
    for start in (0..count).step_by(16) {
        let n = (count - start).min(16);
        let mut planes = [_mm_setzero_si128(); 4];
        for c in 0..4 {
            let mut input = [0; 16];
            input[..n].copy_from_slice(&buffer[c * count + start..c * count + start + n]);
            let v = load(&input);
            let mut r = _mm_xor_si128(
                _mm_and_si128(_mm_srli_epi16::<1>(v), _mm_set1_epi8(127)),
                _mm_sub_epi8(_mm_setzero_si128(), _mm_and_si128(v, _mm_set1_epi8(1))),
            );
            r = _mm_add_epi8(r, _mm_slli_si128::<1>(r));
            r = _mm_add_epi8(r, _mm_slli_si128::<2>(r));
            r = _mm_add_epi8(r, _mm_slli_si128::<4>(r));
            r = _mm_add_epi8(r, _mm_slli_si128::<8>(r));
            r = _mm_add_epi8(r, _mm_set1_epi8(previous[c] as i8));
            let mut bytes = [0; 16];
            store(&mut bytes, r);
            previous[c] = bytes[n - 1];
            planes[c] = r;
        }
        let ab0 = _mm_unpacklo_epi8(planes[0], planes[1]);
        let ab1 = _mm_unpackhi_epi8(planes[0], planes[1]);
        let cd0 = _mm_unpacklo_epi8(planes[2], planes[3]);
        let cd1 = _mm_unpackhi_epi8(planes[2], planes[3]);
        let packed = [
            _mm_unpacklo_epi16(ab0, cd0),
            _mm_unpackhi_epi16(ab0, cd0),
            _mm_unpacklo_epi16(ab1, cd1),
            _mm_unpackhi_epi16(ab1, cd1),
        ];
        let mut bytes = [0; 64];
        for (chunk, v) in bytes.as_chunks_mut::<16>().0.iter_mut().zip(packed) {
            store(chunk, v);
        }
        for i in 0..n {
            target[(start + i) * stride..(start + i) * stride + 4]
                .copy_from_slice(&bytes[i * 4..i * 4 + 4]);
        }
    }
}

#[cfg(feature = "parity-internals")]
pub(super) fn sqrt4(_token: Baseline, values: [f32; 4]) -> [f32; 4] {
    // SAFETY: Baseline proves this backend's ISA; the kernel uses only values
    // and the already audited fixed-array store helper.
    unsafe { sqrt4_kernel(values) }
}
#[cfg(feature = "parity-internals")]
#[target_feature(enable = "sse2")]
fn sqrt4_kernel(values: [f32; 4]) -> [f32; 4] {
    fout(sqrt(fload(values)))
}

#[target_feature(enable = "sse2")]
fn exp_kernel(data: &mut [u8]) {
    let (chunks, tail) = data.as_chunks_mut::<16>();
    for chunk in chunks {
        let v = load(chunk);
        let m = _mm_cvtepi32_ps(_mm_srai_epi32::<8>(_mm_slli_epi32::<8>(v)));
        let e = _mm_slli_epi32::<23>(_mm_add_epi32(_mm_srai_epi32::<24>(v), _mm_set1_epi32(127)));
        store(chunk, _mm_castps_si128(_mm_mul_ps(_mm_castsi128_ps(e), m)));
    }
    crate::codec::filter::scalar_exp(tail);
}

#[target_feature(enable = "sse2")]
fn color_kernel<const W: usize, const N: usize>(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<N>();
    for block in blocks {
        let (y, co, cg, alpha) = if W == 4 {
            let v = load(block.first_chunk::<16>().unwrap());
            (
                _mm_and_si128(v, _mm_set1_epi32(255)),
                _mm_srai_epi32::<24>(_mm_slli_epi32::<16>(v)),
                _mm_srai_epi32::<24>(_mm_slli_epi32::<8>(v)),
                _mm_srli_epi32::<24>(v),
            )
        } else {
            let (lo, hi) = {
                let a = load(block[..16].first_chunk().unwrap());
                let b = load(block[16..].first_chunk().unwrap());
                (
                    _mm_castps_si128(_mm_shuffle_ps::<0x88>(
                        _mm_castsi128_ps(a),
                        _mm_castsi128_ps(b),
                    )),
                    _mm_castps_si128(_mm_shuffle_ps::<0xdd>(
                        _mm_castsi128_ps(a),
                        _mm_castsi128_ps(b),
                    )),
                )
            };
            (
                _mm_and_si128(lo, _mm_set1_epi32(65535)),
                _mm_srai_epi32::<16>(lo),
                _mm_srai_epi32::<16>(_mm_slli_epi32::<16>(hi)),
                _mm_srli_epi32::<16>(hi),
            )
        };
        let mut scale = alpha;
        scale = _mm_or_si128(scale, _mm_srai_epi32::<1>(scale));
        scale = _mm_or_si128(scale, _mm_srai_epi32::<2>(scale));
        scale = _mm_or_si128(scale, _mm_srai_epi32::<4>(scale));
        scale = _mm_or_si128(scale, _mm_srai_epi32::<8>(scale));
        let a = _mm_or_si128(
            _mm_and_si128(_mm_slli_epi32::<1>(alpha), scale),
            _mm_and_si128(alpha, _mm_set1_epi32(1)),
        );
        let ss = div(
            splat(if W == 4 { 255.0 } else { 65535.0 }),
            _mm_cvtepi32_ps(scale),
        );
        let channels = [
            _mm_sub_epi32(_mm_add_epi32(y, co), cg),
            _mm_add_epi32(y, cg),
            _mm_sub_epi32(_mm_sub_epi32(y, co), cg),
            a,
        ];
        let f = channels.map(|v| add(mul(_mm_cvtepi32_ps(v), ss), splat(0.5)));
        let mut mask = _mm_castsi128_ps(_mm_set1_epi32(-1));
        for v in f {
            mask = _mm_and_ps(
                mask,
                _mm_and_ps(
                    _mm_cmpge_ps(v, splat(-2147483648.0)),
                    _mm_cmplt_ps(v, splat(2147483648.0)),
                ),
            );
        }
        let valid = _mm_movemask_ps(mask) == 15;
        if !valid {
            crate::codec::filter::scalar_color(block, W)?;
            continue;
        }
        let [r, g, b, a] = f.map(|v| _mm_cvttps_epi32(v));
        if W == 4 {
            let value = _mm_or_si128(
                _mm_or_si128(
                    _mm_and_si128(r, _mm_set1_epi32(255)),
                    _mm_slli_epi32::<8>(_mm_and_si128(g, _mm_set1_epi32(255))),
                ),
                _mm_or_si128(
                    _mm_slli_epi32::<16>(_mm_and_si128(b, _mm_set1_epi32(255))),
                    _mm_slli_epi32::<24>(a),
                ),
            );
            store(block.first_chunk_mut::<16>().unwrap(), value);
        } else {
            let lo = _mm_or_si128(
                _mm_and_si128(r, _mm_set1_epi32(65535)),
                _mm_slli_epi32::<16>(g),
            );
            let hi = _mm_or_si128(
                _mm_and_si128(b, _mm_set1_epi32(65535)),
                _mm_slli_epi32::<16>(a),
            );
            store(
                block[..16].first_chunk_mut().unwrap(),
                _mm_unpacklo_epi32(lo, hi),
            );
            store(
                block[16..].first_chunk_mut().unwrap(),
                _mm_unpackhi_epi32(lo, hi),
            );
        }
    }
    crate::codec::filter::scalar_color(tail, W)
}

triangle_kernel!("ssse3,sse4.1");
#[target_feature(enable = "ssse3,sse4.1")]
fn triangle_step(state: &mut [u8; 16], extra: &[u8; 16], shuf: &[u8; 16], next: &[u8; 16]) {
    let s = _mm_blend_epi16::<7>(load(state), load(extra));
    store(
        state,
        _mm_add_epi8(_mm_shuffle_epi8(s, load(shuf)), load(next)),
    );
}
pub(super) fn triangles(
    _token: Sse41,
    source: &[u8],
    bound: usize,
    codes: &[u8],
    data: usize,
    count: usize,
    out: &mut impl FnMut(usize, u32),
) -> Option<Result<usize, crate::Error>> {
    // SAFETY: the dispatch token proves this kernel's ISA, and the kernel
    // checks each complete input window and scalar-equivalent bound before use.
    unsafe { triangles_kernel(source, bound, codes, data, count, out) }
}
