use super::dispatch::Baseline;
use core::arch::aarch64::*;
#[inline]
#[target_feature(enable = "neon")]
fn fload(v: [f32; 4]) -> float32x4_t {
    vsetq_lane_f32::<3>(
        v[3],
        vsetq_lane_f32::<2>(v[2], vsetq_lane_f32::<1>(v[1], vdupq_n_f32(v[0]))),
    )
}
#[inline]
#[target_feature(enable = "neon")]
fn splat(v: f32) -> float32x4_t {
    vdupq_n_f32(v)
}
#[inline]
#[target_feature(enable = "neon")]
fn add(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vaddq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn sub(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vsubq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn mul(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vmulq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn div(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vdivq_f32(a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn sqrt(a: float32x4_t) -> float32x4_t {
    select(ge(a, splat(0.0)), vsqrtq_f32(a), splat(f32::NAN))
}
#[inline]
#[target_feature(enable = "neon")]
fn abs(a: float32x4_t) -> float32x4_t {
    vabsq_f32(a)
}
#[inline]
#[target_feature(enable = "neon")]
fn neg(a: float32x4_t) -> float32x4_t {
    vnegq_f32(a)
}
#[inline]
#[target_feature(enable = "neon")]
fn ge(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vreinterpretq_f32_u32(vcgeq_f32(a, b))
}
#[inline]
#[target_feature(enable = "neon")]
fn select(mask: float32x4_t, a: float32x4_t, b: float32x4_t) -> float32x4_t {
    vbslq_f32(vreinterpretq_u32_f32(mask), a, b)
}
#[inline]
#[target_feature(enable = "neon")]
fn load(bytes: &[u8; 16]) -> uint8x16_t {
    // SAFETY: the initialized array supplies 16 readable bytes; vld1q_u8
    // permits byte alignment and the pointer is not retained.
    unsafe { vld1q_u8(bytes.as_ptr()) }
}
#[inline]
#[target_feature(enable = "neon")]
fn store(bytes: &mut [u8; 16], v: uint8x16_t) {
    // SAFETY: the exclusive array supplies 16 writable bytes; vst1q_u8
    // permits byte alignment and the pointer is not retained.
    unsafe { vst1q_u8(bytes.as_mut_ptr(), v) }
}
#[target_feature(enable = "neon")]
fn fout(v: float32x4_t) -> [f32; 4] {
    let mut b = [0; 16];
    store(&mut b, vreinterpretq_u8_f32(v));
    core::array::from_fn(|i| {
        f32::from_bits(u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
    })
}
#[target_feature(enable = "neon")]
fn trunc(v: float32x4_t) -> [i32; 4] {
    let mut b = [0; 16];
    store(&mut b, vreinterpretq_u8_s32(vcvtq_s32_f32(v)));
    core::array::from_fn(|i| i32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
}
#[target_feature(enable = "neon")]
fn expand(data: &[u8; 16], mask: &[u8; 16], fields: &[u8; 16], out: &mut [u8; 16]) {
    store(
        out,
        vorrq_u8(vqtbl1q_u8(load(data), load(mask)), load(fields)),
    );
}
filter_kernel!("neon");
portable_group!("neon");
pub(super) fn filter(
    _token: Baseline,
    kind: u8,
    data: &mut [u8],
    stride: usize,
) -> Result<(), crate::Error> {
    // SAFETY: Baseline is issued only on this architecture with neon
    // enabled; the safe kernel checks all slice bounds.
    unsafe { filter_kernel(kind, data, stride) }
}
pub(super) fn group(_token: Baseline, data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> usize {
    // SAFETY: Baseline proves neon availability; complete initialized
    // input and exclusive output arrays supply all memory used by the kernel.
    unsafe { group_kernel(data, out, bits) }
}
pub(super) fn meshlet(_token: Baseline, data: &[u8; 16], code: u8, last: u32) -> ([u32; 4], usize) {
    // SAFETY: Baseline proves AArch64 NEON, and the complete input is typed.
    unsafe { meshlet_kernel(data, code, last) }
}
#[target_feature(enable = "neon")]
fn meshlet_kernel(data: &[u8; 16], code: u8, last: u32) -> ([u32; 4], usize) {
    let (mask, used) = super::meshlet_mask(code);
    let v = vreinterpretq_u32_u8(vqtbl1q_u8(load(data), load(&mask)));
    let d = veorq_u32(
        vshrq_n_u32::<1>(v),
        vsubq_u32(vdupq_n_u32(0), vandq_u32(v, vdupq_n_u32(1))),
    );
    let mut r = vaddq_u32(d, vdupq_n_u32(1));
    r = vaddq_u32(r, vextq_u32::<3>(vdupq_n_u32(0), r));
    r = vaddq_u32(r, vextq_u32::<2>(vdupq_n_u32(0), r));
    r = vaddq_u32(r, vdupq_n_u32(last));
    let mut out = [0; 16];
    store(&mut out, vreinterpretq_u8_u32(r));
    (
        core::array::from_fn(|i| u32::from_le_bytes(out[i * 4..i * 4 + 4].try_into().unwrap())),
        used,
    )
}

#[cfg(feature = "parity-internals")]
pub(super) fn sqrt4(_token: Baseline, values: [f32; 4]) -> [f32; 4] {
    // SAFETY: Baseline proves this backend's ISA; the kernel uses only values
    // and the already audited fixed-array store helper.
    unsafe { sqrt4_kernel(values) }
}
#[cfg(feature = "parity-internals")]
#[target_feature(enable = "neon")]
fn sqrt4_kernel(values: [f32; 4]) -> [f32; 4] {
    fout(sqrt(fload(values)))
}

#[target_feature(enable = "neon")]
fn exp_kernel(data: &mut [u8]) {
    let (chunks, tail) = data.as_chunks_mut::<16>();
    for chunk in chunks {
        let v = vreinterpretq_s32_u8(load(chunk));
        let m = vcvtq_f32_s32(vshrq_n_s32::<8>(vshlq_n_s32::<8>(v)));
        let e = vshlq_n_s32::<23>(vaddq_s32(vshrq_n_s32::<24>(v), vdupq_n_s32(127)));
        store(
            chunk,
            vreinterpretq_u8_f32(vmulq_f32(vreinterpretq_f32_s32(e), m)),
        );
    }
    crate::codec::filter::scalar_exp(tail);
}

#[target_feature(enable = "neon")]
fn color_kernel<const W: usize, const N: usize>(data: &mut [u8]) -> Result<(), crate::Error> {
    let (blocks, tail) = data.as_chunks_mut::<N>();
    for block in blocks {
        let (y, co, cg, alpha) = if W == 4 {
            let v = vreinterpretq_s32_u8(load(block.first_chunk::<16>().unwrap()));
            (
                vandq_s32(v, vdupq_n_s32(255)),
                vshrq_n_s32::<24>(vshlq_n_s32::<16>(v)),
                vshrq_n_s32::<24>(vshlq_n_s32::<8>(v)),
                vreinterpretq_s32_u32(vshrq_n_u32::<24>(vreinterpretq_u32_s32(v))),
            )
        } else {
            let (lo, hi) = {
                let a = vreinterpretq_s32_u8(load(block[..16].first_chunk().unwrap()));
                let b = vreinterpretq_s32_u8(load(block[16..].first_chunk().unwrap()));
                (vuzp1q_s32(a, b), vuzp2q_s32(a, b))
            };
            (
                vandq_s32(lo, vdupq_n_s32(65535)),
                vshrq_n_s32::<16>(lo),
                vshrq_n_s32::<16>(vshlq_n_s32::<16>(hi)),
                vreinterpretq_s32_u32(vshrq_n_u32::<16>(vreinterpretq_u32_s32(hi))),
            )
        };
        let mut scale = alpha;
        scale = vorrq_s32(scale, vshrq_n_s32::<1>(scale));
        scale = vorrq_s32(scale, vshrq_n_s32::<2>(scale));
        scale = vorrq_s32(scale, vshrq_n_s32::<4>(scale));
        scale = vorrq_s32(scale, vshrq_n_s32::<8>(scale));
        let a = vorrq_s32(
            vandq_s32(vshlq_n_s32::<1>(alpha), scale),
            vandq_s32(alpha, vdupq_n_s32(1)),
        );
        let ss = div(
            splat(if W == 4 { 255.0 } else { 65535.0 }),
            vcvtq_f32_s32(scale),
        );
        let channels = [
            vsubq_s32(vaddq_s32(y, co), cg),
            vaddq_s32(y, cg),
            vsubq_s32(vsubq_s32(y, co), cg),
            a,
        ];
        let f = channels.map(|v| add(mul(vcvtq_f32_s32(v), ss), splat(0.5)));
        let mut mask = vdupq_n_u32(u32::MAX);
        for v in f {
            mask = vandq_u32(
                mask,
                vandq_u32(
                    vcgeq_f32(v, splat(-2147483648.0)),
                    vcltq_f32(v, splat(2147483648.0)),
                ),
            );
        }
        let valid = vminvq_u32(mask) == u32::MAX;
        if !valid {
            crate::codec::filter::scalar_color(block, W)?;
            continue;
        }
        let [r, g, b, a] = f.map(|v| vcvtq_s32_f32(v));
        if W == 4 {
            let value = vorrq_s32(
                vorrq_s32(
                    vandq_s32(r, vdupq_n_s32(255)),
                    vshlq_n_s32::<8>(vandq_s32(g, vdupq_n_s32(255))),
                ),
                vorrq_s32(
                    vshlq_n_s32::<16>(vandq_s32(b, vdupq_n_s32(255))),
                    vshlq_n_s32::<24>(a),
                ),
            );
            store(
                block.first_chunk_mut::<16>().unwrap(),
                vreinterpretq_u8_s32(value),
            );
        } else {
            let lo = vorrq_s32(vandq_s32(r, vdupq_n_s32(65535)), vshlq_n_s32::<16>(g));
            let hi = vorrq_s32(vandq_s32(b, vdupq_n_s32(65535)), vshlq_n_s32::<16>(a));
            store(
                block[..16].first_chunk_mut().unwrap(),
                vreinterpretq_u8_s32(vzip1q_s32(lo, hi)),
            );
            store(
                block[16..].first_chunk_mut().unwrap(),
                vreinterpretq_u8_s32(vzip2q_s32(lo, hi)),
            );
        }
    }
    crate::codec::filter::scalar_color(tail, W)
}

triangle_kernel!("neon");
#[target_feature(enable = "neon")]
fn triangle_step(state: &mut [u8; 16], extra: &[u8; 16], shuf: &[u8; 16], next: &[u8; 16]) {
    let mask = [255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let s = vbslq_u8(load(&mask), load(extra), load(state));
    store(state, vaddq_u8(vqtbl1q_u8(s, load(shuf)), load(next)));
}
pub(super) fn triangles(
    _token: Baseline,
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

#[inline]
#[target_feature(enable = "neon")]
fn combine<const CHANNEL: u8>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    if CHANNEL == 0 {
        vaddq_u8(a, b)
    } else if CHANNEL == 1 {
        vreinterpretq_u8_u16(vaddq_u16(vreinterpretq_u16_u8(a), vreinterpretq_u16_u8(b)))
    } else {
        veorq_u8(a, b)
    }
}
#[inline]
#[target_feature(enable = "neon")]
fn delta_prefix<const CHANNEL: u8>(
    mut r: uint8x16_t,
    previous: uint8x16_t,
    rot: u32,
) -> uint8x16_t {
    r = if CHANNEL == 0 {
        veorq_u8(
            vshrq_n_u8::<1>(r),
            vsubq_u8(vdupq_n_u8(0), vandq_u8(r, vdupq_n_u8(1))),
        )
    } else if CHANNEL == 1 {
        let v = vreinterpretq_u16_u8(r);
        vreinterpretq_u8_u16(veorq_u16(
            vshrq_n_u16::<1>(v),
            vsubq_u16(vdupq_n_u16(0), vandq_u16(v, vdupq_n_u16(1))),
        ))
    } else {
        let v = vreinterpretq_u32_u8(r);
        vreinterpretq_u8_u32(vorrq_u32(
            vshlq_u32(v, vdupq_n_s32(rot as i32)),
            vshlq_u32(v, vdupq_n_s32(rot as i32 - 32)),
        ))
    };
    let zero = vdupq_n_u8(0);
    r = combine::<CHANNEL>(r, vextq_u8::<12>(zero, r));
    r = combine::<CHANNEL>(r, vextq_u8::<8>(zero, r));
    combine::<CHANNEL>(r, previous)
}
#[inline]
#[target_feature(enable = "neon")]
fn deltas_kernel<const CHANNEL: u8, const FIXED: usize>(
    buffer: &[u8],
    target: &mut [u8],
    count: usize,
    stride: usize,
    last: &[u8],
    rot: u32,
) {
    let stride = if FIXED == 0 { stride } else { FIXED };
    let mut previous = vreinterpretq_u8_u32(vdupq_n_u32(u32::from_le_bytes(
        last[..4].try_into().unwrap(),
    )));
    let full = count & !15;
    for start in (0..full).step_by(16) {
        let planes = core::array::from_fn::<_, 4, _>(|c| {
            load(buffer[c * count + start..].first_chunk::<16>().unwrap())
        });
        let ab0 = vzip1q_u8(planes[0], planes[1]);
        let cd0 = vzip1q_u8(planes[2], planes[3]);
        let ab1 = vzip2q_u8(planes[0], planes[1]);
        let cd1 = vzip2q_u8(planes[2], planes[3]);
        let records = [
            vreinterpretq_u8_u16(vzip1q_u16(
                vreinterpretq_u16_u8(ab0),
                vreinterpretq_u16_u8(cd0),
            )),
            vreinterpretq_u8_u16(vzip2q_u16(
                vreinterpretq_u16_u8(ab0),
                vreinterpretq_u16_u8(cd0),
            )),
            vreinterpretq_u8_u16(vzip1q_u16(
                vreinterpretq_u16_u8(ab1),
                vreinterpretq_u16_u8(cd1),
            )),
            vreinterpretq_u8_u16(vzip2q_u16(
                vreinterpretq_u16_u8(ab1),
                vreinterpretq_u16_u8(cd1),
            )),
        ];
        for (g, record) in records.into_iter().enumerate() {
            let r = delta_prefix::<CHANNEL>(record, previous, rot);
            previous = vreinterpretq_u8_u32(vdupq_laneq_u32::<3>(vreinterpretq_u32_u8(r)));
            let mut bytes = [0; 16];
            store(&mut bytes, r);
            let dst = &mut target[(start + g * 4) * stride..];
            if stride == 4 {
                dst[..16].copy_from_slice(&bytes);
            } else {
                for (i, word) in bytes.as_chunks::<4>().0.iter().enumerate() {
                    dst[i * stride..i * stride + 4].copy_from_slice(word);
                }
            }
        }
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
#[target_feature(enable = "neon")]
fn header_group(data: &[u8; 24], out: &mut [u8; 16], h: usize) -> usize {
    const BITS: [u32; 9] = [0, 2, 4, 8, 0, 1, 2, 4, 8];
    group_kernel(data, out, BITS[h])
}
#[inline]
#[target_feature(enable = "neon")]
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
#[target_feature(enable = "neon")]
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
            let mut used = header_group(
                window.first_chunk().unwrap(),
                &mut chunks[0],
                H + usize::from(control & 3),
            );
            used += header_group(
                window[used..].first_chunk().unwrap(),
                &mut chunks[1],
                H + usize::from((control >> 2) & 3),
            );
            used += header_group(
                window[used..].first_chunk().unwrap(),
                &mut chunks[2],
                H + usize::from((control >> 4) & 3),
            );
            used += header_group(
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
                pos += header_group(window, chunk, H + usize::from((control >> (j * 2)) & 3));
            }
        }
    }
    for (j, chunk) in tail.as_chunks_mut::<16>().0.iter_mut().enumerate() {
        let window = data
            .get(pos..)
            .and_then(|v| v.first_chunk::<24>())
            .ok_or(crate::Error::InvalidStream)?;
        pos += header_group(
            window,
            chunk,
            H + usize::from((header[blocks.len()] >> (j * 2)) & 3),
        );
    }
    Ok(pos)
}

pub(super) fn vertex<D: crate::codec::vertex::Destination + ?Sized>(
    _token: Baseline,
    output: &mut D,
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), crate::Error> {
    // SAFETY: Baseline proves AArch64 NEON. Every load/store uses an
    // initialized fixed array behind checked slices; the parser validates
    // complete group windows and keeps scalar channel commit order.
    unsafe { vertex_kernel::<1024, 0, D>(output, count, stride, data) }
}
#[target_feature(enable = "neon")]
fn vertex_kernel<
    const SCRATCH: usize,
    const FIXED: usize,
    D: crate::codec::vertex::Destination + ?Sized,
>(
    output: &mut D,
    count: usize,
    stride: usize,
    data: &[u8],
) -> Result<(), crate::Error> {
    let stride = if FIXED == 0 { stride } else { FIXED };
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
    // Four planes, each starting j * block and rounded up to 16 lanes.
    // Up to two 16-record groups fit 128 bytes; full blocks fit 1024.
    let mut deltas = [0u8; SCRATCH];
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
        let block_output = output.block(offset * stride, (offset + block) * stride);
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
                0 => deltas_kernel::<0, FIXED>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    0,
                ),
                1 => deltas_kernel::<1, FIXED>(
                    &deltas[..block * 4],
                    target,
                    block,
                    stride,
                    &last[k..k + 4],
                    0,
                ),
                2 => deltas_kernel::<2, FIXED>(
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
        output.finish_block();
        offset += block;
    }
    if data.len().checked_sub(pos) != Some(padded) {
        return Err(crate::Error::InvalidStream);
    }
    Ok(())
}
