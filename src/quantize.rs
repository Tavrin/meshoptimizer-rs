//! Bit conversions from upstream quantization.cpp and meshoptimizer.h.
use crate::Error;

/// Quantize to an unsigned normalized integer with 0 through 30 bits.
/// Infinities clamp; NaN follows upstream comparisons and becomes zero.
#[inline(always)]
pub fn quantize_unorm(value: f32, bits: u32) -> Result<i32, Error> {
    if bits > 30 {
        return Err(Error::InvalidParameter);
    }
    let power = 1u32 << bits;
    if value >= 1.0 {
        // f32 rounds the endpoint upward starting at 24 bits, including
        // the addition of 0.5 at exactly 24 bits.
        return Ok(if bits >= 24 { power } else { power - 1 } as i32);
    }
    if value > 0.0 {
        let scale = (power - 1) as f32;
        Ok((value * scale + 0.5) as i32)
    } else {
        // Includes both zeros, negative infinity and all NaNs.
        Ok(0)
    }
}

/// Quantize to a signed normalized integer with 1 through 31 bits.
/// Infinities clamp; NaN follows upstream comparisons and becomes the minimum.
#[inline]
pub fn quantize_snorm(mut value: f32, bits: u32) -> Result<i32, Error> {
    if !(1..=31).contains(&bits) {
        return Err(Error::InvalidParameter);
    }
    let scale = ((1u32 << (bits - 1)) - 1) as f32;
    let round = if value >= 0.0 { 0.5 } else { -0.5 };
    value = if value >= -1.0 { value } else { -1.0 };
    value = if value <= 1.0 { value } else { 1.0 };
    Ok((value * scale + round) as i32)
}

/// Convert to upstream's half representation, flushing subnormals and
/// canonicalizing NaNs to signed quiet NaNs.
#[inline]
pub fn quantize_half(value: f32) -> u16 {
    let ui = value.to_bits();
    let sign = (ui >> 16) & 0x8000;
    let em = ui & 0x7fffffff;
    let mut half = em.wrapping_sub(112 << 23).wrapping_add(1 << 12) >> 13;
    if em < 113 << 23 {
        half = 0;
    }
    if em >= 143 << 23 {
        half = 0x7c00;
    }
    if em > 255 << 23 {
        half = 0x7e00;
    }
    (sign | half) as u16
}

/// Round to 0 through 23 mantissa bits. Preserve infinity and NaN payloads;
/// flush all exponent-zero values, including negative zero, to positive zero.
#[inline]
pub fn quantize_float(value: f32, bits: u32) -> Result<f32, Error> {
    if bits > 23 {
        return Err(Error::InvalidParameter);
    }
    let mut ui = value.to_bits();
    let mask = (1u32 << (23 - bits)) - 1;
    let round = (1u32 << (23 - bits)) >> 1;
    let exponent = ui & 0x7f800000;
    if exponent != 0x7f800000 {
        ui = ui.wrapping_add(round) & !mask;
    }
    if exponent == 0 {
        ui = 0;
    }
    Ok(f32::from_bits(ui))
}

/// Expand a half value; flush half subnormals, preserve signed zero and NaN payloads.
#[inline]
pub fn dequantize_half(half: u16) -> f32 {
    let sign = u32::from(half & 0x8000) << 16;
    let em = u32::from(half & 0x7fff);
    let mut result = (em + (112 << 10)) << 13;
    if em < 1 << 10 {
        result = 0;
    }
    if em >= 31 << 10 {
        result += 112 << 23;
    }
    f32::from_bits(sign | result)
}

/// Compute the exponent of a conservative signed position grid.
/// Bounds must be finite and ordered; `min_exp` is at least -126 and
/// `max_bits` is 2 through 24. Overflowing scaled bounds return an error.
#[inline]
pub fn compute_position_exponent(
    min: [f32; 3],
    max: [f32; 3],
    min_exp: i32,
    max_bits: u32,
) -> Result<i32, Error> {
    if min_exp < -126 || !(2..=24).contains(&max_bits) {
        return Err(Error::InvalidParameter);
    }
    let mut maxc = 0.0f32;
    for k in 0..3 {
        if !min[k].is_finite() || !max[k].is_finite() || min[k] > max[k] {
            return Err(Error::InvalidParameter);
        }
        maxc = if maxc < min[k].abs() {
            min[k].abs()
        } else {
            maxc
        };
        maxc = if maxc < max[k].abs() {
            max[k].abs()
        } else {
            maxc
        };
    }
    let (fraction, exponent) = libm::frexpf(maxc);
    let offset = 23 - i32::from(fraction >= 1.0 - f32::EPSILON / 2.0);
    let mut exp = min_exp.max(exponent - offset);
    let scale = libm::ldexpf(1.0, -exp);
    let mut range = 0.0f32;
    for k in 0..3 {
        let a = libm::floorf(min[k] * scale);
        let v = libm::ceilf(max[k] * scale);
        let d = v - a;
        if !d.is_finite() {
            return Err(Error::NumericalFailure);
        }
        range = if range < d { d } else { range };
    }
    let (fraction, exponent) = libm::frexpf(range);
    if exponent > max_bits as i32 {
        exp = exp
            .checked_add(exponent - max_bits as i32)
            .ok_or(Error::SizeOverflow)?;
        exp = exp
            .checked_add(i32::from(
                fraction >= 1.0 - 1.0 / ((1u32 << max_bits) - 1) as f32,
            ))
            .ok_or(Error::SizeOverflow)?;
    }
    Ok(exp)
}
