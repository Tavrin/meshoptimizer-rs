//! Independent safe Rust port of meshoptimizer 1.3.
//!
//! The standard vertex-cache optimizer and overdraw optimizer preserve the
//! pinned scalar-strict upstream ordering. All operations validate inputs and
//! use fallible allocation with explicit per-call resource limits.
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

extern crate alloc;

mod budget;
mod cache;
pub mod codec;
mod error;
mod input;
mod math;
mod overdraw;
mod quantize;
mod remap;
mod simplify;
mod workspace;
pub use simplify::{
    simplify, simplify_into, simplify_points, simplify_points_into, simplify_prune,
    simplify_prune_into, simplify_scale, simplify_sloppy, simplify_sloppy_into,
    simplify_with_attributes, simplify_with_attributes_into, simplify_with_update, SimplifiedMesh,
    SimplifyOptions, SimplifyResult, SimplifySettings,
};

pub use cache::{
    optimize_vertex_cache, optimize_vertex_cache_fifo, optimize_vertex_cache_fifo_in_place,
    optimize_vertex_cache_fifo_into, optimize_vertex_cache_in_place, optimize_vertex_cache_into,
    optimize_vertex_cache_strip, optimize_vertex_cache_strip_in_place,
    optimize_vertex_cache_strip_into,
};
pub use error::Error;
pub use input::{
    validate_vertex_flags, Attributes, AttributesMut, ByteOrder, Positions, PositionsMut,
    VertexFlags,
};
pub use overdraw::{optimize_overdraw, optimize_overdraw_in_place, optimize_overdraw_into};
pub use quantize::{
    compute_position_exponent, dequantize_half, quantize_float, quantize_half, quantize_snorm,
    quantize_unorm,
};
pub use remap::*;
pub use workspace::{Limits, Usage, Workspace};

mod meshlet;
mod meshlet_spatial;
mod meshlet_util;
mod processing;
mod spatial;
pub use meshlet::*;
pub use meshlet_util::*;
pub use spatial::*;

mod partition;
pub use partition::*;

#[cfg(feature = "clusterlod")]
pub mod clusterlod;
/// Explicit destructive spelling for caller-buffer fetch optimization.
/// This is the same checked operation as `optimize_vertex_fetch_into`.
pub use remap::optimize_vertex_fetch_into as optimize_vertex_fetch_in_place;
/// Explicit destructive spelling for update simplification with mutable views.
/// This is the same checked operation as `simplify_with_update`.
pub use simplify::simplify_with_update as simplify_with_update_in_place;
