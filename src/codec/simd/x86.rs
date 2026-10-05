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
#[target_feature(enable = "sse2")]
fn filter_kernel(kind: u8, data: &mut [u8], stride: usize) -> Result<(), crate::Error> {
    match kind {
        1 if stride == 4 => oct_kernel::<4, 16>(data),
        1 => oct_kernel::<8, 32>(data),
        2 => quat_kernel(data),
        3 => {
            exp_kernel(data);
            Ok(())
        }
        _ if stride == 4 => color_kernel::<4, 16>(data),
        _ => color_kernel::<8, 32>(data),
    }
}

// Four packed halfword records become four independent component vectors.
#[inline]
#[target_feature(enable = "sse2")]
fn fields16(block: &[u8; 32]) -> [__m128i; 4] {
    let a = load(block[..16].first_chunk().unwrap());
    let b = load(block[16..].first_chunk().unwrap());
    let lo = _mm_castps_si128(_mm_shuffle_ps::<0x88>(
        _mm_castsi128_ps(a),
        _mm_castsi128_ps(b),
    ));
    let hi = _mm_castps_si128(_mm_shuffle_ps::<0xdd>(
        _mm_castsi128_ps(a),
        _mm_castsi128_ps(b),
    ));
    [
        _mm_srai_epi32::<16>(_mm_slli_epi32::<16>(lo)),
        _mm_srai_epi32::<16>(lo),
        _mm_srai_epi32::<16>(_mm_slli_epi32::<16>(hi)),
        _mm_srai_epi32::<16>(hi),
    ]
}
#[inline]
#[target_feature(enable = "sse2")]
fn round_vector(value: __m128, sign: __m128) -> __m128i {
    _mm_cvttps_epi32(add(
        value,
        select(ge(sign, splat(0.0)), splat(0.5), splat(-0.5)),
    ))
}
#[inline]
#[target_feature(enable = "sse2")]
fn pair16(a: __m128i, b: __m128i) -> __m128i {
    _mm_or_si128(
        _mm_and_si128(a, _mm_set1_epi32(65535)),
        _mm_slli_epi32::<16>(b),
    )
}
#[target_feature(enable = "sse2")]
fn oct_kernel<const W: usize, const N: usize>(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<N>();
    for block in blocks {
        let raw = if W == 4 {
            let v = load(block.first_chunk::<16>().unwrap());
            [
                _mm_srai_epi32::<24>(_mm_slli_epi32::<24>(v)),
                _mm_srai_epi32::<24>(_mm_slli_epi32::<16>(v)),
                _mm_srai_epi32::<24>(_mm_slli_epi32::<8>(v)),
                _mm_srli_epi32::<24>(v),
            ]
        } else {
            fields16(block.first_chunk::<32>().unwrap())
        };
        let mut x = _mm_cvtepi32_ps(raw[0]);
        let mut y = _mm_cvtepi32_ps(raw[1]);
        let z = sub(sub(_mm_cvtepi32_ps(raw[2]), abs(x)), abs(y));
        let t = select(ge(z, splat(0.0)), splat(0.0), z);
        x = add(x, select(ge(x, splat(0.0)), t, neg(t)));
        y = add(y, select(ge(y, splat(0.0)), t, neg(t)));
        let length = _mm_sqrt_ps(add(add(mul(x, x), mul(y, y)), mul(z, z)));
        if _mm_movemask_ps(_mm_cmpeq_ps(length, splat(0.0))) != 0 {
            return Err(crate::Error::NumericalFailure);
        }
        let ss = div(splat(if W == 4 { 127.0 } else { 32767.0 }), length);
        let r = round_vector(mul(x, ss), x);
        let g = round_vector(mul(y, ss), y);
        let b = round_vector(mul(z, ss), z);
        if W == 4 {
            let packed = _mm_or_si128(
                _mm_or_si128(
                    _mm_and_si128(r, _mm_set1_epi32(255)),
                    _mm_slli_epi32::<8>(_mm_and_si128(g, _mm_set1_epi32(255))),
                ),
                _mm_or_si128(
                    _mm_slli_epi32::<16>(_mm_and_si128(b, _mm_set1_epi32(255))),
                    _mm_slli_epi32::<24>(raw[3]),
                ),
            );
            store(block.first_chunk_mut::<16>().unwrap(), packed);
        } else {
            let rg = pair16(r, g);
            let ba = pair16(b, raw[3]);
            store(
                block[..16].first_chunk_mut().unwrap(),
                _mm_unpacklo_epi32(rg, ba),
            );
            store(
                block[16..].first_chunk_mut().unwrap(),
                _mm_unpackhi_epi32(rg, ba),
            );
        }
    }
    crate::codec::filter::scalar_oct(tail, W)
}
#[target_feature(enable = "sse2")]
fn quat_kernel(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<32>();
    let scale = splat(32767.0 / crate::math::sqrt(2.0));
    for block in blocks {
        // Keep rotation selectors before the packed output overwrites them.
        let rotations = [block[6], block[14], block[22], block[30]];
        let [xi, yi, zi, ci] = fields16(block);
        let x = _mm_cvtepi32_ps(xi);
        let y = _mm_cvtepi32_ps(yi);
        let z = _mm_cvtepi32_ps(zi);
        let s = _mm_cvtepi32_ps(_mm_or_si128(ci, _mm_set1_epi32(3)));
        let ww = sub(
            sub(sub(mul(mul(s, s), splat(2.0)), mul(x, x)), mul(y, y)),
            mul(z, z),
        );
        let w = _mm_sqrt_ps(select(ge(ww, splat(0.0)), ww, splat(0.0)));
        let ss = div(scale, s);
        let xr = round_vector(mul(x, ss), x);
        let yr = round_vector(mul(y, ss), y);
        let zr = round_vector(mul(z, ss), z);
        let wr = _mm_cvttps_epi32(add(mul(w, ss), splat(0.5)));
        let xy = pair16(xr, yr);
        let zw = pair16(zr, wr);
        let mut packed = [0u8; 32];
        store(
            packed[..16].first_chunk_mut().unwrap(),
            _mm_unpacklo_epi32(xy, zw),
        );
        store(
            packed[16..].first_chunk_mut().unwrap(),
            _mm_unpackhi_epi32(xy, zw),
        );
        for ((out, value), rotation) in block
            .as_chunks_mut::<8>()
            .0
            .iter_mut()
            .zip(packed.as_chunks::<8>().0)
            .zip(rotations)
        {
            *out = u64::from_le_bytes(*value)
                .rotate_left(u32::from((rotation.wrapping_add(1)) & 3) * 16)
                .to_le_bytes();
        }
    }
    crate::codec::filter::scalar_quat(tail)
}
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

