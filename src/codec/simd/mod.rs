#![allow(unsafe_code)]
//! Audited SIMD implementation. See SAFETY.md; no ISA type leaves this module.
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
#[macro_use]
mod kernels;
#[cfg(target_arch = "aarch64")]
mod aarch64;
pub(crate) mod dispatch;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod wasm32;
#[cfg(target_arch = "x86_64")]
mod x86;

#[cfg(target_arch = "aarch64")]
use aarch64 as arch;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use wasm32 as arch;
#[cfg(target_arch = "x86_64")]
use x86 as arch;

pub(super) fn group(data: &[u8; 24], out: &mut [u8; 16], bits: u32) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::ssse3() {
        return Some(arch::group(token, data, out, bits));
    }
    #[cfg(any(
        target_arch = "aarch64",
        all(target_arch = "wasm32", target_feature = "simd128")
    ))]
    if let Some(token) = dispatch::baseline() {
        return Some(arch::group(token, data, out, bits));
    }
    let _ = (data, out, bits);
    None
}
pub(super) fn bytes(
    data: &[u8],
    pos: usize,
    out: &mut [u8],
    bits: &[u32],
) -> Option<Result<usize, crate::Error>> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::ssse3() {
        return Some(arch::bytes(token, data, pos, out, bits));
    }
    let _ = (data, pos, out, bits);
    None
}
pub(super) fn meshlet(data: &[u8; 16], code: u8, last: u32) -> Option<([u32; 4], usize)> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::sse41() {
        return Some(arch::meshlet(token, data, code, last));
    }
    #[cfg(target_arch = "aarch64")]
    if let Some(token) = dispatch::baseline() {
        return Some(arch::meshlet(token, data, code, last));
    }
    let _ = (data, code, last);
    None
}
pub(super) fn meshlet_vertices(
    source: &[u8],
    bound: usize,
    codes: &[u8],
    count: usize,
    out: &mut impl FnMut(usize, u32),
) -> Option<Result<usize, crate::Error>> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::sse41() {
        return Some(arch::meshlet_vertices(
            token, source, bound, codes, count, out,
        ));
    }
    let _ = (source, bound, codes, count, out);
    None
}
#[allow(clippy::too_many_arguments)]
pub(super) fn meshlet_bytes<const VS: usize, const TS: usize>(
    source: &[u8],
    bound: usize,
    ctrl: &[u8],
    codes: &[u8],
    vertices: &mut [u8],
    triangles: &mut [u8],
) -> Option<Result<(), crate::Error>> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::sse41() {
        return arch::meshlet_bytes::<VS, TS>(
            token, source, bound, ctrl, codes, vertices, triangles,
        );
    }
    let _ = (source, bound, ctrl, codes, vertices, triangles);
    None
}
pub(super) fn meshlet_raw(
    source: &[u8],
    bound: usize,
    ctrl: &[u8],
    codes: &[u8],
    vertices: &mut [u32],
    triangles: &mut [u32],
) -> Option<Result<(), crate::Error>> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::sse41() {
        return arch::meshlet_raw(token, source, bound, ctrl, codes, vertices, triangles);
    }
    let _ = (source, bound, ctrl, codes, vertices, triangles);
    None
}
pub(super) fn filter(kind: u8, data: &mut [u8], stride: usize) -> Option<Result<(), crate::Error>> {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        all(target_arch = "wasm32", target_feature = "simd128")
    ))]
    if let Some(token) = dispatch::baseline() {
        // Preserve the scalar exact-record cache on repeated Oct/Quat runs.
        let key = if kind == 1 { stride * 3 / 4 } else { stride };
        if (kind == 1 || kind == 2)
            && data.len() >= stride * 4
            && (1..4).all(|i| data[..key] == data[i * stride..i * stride + key])
        {
            return None;
        }
        return Some(arch::filter(token, kind, data, stride));
    }
    let _ = (kind, data, stride);
    None
}

// Escape expansion masks. High bit denotes a zero byte on every ISA.
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
const fn masks() -> [[u8; 16]; 256] {
    let mut table = [[128u8; 16]; 256];
    let mut mask = 0;
    while mask < 256 {
        let mut n = 0;
        let mut i = 0;
        while i < 8 {
            if mask & (1 << i) != 0 {
                table[mask][i] = n;
                n += 1;
            }
            i += 1;
        }
        mask += 1;
    }
    table
}
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
const MASKS: [[u8; 16]; 256] = masks();

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const fn meshlet_masks() -> [([u8; 16], usize); 256] {
    let mut table = [([128; 16], 0); 256];
    let mut code = 0;
    while code < 256 {
        let mut offset = 0;
        let mut k = 0;
        while k < 4 {
            let length = if code == 255 {
                4
            } else {
                ((code >> k) & 1) | ((code >> (k + 3)) & 2)
            };
            let mut b = 0;
            while b < length {
                table[code].0[k * 4 + b] = (offset + b) as u8;
                b += 1;
            }
            offset += length;
            k += 1;
        }
        table[code].1 = offset;
        code += 1;
    }
    table
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn meshlet_mask(code: u8) -> ([u8; 16], usize) {
    const TABLE: [([u8; 16], usize); 256] = meshlet_masks();
    TABLE[code as usize]
}

pub(super) fn deltas8(
    buffer: &[u8],
    target: &mut [u8],
    count: usize,
    stride: usize,
    last: &[u8],
) -> bool {
    #[cfg(any(
        target_arch = "x86_64",
        all(target_arch = "wasm32", target_feature = "simd128")
    ))]
    if let Some(token) = dispatch::baseline() {
        arch::deltas8(token, buffer, target, count, stride, last);
        return true;
    }
    let _ = (buffer, target, count, stride, last);
    false
}

