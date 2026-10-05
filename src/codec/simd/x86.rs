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
fn load8(bytes: &[u8; 8]) -> __m128i {
    // SAFETY: the initialized array supplies eight readable bytes; loadl
    // permits byte alignment, zeroes upper lanes and retains no pointer.
    unsafe { _mm_loadl_epi64(bytes.as_ptr().cast()) }
}
const GROUP_MASKS: [[u8; 8]; 256] = {
    let mut table = [[128; 8]; 256];
    let mut i = 0;
    while i < 256 {
        let mut j = 0;
        while j < 8 {
            table[i][j] = super::MASKS[i][j];
            j += 1;
        }
        i += 1;
    }
    table
};
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
// Header-space indices: v0=0..3, v1 low=4..7, v1 high=5..8.
// Zero and literal groups use the same branchless shuffle path as packed groups.
const GROUP_CONFIG: [[[u8; 16]; 4]; 9] = {
    let mut table = [[[0; 16]; 4]; 9];
    let bits = [0, 2, 4, 8, 0, 1, 2, 4, 8];
    let mut h = 0;
    while h < 9 {
        let (rep, even, odd, escape) = match bits[h] {
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
            0 => ([128; 16], [0; 16], [0; 16], 1),
            _ => ([128; 16], [0; 16], [0; 16], 0),
        };
        table[h] = [[escape; 16], rep, even, odd];
        h += 1;
    }
    table
};
const GROUP_SKIP: [usize; 9] = [0, 4, 8, 0, 0, 2, 4, 8, 0];
const GROUP_SHIFT: [u32; 9] = [4, 1, 2, 3, 4, 0, 1, 2, 3];
const GROUP_LANES: [u64; 9] = [
    0,
    0x55555555,
    0x1111111111111111,
    0,
    0,
    0xffff,
    0x55555555,
    0x1111111111111111,
    0,
];
const GROUP_ADVANCE: [usize; 9] = [0, 4, 8, 16, 0, 2, 4, 8, 16];