// Keep the decode configuration in memory: selecting scalar-built constants
// made LLVM synthesize shuffle masks in registers on every packed group.
const GROUP_CONFIG: [[[u8; 16]; 4]; 9] = {
    let mut table = [[[0; 16]; 4]; 9];
    let mut bits = 0;
    while bits < 9 {
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
            _ => ([0; 16], [0; 16], [0; 16], 0),
        };
        table[bits] = [[escape as u8; 16], rep, even, odd];
        bits += 1;
    }
    table
};

pub(super) fn group(_token: Ssse3, data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    // SAFETY: Ssse3 is constructed only after SSSE3 AND POPCNT detection (or
    // compile-time features in no_std); the kernel's complete input is typed.
    unsafe { group_kernel(data, out, bits) }
}
#[inline]
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
    let [sent, rep, even, odd] = &GROUP_CONFIG[bits as usize];
    let input = load(data.first_chunk::<16>().unwrap());
    let words = _mm_shuffle_epi8(input, load(rep));
    let fields = _mm_or_si128(
        _mm_mulhi_epu16(words, load(even)),
        _mm_slli_epi16::<8>(_mm_mulhi_epu16(words, load(odd))),
    );
    let sent = load(sent);
    let sel = _mm_and_si128(fields, sent);
    let mask = _mm_cmpeq_epi8(sel, sent);
    let m = _mm_movemask_epi8(mask) as usize;
    let lo = m & 255;
    let hi = m >> 8;
    let counts = _mm_sad_epu8(mask, _mm_setzero_si128());
    let upper = _mm_sub_epi8(
        load(&super::MASKS[hi]),
        _mm_shuffle_epi8(counts, _mm_setzero_si128()),
    );
    let shuf = _mm_unpacklo_epi64(load(&super::MASKS[lo]), upper);
    let skip = (bits * 2) as usize;
    let rest = load(data[skip..].first_chunk::<16>().unwrap());
    let result = _mm_or_si128(_mm_shuffle_epi8(rest, shuf), _mm_andnot_si128(mask, sel));
    store(out, result);
    skip + m.count_ones() as usize
}