#[cfg(feature = "parity-internals")]
pub(crate) fn sqrt4(values: [f32; 4]) -> Option<[f32; 4]> {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        all(target_arch = "wasm32", target_feature = "simd128")
    ))]
    if let Some(token) = dispatch::baseline() {
        return Some(arch::sqrt4(token, values));
    }
    let _ = values;
    None
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const fn triangle_tables() -> [([u8; 16], [u8; 16], usize, usize); 256] {
    let mut table = [([0; 16], [0; 16], 0, 0); 256];
    let mut code = 0;
    while code < 256 {
        let mut shuf = [0u8; 16];
        let mut next = [0u8; 16];
        let mut extra = 0;
        let mut nextoff = 0;
        shuf[6] = 12;
        shuf[7] = 13;
        shuf[8] = 14;
        shuf[15] = 15;
        let mut k = 0;
        let mut first_extra = 0;
        while k < 2 {
            let tri = (code >> (k * 4)) & 15;
            if tri < 12 {
                if k == 1 && tri / 4 == 0 {
                    let a = 9 + if tri & 2 != 0 { 2 } else { 0 };
                    let b = 9 + if tri & 2 != 0 { 1 } else { 2 };
                    shuf[12] = shuf[a];
                    next[12] = next[a];
                    shuf[13] = shuf[b];
                    next[13] = next[b];
                } else {
                    let off = 6 + k * 3 + (2 - tri / 4) * 3;
                    shuf[9 + k * 3] = (off + if tri & 2 != 0 { 2 } else { 0 }) as u8;
                    shuf[10 + k * 3] = (off + if tri & 2 != 0 { 1 } else { 2 }) as u8;
                }
            }
            let mut c = if tri < 12 { 2 } else { 0 };
            while c < 3 {
                let flag = if tri < 12 {
                    tri & 1 != 0
                } else if c == 0 {
                    tri > 12
                } else if c == 1 {
                    tri > 13
                } else {
                    tri > 14
                };
                let slot = 9 + k * 3 + c;
                if flag {
                    shuf[slot] = extra as u8;
                    extra += 1;
                } else {
                    shuf[slot] = 15;
                    next[slot] = nextoff as u8;
                    nextoff += 1;
                }
                c += 1;
            }
            if k == 0 {
                first_extra = extra;
            }
            k += 1;
        }
        next[15] = nextoff as u8;
        table[code] = (shuf, next, extra, first_extra);
        code += 1;
    }
    table
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const TRIANGLE_TABLES: [([u8; 16], [u8; 16], usize, usize); 256] = triangle_tables();

pub(super) fn triangles(
    source: &[u8],
    bound: usize,
    codes: &[u8],
    data: usize,
    count: usize,
    out: &mut impl FnMut(usize, u32),
) -> Option<Result<usize, crate::Error>> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::sse41() {
        return arch::triangles(token, source, bound, codes, data, count, out);
    }
    #[cfg(target_arch = "aarch64")]
    if let Some(token) = dispatch::baseline() {
        return arch::triangles(token, source, bound, codes, data, count, out);
    }
    let _ = (source, bound, codes, data, count, out);
    None
}

