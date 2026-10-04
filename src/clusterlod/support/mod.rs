// Private snapshot of lane p01x; see parity/p03/support-provenance.json.
#![allow(dead_code, unused_imports)]
mod budget;
mod input;
mod math;
mod simplify;
mod workspace;
pub(crate) use crate::Error;
pub(crate) use input::{
    validate_vertex_flags, Attributes, AttributesMut, Positions, PositionsMut, VertexFlags,
};
pub(crate) use simplify::{
    simplify_scale, simplify_sloppy, simplify_with_attributes, SimplifiedMesh, SimplifyOptions,
    SimplifySettings,
};
pub(crate) use workspace::{Limits, Workspace};
