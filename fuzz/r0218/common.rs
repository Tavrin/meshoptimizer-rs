//! Shared input decoding for the 0.2.18 fuzz targets (cutout trimming and impostors).
#![allow(dead_code)]

use meshoptimizer_rs::{Error, Limits, Workspace};

pub struct Cur<'a>(&'a [u8], usize);
impl<'a> Cur<'a> {
    pub fn new(d: &'a [u8]) -> Self {
        Self(d, 0)
    }
    pub fn u8(&mut self) -> u8 {
        let v = self.0.get(self.1).copied().unwrap_or(0);
        self.1 += 1;
        v
    }
    /// Skip `n` bytes and return them (zero padded past the end).
    pub fn take(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.u8()).collect()
    }
    pub fn unit(&mut self) -> f32 {
        f32::from(self.u8()) / 255.
    }
    /// Any f32 bit pattern, including NaN, infinities and subnormals.
    pub fn raw_f32(&mut self) -> f32 {
        f32::from_bits(u32::from_le_bytes([self.u8(), self.u8(), self.u8(), self.u8()]))
    }
}

/// Bytes 253..=255 become 1e30, infinity and NaN when `special` is set.
pub fn coord(b: u8, special: bool, scale: f32, bias: f32) -> f32 {
    if special {
        match b {
            255 => return f32::NAN,
            254 => return f32::INFINITY,
            253 => return 1e30,
            _ => {}
        }
    }
    (f32::from(b) - bias) / scale
}

/// Deterministic mixing for samplers and filler attributes.
pub fn mix(a: u64, b: u64) -> u64 {
    let mut x = a ^ b.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// Resource ceiling used as the "budgeted" arm; an `Ok` under it must equal the unbudgeted result.
pub fn budget(c: &mut Cur<'_>) -> Limits {
    let w = c.u8();
    let b = c.u8();
    if w >= 128 {
        return Limits {
            max_bytes: 64 << 20,
            max_work: 1 << 24,
        };
    }
    Limits {
        max_bytes: 1usize << (14 + usize::from(b) % 13),
        max_work: 1u64 << (12 + u32::from(w) % 14),
    }
}

pub fn workspace(l: Limits) -> Workspace {
    Workspace::new(l)
}

/// A budgeted call may fail only on resources; everything else must match the unbudgeted call.
pub fn resource(e: &Error) -> bool {
    matches!(e, Error::LimitExceeded | Error::AllocationFailed)
}
