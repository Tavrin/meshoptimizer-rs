//! Optional fixed-size far-field metadata; payload bytes remain caller owned.
use super::*;
use crate::{ImpostorGrid, ImpostorLayout, ImpostorSwitch};

/// `FARF` record length, including reserved zero fields.
pub const FAR_FIELD_BYTES: usize = 64;
/// Caller-owned impostor identity and conservative mesh-space cut error.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct FarField {
    /// Opaque caller payload kind or locator tag.
    pub caller_tag: u32,
    /// XXH64 of the external payload bytes, seed zero. Not authentication.
    pub payload_hash: u64,
    /// Atlas direction grid.
    pub grid: ImpostorGrid,
    /// Texels per view side, 1 through 4096.
    pub view_resolution: u32,
    /// World-space texel error from `impostor_switch_distance`.
    pub texel_error: f64,
    /// World-space view error from `impostor_switch_distance`.
    pub view_error: f64,
    /// Upward-rounded maximum error; project it with the same sphere and cut
    /// test as clusters. Select the impostor when projected error <= threshold.
    pub world_error: f32,
}
impl FarField {
    /// Derive a handoff from a switch result and hash the caller's payload.
    /// The caller must use the DAG's all-level sphere for the switch computation.
    ///
    /// # Errors
    /// `Invalid` for invalid grid, resolution, switch terms or f32 overflow.
    pub fn from_switch(
        caller_tag: u32,
        payload: &[u8],
        grid: ImpostorGrid,
        view_resolution: u32,
        switch: ImpostorSwitch,
    ) -> Result<Self, Error> {
        if !switch.distance.is_finite() || switch.distance < 0.0 {
            return Err(Error::Invalid);
        }
        let value = Self {
            caller_tag,
            payload_hash: xxh64(payload, 0),
            grid,
            view_resolution,
            texel_error: switch.texel_error,
            view_error: switch.view_error,
            world_error: crate::clusterlod::bounds_math::f32_up(
                switch.texel_error.max(switch.view_error),
            ),
        };
        value.encode()?;
        Ok(value)
    }
    /// Encode a validated 64-byte little-endian record.
    ///
    /// # Errors
    /// `Invalid` for non-finite/negative errors, invalid grid/resolution, or a
    /// world error smaller than either component.
    pub fn encode(self) -> Result<[u8; FAR_FIELD_BYTES], Error> {
        self.grid.view_count().map_err(|_| Error::Invalid)?;
        if !(1..=4096).contains(&self.view_resolution)
            || ![
                self.texel_error,
                self.view_error,
                f64::from(self.world_error),
            ]
            .iter()
            .all(|x| x.is_finite() && *x >= 0.0)
            || f64::from(self.world_error) < self.texel_error.max(self.view_error)
        {
            return Err(Error::Invalid);
        }
        let mut b = [0u8; FAR_FIELD_BYTES];
        b[0..4].copy_from_slice(&self.caller_tag.to_le_bytes());
        b[8..16].copy_from_slice(&self.payload_hash.to_le_bytes());
        let layout = match self.grid.layout {
            ImpostorLayout::Hemisphere => 0u32,
            ImpostorLayout::Sphere => 1,
        };
        b[16..20].copy_from_slice(&layout.to_le_bytes());
        b[20..24].copy_from_slice(&self.grid.views_per_axis.to_le_bytes());
        b[24..28].copy_from_slice(&self.view_resolution.to_le_bytes());
        b[32..40].copy_from_slice(&self.texel_error.to_le_bytes());
        b[40..48].copy_from_slice(&self.view_error.to_le_bytes());
        b[48..52].copy_from_slice(&self.world_error.to_le_bytes());
        Ok(b)
    }
    /// Validate untrusted metadata without reading or allocating a payload.
    ///
    /// # Errors
    /// `Invalid` for bad length, reserved words, enum, grid, or error terms.
    pub fn decode(b: &[u8]) -> Result<Self, Error> {
        if b.len() != FAR_FIELD_BYTES || [4, 28, 52, 56, 60].iter().any(|&at| le32(b, at) != Ok(0))
        {
            return Err(Error::Invalid);
        }
        let layout = match le32(b, 16)? {
            0 => ImpostorLayout::Hemisphere,
            1 => ImpostorLayout::Sphere,
            _ => return Err(Error::Invalid),
        };
        let value = Self {
            caller_tag: le32(b, 0)?,
            payload_hash: le64(b, 8)?,
            grid: ImpostorGrid::new(layout, le32(b, 20)?).map_err(|_| Error::Invalid)?,
            view_resolution: le32(b, 24)?,
            texel_error: f64::from_bits(le64(b, 32)?),
            view_error: f64::from_bits(le64(b, 40)?),
            world_error: fl(b, 48)?,
        };
        value.encode()?;
        Ok(value)
    }
}
