//! Independent safe Rust port of meshoptimizer 1.3.
//!
//! The standard vertex-cache optimizer and overdraw optimizer preserve the
//! pinned scalar-strict upstream ordering. All operations validate inputs and
//! use fallible allocation with explicit per-call resource limits.
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

extern crate alloc;

mod cache;
pub mod codec;
mod error;
mod input;
mod math;
mod overdraw;
mod simplify;
mod workspace;
pub use simplify::{
    simplify, simplify_into, simplify_scale, simplify_with_attributes,
    simplify_with_attributes_into, SimplifiedMesh, SimplifyOptions, SimplifyResult,
    SimplifySettings,
};

pub use cache::{
    optimize_vertex_cache, optimize_vertex_cache_in_place, optimize_vertex_cache_into,
};
pub use error::Error;
pub use input::{validate_vertex_flags, Attributes, ByteOrder, Positions, VertexFlags};
pub use overdraw::{optimize_overdraw, optimize_overdraw_in_place, optimize_overdraw_into};
pub use workspace::{Limits, Usage, Workspace};