pub(super) fn group(_token: Ssse3, data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    // SAFETY: Ssse3 is constructed only after SSSE3 AND POPCNT detection (or
    // compile-time features in no_std); the kernel's complete input is typed.
    unsafe {
        group_kernel(
            data,
            out,
            match bits {
                0 => 0,
                1 => 5,
                2 => 1,
                4 => 2,
                8 => 3,
                _ => unreachable!(),
            },
        )
    }
}
#[inline]
#[target_feature(enable = "ssse3,popcnt")]
fn group_kernel(data: &[u8; 24], out: &mut [u8; 16], h: usize) -> usize {
    // Derive consumption from the original packed bits, independently of the
    // SIMD escape/shuffle chain, as upstream SIMD_LATENCYOPT does on x64.
    let n = GROUP_SHIFT[h];
    let mut packed = u64::from_le_bytes(*data.first_chunk::<8>().unwrap());
    packed &= packed >> n;
    packed &= packed >> (n >> 1);
    let used = GROUP_ADVANCE[h] + (packed & GROUP_LANES[h]).count_ones() as usize;
    let [sent, rep, even, odd] = &GROUP_CONFIG[h];
    let input = load8(data.first_chunk::<8>().unwrap());
    let words = _mm_shuffle_epi8(input, load(rep));
    let fields = _mm_or_si128(
        _mm_mulhi_epu16(words, load(even)),
        _mm_slli_epi16::<8>(_mm_mulhi_epu16(words, load(odd))),
    );
    let sent = load(sent);
    let sel = _mm_and_si128(fields, sent);
    let mask = _mm_cmpeq_epi8(sel, sent);
    let m = _mm_movemask_epi8(mask) as usize;
    let counts = _mm_sad_epu8(mask, _mm_setzero_si128());
    let upper = _mm_sub_epi8(
        load8(&GROUP_MASKS[m >> 8]),
        _mm_shuffle_epi8(counts, _mm_setzero_si128()),
    );
    let shuf = _mm_unpacklo_epi64(load8(&GROUP_MASKS[m & 255]), upper);
    let skip = GROUP_SKIP[h];
    let rest = load(data[skip..].first_chunk::<16>().unwrap());
    store(
        out,
        _mm_or_si128(_mm_shuffle_epi8(rest, shuf), _mm_andnot_si128(mask, sel)),
    );
    used
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
    unsafe {
        bytes_kernel(
            data,
            pos,
            out,
            if bits[1] == 1 {
                4
            } else if bits[0] == 1 {
                5
            } else {
                0
            },
        )
    }
}
#[inline]
#[target_feature(enable = "ssse3,popcnt")]
fn bytes_kernel(
    data: &[u8],
    pos: usize,
    out: &mut [u8],
    hshift: usize,
) -> Result<usize, crate::Error> {
    match hshift {
        0 => bytes_header_kernel::<0>(data, pos, out),
        4 => bytes_header_kernel::<4>(data, pos, out),
        5 => bytes_header_kernel::<5>(data, pos, out),
        _ => unreachable!(),
    }
}
#[inline]
#[target_feature(enable = "ssse3,popcnt")]
fn bytes_header_kernel<const H: usize>(
    data: &[u8],
    mut pos: usize,
    out: &mut [u8],
) -> Result<usize, crate::Error> {
    let headers = (out.len() / 16).div_ceil(4);
    let header = data
        .get(pos..pos + headers)
        .ok_or(crate::Error::InvalidStream)?;
    pos += headers;
    let (blocks, tail) = out.as_chunks_mut::<64>();
    for (i, block) in blocks.iter_mut().enumerate() {
        let control = header[i];
        if let Some(window) = data.get(pos..).and_then(|v| v.first_chunk::<96>()) {
            if control == 0 && H != 5 {
                block.fill(0);
                continue;
            }
            if control == 255 && H != 4 {
                block.copy_from_slice(&window[..64]);
                pos += 64;
                continue;
            }
            // Each group consumes at most 24 bytes. A single checked 96-byte
            // array therefore proves all four lookaheads, even full escapes.
            let chunks = block.as_chunks_mut::<16>().0;
            let mut used = group_kernel(
                window.first_chunk().unwrap(),
                &mut chunks[0],
                H + usize::from(control & 3),
            );
            used += group_kernel(
                window[used..].first_chunk().unwrap(),
                &mut chunks[1],
                H + usize::from((control >> 2) & 3),
            );
            used += group_kernel(
                window[used..].first_chunk().unwrap(),
                &mut chunks[2],
                H + usize::from((control >> 4) & 3),
            );
            used += group_kernel(
                window[used..].first_chunk().unwrap(),
                &mut chunks[3],
                H + usize::from(control >> 6),
            );
            pos += used;
        } else {
            for (j, chunk) in block.as_chunks_mut::<16>().0.iter_mut().enumerate() {
                let window = data
                    .get(pos..)
                    .and_then(|v| v.first_chunk::<24>())
                    .ok_or(crate::Error::InvalidStream)?;
                pos += group_kernel(window, chunk, H + usize::from((control >> (j * 2)) & 3));
            }
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
            H + usize::from((header[blocks.len()] >> (j * 2)) & 3),
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
    deltas_kernel::<0>(buffer, target, count, stride, last, 0);
}

#[inline]
#[target_feature(enable = "sse2")]
fn deltas_kernel<const CHANNEL: u8>(
    buffer: &[u8],
    target: &mut [u8],
    count: usize,
    stride: usize,
    last: &[u8],
    rot: u32,
) {
    let mut previous = _mm_set1_epi32(i32::from_le_bytes(last[..4].try_into().unwrap()));
    let full = count & !15;
    for start in (0..full).step_by(16) {
        let planes = core::array::from_fn::<_, 4, _>(|c| {
            load(buffer[c * count + start..].first_chunk::<16>().unwrap())
        });
        let ab0 = _mm_unpacklo_epi8(planes[0], planes[1]);
        let ab1 = _mm_unpackhi_epi8(planes[0], planes[1]);
        let cd0 = _mm_unpacklo_epi8(planes[2], planes[3]);
        let cd1 = _mm_unpackhi_epi8(planes[2], planes[3]);
        let records = [
            _mm_unpacklo_epi16(ab0, cd0),
            _mm_unpackhi_epi16(ab0, cd0),
            _mm_unpacklo_epi16(ab1, cd1),
            _mm_unpackhi_epi16(ab1, cd1),
        ];
        // One complete destination span; no partial-record decisions in
        // the unrolled path. Each vector stays live until its four stores.
        let dst = &mut target[start * stride..(start + 15) * stride + 4];
        macro_rules! emit {
            ($g:literal) => {{
                let r = delta_prefix::<CHANNEL>(records[$g], previous, rot);
                previous = _mm_shuffle_epi32::<0xff>(r);
                scatter4(&mut dst[$g * 4 * stride..], stride, r);
            }};
        }
        emit!(0);
        emit!(1);
        emit!(2);
        emit!(3);
    }
    if full < count {
        let mut last = [0; 16];
        store(&mut last, previous);
        let dst = &mut target[full * stride..];
        match CHANNEL {
            0 => crate::codec::vertex::scalar_deltas::<1, false>(
                buffer, dst, count, stride, &last, rot, full,
            ),
            1 => crate::codec::vertex::scalar_deltas::<2, false>(
                buffer, dst, count, stride, &last, rot, full,
            ),
            _ => crate::codec::vertex::scalar_deltas::<4, true>(
                buffer, dst, count, stride, &last, rot, full,
            ),
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
fn delta_prefix<const CHANNEL: u8>(mut r: __m128i, previous: __m128i, rot: u32) -> __m128i {
    r = if CHANNEL == 0 {
        _mm_xor_si128(
            _mm_and_si128(_mm_srli_epi16::<1>(r), _mm_set1_epi8(127)),
            _mm_sub_epi8(_mm_setzero_si128(), _mm_and_si128(r, _mm_set1_epi8(1))),
        )
    } else if CHANNEL == 1 {
        _mm_xor_si128(
            _mm_srli_epi16::<1>(r),
            _mm_sub_epi16(_mm_setzero_si128(), _mm_and_si128(r, _mm_set1_epi16(1))),
        )
    } else {
        _mm_or_si128(
            _mm_sll_epi32(r, _mm_cvtsi32_si128(rot as i32)),
            _mm_srl_epi32(r, _mm_cvtsi32_si128((32 - rot) as i32)),
        )
    };
    // Prefix across four packed records; byte/halfword carries stay
    // within their components, XOR lanes preserve rotated-bit identity.
    r = combine::<CHANNEL>(r, _mm_slli_si128::<4>(r));
    r = combine::<CHANNEL>(r, _mm_slli_si128::<8>(r));
    r = combine::<CHANNEL>(r, previous);
    r
}
#[inline]
#[target_feature(enable = "sse2")]
fn scatter4(dst: &mut [u8], stride: usize, r: __m128i) {
    if stride == 4 {
        store(dst.first_chunk_mut().unwrap(), r);
    } else {
        dst[..4].copy_from_slice(&_mm_cvtsi128_si32(r).to_le_bytes());
        dst[stride..stride + 4]
            .copy_from_slice(&_mm_cvtsi128_si32(_mm_shuffle_epi32::<0x55>(r)).to_le_bytes());
        dst[stride * 2..stride * 2 + 4]
            .copy_from_slice(&_mm_cvtsi128_si32(_mm_shuffle_epi32::<0xaa>(r)).to_le_bytes());
        dst[stride * 3..stride * 3 + 4]
            .copy_from_slice(&_mm_cvtsi128_si32(_mm_shuffle_epi32::<0xff>(r)).to_le_bytes());
    }
}
#[inline]
#[target_feature(enable = "sse2")]
fn combine<const CHANNEL: u8>(a: __m128i, b: __m128i) -> __m128i {
    if CHANNEL == 0 {
        _mm_add_epi8(a, b)
    } else if CHANNEL == 1 {
        _mm_add_epi16(a, b)
    } else {
        _mm_xor_si128(a, b)
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
        // With nonzero alpha every channel is finite. Alpha zero makes
        // f[3] NaN, which is the final operand of both reductions and hence
        // survives SSE min/max's second-operand NaN rule.
        let lo = _mm_min_ps(_mm_min_ps(_mm_min_ps(f[0], f[1]), f[2]), f[3]);
        let hi = _mm_max_ps(_mm_max_ps(_mm_max_ps(f[0], f[1]), f[2]), f[3]);
        let valid = _mm_movemask_ps(_mm_and_ps(
            _mm_cmpge_ps(lo, splat(-2147483648.0)),
            _mm_cmplt_ps(hi, splat(2147483648.0)),
        )) == 15;
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

// Upstream packs next[10..16] into the unused shuffle[0..6].
const TRIANGLE_MASKS: [[u8; 16]; 256] = {
    let mut masks = [[0; 16]; 256];
    let mut i = 0;
    while i < 256 {
        let (shuf, next, _, _) = super::TRIANGLE_TABLES[i];
        let mut j = 0;
        while j < 16 {
            masks[i][j] = if j < 6 { next[j + 10] } else { shuf[j] };
            j += 1;
        }
        i += 1;
    }
    masks
};
const TRIANGLE_META: [[u8; 3]; 256] = {
    let mut meta = [[0; 3]; 256];
    let mut i = 0;
    while i < 256 {
        let (_, next, used, first) = super::TRIANGLE_TABLES[i];
        meta[i] = [used as u8, first as u8, next[15]];
        i += 1;
    }
    meta
};

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
    let mut counter = 0usize;
    for i in (0..count).step_by(2) {
        if data > bound {
            return Some(Err(crate::Error::InvalidStream));
        }
        let code = codes[i / 2] as usize;
        let mask = load(&TRIANGLE_MASKS[code]);
        let [used, first_used, advance] = TRIANGLE_META[code].map(usize::from);
        counter += advance;
        if counter >= 256 {
            return None;
        }
        if i + 1 < count && data + first_used > bound {
            return Some(Err(crate::Error::InvalidStream));
        }
        let Some(window) = source.get(data..).and_then(|b| b.first_chunk::<16>()) else {
            return Some(Err(crate::Error::InvalidStream));
        };
        let merged = _mm_blend_epi16::<7>(state, load(window));
        state = _mm_add_epi8(_mm_shuffle_epi8(merged, mask), _mm_slli_si128::<10>(mask));
        let packed = _mm_cvtsi128_si64(_mm_srli_si128::<9>(state)) as u64;
        let bytes = packed.to_le_bytes();
        for k in 0..(count - i).min(2) {
            let a = u32::from(bytes[k * 3]);
            let b = u32::from(bytes[1 + k * 3]);
            let c = u32::from(bytes[2 + k * 3]);
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
                                0
                            } else {
                                4 + usize::from(ctrl)
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
                1 => deltas_kernel::<1>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    0,
                ),
                2 => deltas_kernel::<2>(
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

// Grouped output avoids an edge-format scalar callback for every triangle.
// Full groups use const-sized copies; counted tails are separate.
trait MeshletSink {
    fn vertices<const LIVE: usize>(&mut self, index: usize, values: [u8; 16]);
    fn triangles<const LIVE: usize>(&mut self, index: usize, values: [u8; 16]);
}
struct ByteSink<'a, const VS: usize, const TS: usize> {
    vertices: &'a mut [u8],
    triangles: &'a mut [u8],
}
impl<const VS: usize, const TS: usize> MeshletSink for ByteSink<'_, VS, TS> {
    #[inline(always)]
    fn vertices<const LIVE: usize>(&mut self, index: usize, values: [u8; 16]) {
        let dst = &mut self.vertices[index * VS..(index + LIVE) * VS];
        if VS == 4 {
            dst.copy_from_slice(&values[..LIVE * 4]);
        } else {
            for (out, v) in dst
                .as_chunks_mut::<VS>()
                .0
                .iter_mut()
                .zip(values.as_chunks::<4>().0)
            {
                out.copy_from_slice(&v[..VS]);
            }
        }
    }
    #[inline(always)]
    fn triangles<const LIVE: usize>(&mut self, index: usize, values: [u8; 16]) {
        self.triangles[index * TS..(index + LIVE) * TS].copy_from_slice(&values[..LIVE * TS]);
    }
}
struct RawSink<'a> {
    vertices: &'a mut [u32],
    triangles: &'a mut [u32],
}
impl MeshletSink for RawSink<'_> {
    #[inline(always)]
    fn vertices<const LIVE: usize>(&mut self, index: usize, values: [u8; 16]) {
        for (out, v) in self.vertices[index..index + LIVE]
            .iter_mut()
            .zip(values.as_chunks::<4>().0)
        {
            *out = u32::from_le_bytes(*v);
        }
    }
    #[inline(always)]
    fn triangles<const LIVE: usize>(&mut self, index: usize, values: [u8; 16]) {
        for (out, v) in self.triangles[index..index + LIVE]
            .iter_mut()
            .zip(values.as_chunks::<4>().0)
        {
            *out = u32::from_le_bytes(*v);
        }
    }
}
#[allow(clippy::too_many_arguments)]
pub(super) fn meshlet_bytes<const VS: usize, const TS: usize>(
    _token: Sse41,
    source: &[u8],
    bound: usize,
    ctrl: &[u8],
    codes: &[u8],
    vertices: &mut [u8],
    triangles: &mut [u8],
) -> Option<Result<(), crate::Error>> {
    let vc = vertices.len() / VS;
    let tc = triangles.len() / TS;
    // SAFETY: Sse41 proves SSSE3/SSE4.1; every input load is checked and
    // array-backed, and complete groups and tails use counted output slices.
    unsafe {
        meshlet_output_kernel::<TS>(
            source,
            bound,
            ctrl,
            codes,
            vc,
            tc,
            &mut ByteSink::<VS, TS> {
                vertices,
                triangles,
            },
        )
    }
}
pub(super) fn meshlet_raw(
    _token: Sse41,
    source: &[u8],
    bound: usize,
    ctrl: &[u8],
    codes: &[u8],
    vertices: &mut [u32],
    triangles: &mut [u32],
) -> Option<Result<(), crate::Error>> {
    let vc = vertices.len();
    let tc = triangles.len();
    // SAFETY: Sse41 proves SSSE3/SSE4.1; every input load is checked and
    // array-backed, and complete groups and tails use counted output slices.
    unsafe {
        meshlet_output_kernel::<4>(
            source,
            bound,
            ctrl,
            codes,
            vc,
            tc,
            &mut RawSink {
                vertices,
                triangles,
            },
        )
    }
}
#[inline]
#[target_feature(enable = "ssse3,sse4.1")]
fn meshlet_vertex_step(
    last: __m128i,
    source: &mut &[u8],
    code: u8,
) -> Result<__m128i, crate::Error> {
    let window = source
        .first_chunk::<16>()
        .ok_or(crate::Error::InvalidStream)?;
    let (mask, used) = super::meshlet_mask(code);
    let v = _mm_shuffle_epi8(load(window), load(&mask));
    let d = _mm_xor_si128(
        _mm_srli_epi32::<1>(v),
        _mm_sub_epi32(_mm_setzero_si128(), _mm_and_si128(v, _mm_set1_epi32(1))),
    );
    let mut r = _mm_add_epi32(d, _mm_set1_epi32(1));
    r = _mm_add_epi32(r, _mm_slli_si128::<8>(r));
    r = _mm_add_epi32(r, _mm_slli_si128::<4>(r));
    r = _mm_add_epi32(r, _mm_shuffle_epi32::<0xff>(last));
    *source = &source[used..];
    Ok(r)
}
#[inline]
#[target_feature(enable = "ssse3,sse4.1")]
fn meshlet_triangle_step<const TAIL: bool, const CHECK: bool>(
    state: __m128i,
    source: &mut &[u8],
    code: u8,
    counter: &mut usize,
) -> Option<Result<__m128i, crate::Error>> {
    let [used, first, advance] = TRIANGLE_META[code as usize];
    *counter += usize::from(advance);
    if CHECK && *counter >= 256 {
        return None;
    }
    let Some(window) = source.first_chunk::<16>() else {
        return Some(Err(crate::Error::InvalidStream));
    };
    let mask = load(&TRIANGLE_MASKS[code as usize]);
    let merged = _mm_blend_epi16::<7>(state, load(window));
    let r = _mm_add_epi8(_mm_shuffle_epi8(merged, mask), _mm_slli_si128::<10>(mask));
    *source = &source[usize::from(if TAIL { first } else { used })..];
    Some(Ok(r))
}
#[inline]
#[target_feature(enable = "ssse3,sse4.1")]
fn triangle_output<const TS: usize>(state: __m128i) -> __m128i {
    if TS == 4 {
        _mm_shuffle_epi8(
            state,
            _mm_setr_epi8(9, 10, 11, -1, 12, 13, 14, -1, 0, 0, 0, 0, 0, 0, 0, 0),
        )
    } else {
        _mm_srli_si128::<9>(state)
    }
}
#[inline]
#[target_feature(enable = "ssse3,sse4.1")]
fn triangle_bytes<const TS: usize>(state: __m128i) -> [u8; 16] {
    let mut bytes = [0; 16];
    store(&mut bytes, triangle_output::<TS>(state));
    bytes
}
#[target_feature(enable = "ssse3,sse4.1")]
#[allow(clippy::too_many_arguments)]
fn meshlet_output_kernel<const TS: usize>(
    source: &[u8],
    bound: usize,
    ctrl: &[u8],
    codes: &[u8],
    vc: usize,
    tc: usize,
    sink: &mut impl MeshletSink,
) -> Option<Result<(), crate::Error>> {
    let mut remaining = source;
    let mut last = _mm_set1_epi32(-1);
    for (g, &code) in ctrl[..vc / 4].iter().enumerate() {
        last = match meshlet_vertex_step(last, &mut remaining, code) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let mut values = [0; 16];
        store(&mut values, last);
        sink.vertices::<4>(g * 4, values);
    }
    if !vc.is_multiple_of(4) {
        last = match meshlet_vertex_step(last, &mut remaining, ctrl[vc / 4]) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let mut values = [0; 16];
        store(&mut values, last);
        match vc % 4 {
            1 => sink.vertices::<1>(vc & !3, values),
            2 => sink.vertices::<2>(vc & !3, values),
            _ => sink.vertices::<3>(vc & !3, values),
        }
    }
    let mut state = _mm_setzero_si128();
    let mut counter = 0;
    for (g, pair) in codes[..tc / 2].as_chunks::<2>().0.iter().enumerate() {
        // Validate the canonical u32 counter for both pairs before either
        // SIMD step. Source windows remain checked independently.
        let advance = usize::from(TRIANGLE_META[pair[0] as usize][2])
            + usize::from(TRIANGLE_META[pair[1] as usize][2]);
        if counter + advance >= 256 {
            return None;
        }
        state = match meshlet_triangle_step::<false, false>(
            state,
            &mut remaining,
            pair[0],
            &mut counter,
        )? {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let first = triangle_output::<TS>(state);
        state = match meshlet_triangle_step::<false, false>(
            state,
            &mut remaining,
            pair[1],
            &mut counter,
        )? {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let second = triangle_output::<TS>(state);
        let packed = if TS == 3 {
            // Keep precisely six bytes of the first pair; its high byte also
            // contains the state counter and must not contaminate pair two.
            _mm_or_si128(
                _mm_and_si128(first, _mm_set_epi64x(0, 0x0000_ffff_ffff_ffff)),
                _mm_slli_si128::<6>(second),
            )
        } else {
            _mm_unpacklo_epi64(first, second)
        };
        let mut bytes = [0; 16];
        store(&mut bytes, packed);
        sink.triangles::<4>(g * 4, bytes);
    }
    if tc % 4 >= 2 {
        state = match meshlet_triangle_step::<false, true>(
            state,
            &mut remaining,
            codes[tc / 4 * 2],
            &mut counter,
        )? {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        sink.triangles::<2>(tc & !3, triangle_bytes::<TS>(state));
    }
    if !tc.is_multiple_of(2) {
        state = match meshlet_triangle_step::<true, true>(
            state,
            &mut remaining,
            codes[tc / 2],
            &mut counter,
        )? {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        sink.triangles::<1>(tc & !1, triangle_bytes::<TS>(state));
    }
    // Monotonic consumption and exact final length preserve the scalar bound
    // errors. Earlier malformed-prefix writes are outside the output contract.
    Some(if source.len() - remaining.len() == bound {
        Ok(())
    } else {
        Err(crate::Error::InvalidStream)
    })
}