pub(super) fn vertex(
    output: &mut [u8],
    count: usize,
    stride: usize,
    data: &[u8],
) -> Option<Result<(), crate::Error>> {
    #[cfg(target_arch = "x86_64")]
    if let Some(token) = dispatch::ssse3() {
        return Some(arch::vertex(token, output, count, stride, data));
    }
    let _ = (output, count, stride, data);
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn delta_prefixes_stage_partial_unaligned_vectors() {
        for count in [1, 15, 16, 17, 33] {
            let buffer: alloc::vec::Vec<u8> = (0..count * 4).map(|i| (i * 31 + 17) as u8).collect();
            let last = [1u8, 2, 3, 4];
            let mut out = alloc::vec![0xcc;count*12+7];
            let mut expected = out.clone();
            for c in 0..4 {
                let mut previous = last[c];
                for i in 0..count {
                    let v = buffer[c * count + i];
                    previous = previous.wrapping_add((v >> 1) ^ 0u8.wrapping_sub(v & 1));
                    expected[i * 12 + c] = previous;
                }
            }
            if super::deltas8(&buffer, &mut out[..(count - 1) * 12 + 4], count, 12, &last) {
                assert_eq!(out, expected);
            }
        }
    }
    #[test]
    fn packed_groups_and_meshlet_masks_match_integer_reference() {
        let mut random = 1u32;
        for bits in [0u32, 1, 2, 4, 8] {
            for _ in 0..32 {
                let mut storage = [0; 25];
                for b in &mut storage {
                    random ^= random << 13;
                    random ^= random >> 17;
                    random ^= random << 5;
                    *b = random as u8;
                }
                let data: &[u8; 24] = storage[1..].try_into().unwrap();
                let mut expected = [0; 16];
                let mut used = (bits * 2) as usize;
                if bits == 8 {
                    expected.copy_from_slice(&data[..16]);
                    used = 16;
                } else if bits != 0 {
                    let escape = (1 << bits) - 1;
                    for (i, v) in expected.iter_mut().enumerate() {
                        let shift = if bits == 1 {
                            i % 8
                        } else {
                            8 - bits as usize - (i % (8 / bits as usize)) * bits as usize
                        };
                        let field = (data[i / (8 / bits as usize)] >> shift) & escape;
                        *v = if field == escape {
                            let b = data[used];
                            used += 1;
                            b
                        } else {
                            field
                        };
                    }
                } else {
                    used = 0;
                }
                let mut out = [0; 16];
                if let Some(actual) = super::group(data, &mut out, bits) {
                    assert_eq!(actual, used);
                    assert_eq!(out, expected);
                }
            }
        }
        for code in 0..=255u8 {
            let storage = [7u8; 17];
            let data = storage[1..].try_into().unwrap();
            if let Some((actual, used)) = super::meshlet(data, code, 0x12345678) {
                let mut expected = [0; 4];
                let mut offset = 0;
                let mut last = 0x12345678u32;
                for (k, v) in expected.iter_mut().enumerate() {
                    let n = if code == 255 {
                        4
                    } else {
                        usize::from(((code >> k) & 1) | ((code >> (k + 3)) & 2))
                    };
                    let mut word = [0; 4];
                    word[..n].copy_from_slice(&data[offset..offset + n]);
                    offset += n;
                    let word = u32::from_le_bytes(word);
                    last = last
                        .wrapping_add((word >> 1) ^ 0u32.wrapping_sub(word & 1))
                        .wrapping_add(1);
                    *v = last;
                }
                assert_eq!((actual, used), (expected, offset));
            }
        }
    }
    #[test]
    fn byte_headers_share_lookahead_without_accepting_short_groups() {
        for bits in [[0u32, 2, 4, 8], [0, 1, 2, 4], [1, 2, 4, 8]] {
            for header in 0..=255u8 {
                // Alternating escapes/literals, unaligned input and exact
                // fast-window/tail thresholds exercise all header-space rows.
                let mut storage = [0u8; 99];
                storage[1] = header;
                for (i, b) in storage[2..].iter_mut().enumerate() {
                    *b = (i * 71 + 255) as u8;
                }
                for length in [23usize, 24, 25, 95, 96, 97] {
                    let source = &storage[1..1 + length];
                    let mut expected = [0u8; 64];
                    let mut pos = 1;
                    let mut ok = true;
                    for (g, chunk) in expected.as_chunks_mut::<16>().0.iter_mut().enumerate() {
                        if source.len().saturating_sub(pos) < 24 {
                            ok = false;
                            break;
                        }
                        let b = bits[usize::from((header >> (g * 2)) & 3)];
                        if b == 0 {
                            chunk.fill(0);
                            continue;
                        }
                        if b == 8 {
                            chunk.copy_from_slice(&source[pos..pos + 16]);
                            pos += 16;
                            continue;
                        }
                        let start = pos;
                        pos += (b * 2) as usize;
                        let escape = (1u8 << b) - 1;
                        for (i, v) in chunk.iter_mut().enumerate() {
                            let shift = if b == 1 {
                                i as u32 % 8
                            } else {
                                8 - b - (i as u32 % (8 / b)) * b
                            };
                            let field = (source[start + i / (8 / b) as usize] >> shift) & escape;
                            *v = if field == escape {
                                let value = source[pos];
                                pos += 1;
                                value
                            } else {
                                field
                            };
                        }
                    }
                    let mut out = [0xcc; 64];
                    if let Some(result) = super::bytes(source, 0, &mut out, &bits) {
                        assert_eq!(
                            result,
                            if ok {
                                Ok(pos)
                            } else {
                                Err(crate::Error::InvalidStream)
                            },
                            "{bits:?}/{header}/{length}"
                        );
                        if ok {
                            assert_eq!(out, expected);
                        }
                    }
                }
            }
        }
    }
}