pub(super) fn bytes(
    _token: Ssse3,
    data: &[u8],
    pos: usize,
    out: &mut [u8],
    bits: &[u32],
) -> Result<usize, crate::Error> {
    // SAFETY: Ssse3 proves SSSE3 and POPCNT. The kernel checks a complete
    // 24-byte window before each group and writes checked output chunks only.
    unsafe { bytes_kernel(data, pos, out, bits) }
}
#[inline]
#[target_feature(enable = "ssse3,popcnt")]
fn bytes_kernel(
    data: &[u8],
    mut pos: usize,
    out: &mut [u8],
    bits: &[u32],
) -> Result<usize, crate::Error> {
    let headers = (out.len() / 16).div_ceil(4);
    let header = data
        .get(pos..pos + headers)
        .ok_or(crate::Error::InvalidStream)?;
    pos += headers;
    let (blocks, tail) = out.as_chunks_mut::<64>();
    for (i, block) in blocks.iter_mut().enumerate() {
        let control = header[i];
        // This bound covers even four fully escaped groups. It preserves the
        // scalar lookahead rule on all paths, including zero/literal shortcuts.
        if data.len().saturating_sub(pos) >= 96 {
            if control == 0 && bits[0] == 0 {
                block.fill(0);
                continue;
            }
            if control == 255 && bits[3] == 8 {
                block.copy_from_slice(&data[pos..pos + 64]);
                pos += 64;
                continue;
            }
        }
        for (j, chunk) in block.as_chunks_mut::<16>().0.iter_mut().enumerate() {
            let window = data
                .get(pos..)
                .and_then(|v| v.first_chunk::<24>())
                .ok_or(crate::Error::InvalidStream)?;
            pos += group_kernel(window, chunk, bits[usize::from((control >> (j * 2)) & 3)]);
        }
    }
    for (j, chunk) in tail.as_chunks_mut::<16>().0.iter_mut().enumerate() {
        let window = data
            .get(pos..)
            .and_then(|v| v.first_chunk::<24>())
            .ok_or(crate::Error::InvalidStream)?;
        pos += group_kernel(
            window,
            chunk,
            bits[usize::from((header[blocks.len()] >> (j * 2)) & 3)],
        );
    }
    Ok(pos)
}

