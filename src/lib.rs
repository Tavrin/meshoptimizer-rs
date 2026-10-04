//! A pure safe Rust port of [meshoptimizer](https://github.com/zeux/meshoptimizer) 1.3.
//!
//! Every ported function produces byte-identical output to meshoptimizer 1.3
//! (scalar build). Differential runs against the C++ library, seeded sweeps,
//! fuzzing and a wasm32 identity check prove this on the recorded inputs.
//! The crate needs no C++ toolchain, forbids `unsafe`, and supports `no_std`
//! with `alloc`. Invalid input returns a typed [`Error`] instead of undefined
//! behaviour. A reusable [`Workspace`] holds scratch memory, makes every
//! allocation fallible and enforces per-call memory and work [`Limits`].
//!
//! This is an independent project, not affiliated with meshoptimizer or its
//! author. Coverage, measured performance and the parity records are in the
//! [README](https://github.com/Tavrin/meshoptimizer-rs#readme).
//!
//! # Simplify a mesh
//!
//! ```
//! use meshoptimizer_rs::{simplify, Positions, SimplifyOptions, SimplifySettings, Workspace};
//!
//! let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
//! let indices = [0, 1, 2, 2, 1, 3];
//! let mut workspace = Workspace::default();
//!
//! let settings = SimplifySettings {
//!     target_index_count: 3,
//!     target_error: 0.01,
//!     options: SimplifyOptions::EMPTY,
//! };
//! let lod = simplify(&indices, Positions::from_packed(&positions), settings, &mut workspace)?;
//! println!("{} indices, relative error {}", lod.indices.len(), lod.error);
//! # Ok::<(), meshoptimizer_rs::Error>(())
//! ```
//!
//! # Optimize for the vertex cache, then for overdraw
//!
//! ```
//! use meshoptimizer_rs::{optimize_overdraw, optimize_vertex_cache, Positions, Workspace};
//!
//! let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
//! let indices = [0, 1, 2, 2, 1, 3];
//! let mut workspace = Workspace::default();
//!
//! let cached = optimize_vertex_cache(&indices, positions.len(), &mut workspace)?;
//! let ordered = optimize_overdraw(&cached, Positions::from_packed(&positions), 1.05, &mut workspace)?;
//! assert_eq!(ordered.len(), indices.len());
//! # Ok::<(), meshoptimizer_rs::Error>(())
//! ```
//!
//! # Build meshlets
//!
//! ```
//! use meshoptimizer_rs::{build_meshlets, MeshletSettings, Positions, Workspace};
//!
//! let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
//! let indices = [0, 1, 2, 2, 1, 3];
//! let mut workspace = Workspace::default();
//!
//! // 64 vertices and 124 triangles per meshlet; cone weight 0.25.
//! let built = build_meshlets(
//!     &indices,
//!     Positions::from_packed(&positions),
//!     MeshletSettings::default(),
//!     0.25,
//!     &mut workspace,
//! )?;
//! for meshlet in &built.meshlets {
//!     let start = meshlet.vertex_offset as usize;
//!     let vertices = &built.vertices[start..start + meshlet.vertex_count as usize];
//!     println!("{} triangles over vertices {:?}", meshlet.triangle_count, vertices);
//! }
//! # Ok::<(), meshoptimizer_rs::Error>(())
//! ```
//!
//! # Compress vertex and index buffers
//!
//! The [`codec`] module reads and writes the `EXT_meshopt_compression` formats.
//! Buffers are little-endian bytes. The format version and level are set per
//! call instead of through global setters.
//!
//! ```
//! use meshoptimizer_rs::codec::{
//!     decode_index_buffer, decode_vertex_buffer, encode_index_buffer, encode_vertex_buffer,
//!     IndexEncoding, VertexEncoding,
//! };
//! use meshoptimizer_rs::Workspace;
//!
//! let positions: [[f32; 3]; 4] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
//! let indices = [0, 1, 2, 2, 1, 3];
//! let mut workspace = Workspace::default();
//!
//! let vertex_bytes: Vec<u8> = positions.iter().flatten().flat_map(|v| v.to_le_bytes()).collect();
//! let packed = encode_vertex_buffer(&vertex_bytes, 4, 12, VertexEncoding::DEFAULT, &mut workspace)?;
//! assert_eq!(decode_vertex_buffer(4, 12, &packed, &mut workspace)?, vertex_bytes);
//!
//! let packed = encode_index_buffer(&indices, IndexEncoding::DEFAULT, &mut workspace)?;
//! let decoded = decode_index_buffer(indices.len(), 4, &packed, &mut workspace)?; // u32 bytes
//! assert_eq!(decoded.len(), indices.len() * 4);
//! # Ok::<(), meshoptimizer_rs::Error>(())
//! ```
//!
//! # Features
//!
//! - `std` (default): implements `std::error::Error` for [`Error`]. Disable
//!   default features for `no_std`; an allocator is still required.
//! - `clusterlod`: the cluster-LOD builder from upstream's `demo/clusterlod.h`.
//!   It reproduces the pinned demo exactly; it is not a stable upstream API.
//! - `experimental`: upstream functions and options marked experimental.
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