pub(super) fn meshlet(_token: Sse41, data: &[u8; 16], code: u8, last: u32) -> ([u32; 4], usize) {
    // SAFETY: Sse41 is issued only after SSSE3, POPCNT and SSE4.1 are detected;
    // the kernel uses SSSE3/SSE4.1 on a complete initialized byte array.
    unsafe { meshlet_kernel(data, code, last) }
}
#[inline]
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
pub(super) fn meshlet_vertices(
    _token: Sse41,
    source: &[u8],
    bound: usize,
    codes: &[u8],
    count: usize,
    out: &mut impl FnMut(usize, u32),
) -> Result<usize, crate::Error> {
    // SAFETY: Sse41 proves SSSE3/SSE4.1; the kernel checks every 16-byte
    // input window and emits only the scalar decoder's validated record count.
    unsafe { meshlet_vertices_kernel(source, bound, codes, count, out) }
}
#[target_feature(enable = "ssse3,sse4.1")]
fn meshlet_vertices_kernel(
    source: &[u8],
    bound: usize,
    codes: &[u8],
    count: usize,
    out: &mut impl FnMut(usize, u32),
) -> Result<usize, crate::Error> {
    let mut data = 0;
    let mut last = u32::MAX;
    for (g, &code) in codes.iter().enumerate() {
        if data > bound {
            return Err(crate::Error::InvalidStream);
        }
        let window = source
            .get(data..)
            .and_then(|v| v.first_chunk::<16>())
            .ok_or(crate::Error::InvalidStream)?;
        let (values, used) = meshlet_kernel(window, code, last);
        for (k, value) in values.into_iter().enumerate() {
            if g * 4 + k < count {
                out(g * 4 + k, value);
            }
        }
        last = values[3];
        data += used;
    }
    Ok(data)
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
#[inline]
#[target_feature(enable = "sse2")]
fn deltas8_kernel(buffer: &[u8], target: &mut [u8], count: usize, stride: usize, last: &[u8]) {
    let mut previous = [last[0], last[1], last[2], last[3]];
    for start in (0..count).step_by(16) {
        let n = (count - start).min(16);
        let mut planes = [_mm_setzero_si128(); 4];
        for c in 0..4 {
            let plane = &buffer[c * count + start..c * count + start + n];
            let mut input = [0; 16];
            let v = if let Some(full) = plane.first_chunk::<16>() {
                load(full)
            } else {
                input[..n].copy_from_slice(plane);
                load(&input)
            };
            let mut r = _mm_xor_si128(
                _mm_and_si128(_mm_srli_epi16::<1>(v), _mm_set1_epi8(127)),
                _mm_sub_epi8(_mm_setzero_si128(), _mm_and_si128(v, _mm_set1_epi8(1))),
            );
            r = _mm_add_epi8(r, _mm_slli_si128::<1>(r));
            r = _mm_add_epi8(r, _mm_slli_si128::<2>(r));
            r = _mm_add_epi8(r, _mm_slli_si128::<4>(r));
            r = _mm_add_epi8(r, _mm_slli_si128::<8>(r));
            r = _mm_add_epi8(r, _mm_set1_epi8(previous[c] as i8));
            previous[c] = if n == 16 {
                _mm_cvtsi128_si32(_mm_srli_si128::<15>(r)) as u8
            } else {
                let mut bytes = [0; 16];
                store(&mut bytes, r);
                bytes[n - 1]
            };
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
        if stride == 4 && n == 16 {
            for (chunk, v) in target[start * 4..(start + 16) * 4]
                .as_chunks_mut::<16>()
                .0
                .iter_mut()
                .zip(packed)
            {
                store(chunk, v);
            }
            continue;
        }
        if n == 16 {
            let mut records = target[start * stride..(start + 15) * stride + 4].chunks_mut(stride);
            for v in packed {
                // Keep complete records in registers instead of staging 64
                // bytes and reloading each strided four-byte destination.
                let words = [
                    _mm_cvtsi128_si32(v),
                    _mm_cvtsi128_si32(_mm_shuffle_epi32::<0x55>(v)),
                    _mm_cvtsi128_si32(_mm_shuffle_epi32::<0xaa>(v)),
                    _mm_cvtsi128_si32(_mm_shuffle_epi32::<0xff>(v)),
                ];
                for word in words {
                    records.next().unwrap()[..4].copy_from_slice(&word.to_le_bytes());
                }
            }
            continue;
        }
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

#[target_feature(enable = "ssse3,sse4.1")]
fn triangles_kernel(
    source: &[u8],
    bound: usize,
    codes: &[u8],
    mut data: usize,
    count: usize,
    out: &mut impl FnMut(usize, u32),
) -> Option<Result<usize, crate::Error>> {
    let mut state = _mm_setzero_si128();
    for i in (0..count).step_by(2) {
        if data > bound {
            return Some(Err(crate::Error::InvalidStream));
        }
        let (shuf, next, used, first_used) = super::TRIANGLE_TABLES[codes[i / 2] as usize];
        let counter = _mm_cvtsi128_si32(_mm_srli_si128::<15>(state)) as u8;
        if usize::from(counter) + usize::from(next[15]) >= 256 {
            return None;
        }
        if i + 1 < count && data + first_used > bound {
            return Some(Err(crate::Error::InvalidStream));
        }
        let Some(window) = source.get(data..).and_then(|b| b.first_chunk::<16>()) else {
            return Some(Err(crate::Error::InvalidStream));
        };
        let merged = _mm_blend_epi16::<7>(state, load(window));
        state = _mm_add_epi8(_mm_shuffle_epi8(merged, load(&shuf)), load(&next));
        let mut bytes = [0; 16];
        store(&mut bytes, state);
        for k in 0..(count - i).min(2) {
            let a = u32::from(bytes[9 + k * 3]);
            let b = u32::from(bytes[10 + k * 3]);
            let c = u32::from(bytes[11 + k * 3]);
            out(i + k, c | (a << 8) | (b << 16) | (c << 24));
        }
        data += if i + 1 < count { used } else { first_used };
    }
    Some(Ok(data))
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

pub(super) fn vertex(
    _token: Ssse3,
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), crate::Error> {
    // SAFETY: Ssse3 proves SSSE3 and POPCNT. The kernel retains the scalar
    // parser's checked layout, group lookahead and bounded destination slices;
    // stack scratch is 1,280 bytes, and every vector load/store is array-backed.
    unsafe { vertex_kernel(output, count, stride, data) }
}
#[target_feature(enable = "ssse3,popcnt")]
fn vertex_kernel(
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), crate::Error> {
    let version = crate::codec::decode_vertex_version(data)?;
    let tail = stride + if version == 0 { 0 } else { stride / 4 };
    let padded = tail.max(if version == 0 { 32 } else { 24 });
    if data.len() < 1 + padded {
        return Err(crate::Error::InvalidStream);
    }
    let start = data.len() - tail;
    let channels = &data[start + stride..];
    let mut last = [0u8; 256];
    last[..stride].copy_from_slice(&data[start..start + stride]);
    let block_size = ((8192 / stride) & !15).min(256);
    let mut pos = 1;
    let mut deltas = [0u8; 1024];
    let mut offset = 0;
    while offset < count {
        let block = block_size.min(count - offset);
        let aligned = block.div_ceil(16) * 16;
        let controls = if version == 0 {
            &[][..]
        } else {
            let c = data
                .get(pos..pos + stride / 4)
                .ok_or(crate::Error::InvalidStream)?;
            pos += stride / 4;
            c
        };
        // Each bounded block fits 8 KiB. Write directly into its checked
        // destination slice, avoiding zeroing/copying an extra stack block.
        let block_output = &mut output[offset * stride..(offset + block) * stride];
        for k in (0..stride).step_by(4) {
            let control = if version == 0 { 0 } else { controls[k / 4] };
            for j in 0..4 {
                let ctrl = (control >> (j * 2)) & 3;
                let out = &mut deltas[j * block..j * block + aligned];
                match ctrl {
                    3 => {
                        out[..block].copy_from_slice(
                            data.get(pos..pos + block)
                                .ok_or(crate::Error::InvalidStream)?,
                        );
                        pos += block;
                    }
                    2 => out[..block].fill(0),
                    _ => {
                        pos = bytes_kernel(
                            data,
                            pos,
                            out,
                            if version == 0 {
                                &[0, 2, 4, 8]
                            } else if ctrl == 0 {
                                &[0, 1, 2, 4]
                            } else {
                                &[1, 2, 4, 8]
                            },
                        )?
                    }
                }
            }
            let channel = if version == 0 { 0 } else { channels[k / 4] };
            let target = &mut block_output[k..];
            // k is a four-byte-aligned channel group; only complete records
            // belong to this group. Extend the target to keep chunk strides
            // complete without touching bytes beyond this group's four bytes.
            let target_len = (block - 1) * stride + 4;
            let target = &mut target[..target_len];
            match channel & 3 {
                0 => deltas8_kernel(&deltas[..block * 4], target, block, stride, &last[k..k + 4]),
                1 => crate::codec::vertex::decode_deltas::<2, false>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    0,
                ),
                2 => crate::codec::vertex::decode_deltas::<4, true>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    (32 - u32::from(channel >> 4)) & 31,
                ),
                _ => return Err(crate::Error::InvalidStream),
            }
        }
        last[..stride].copy_from_slice(&block_output[(block - 1) * stride..]);
        offset += block;
    }
    if data.len().checked_sub(pos) != Some(padded) {
        return Err(crate::Error::InvalidStream);
    }
    Ok(())
}
