//! Stable DAG serialization, available without the optional DAG builder.
#[path = "bounds_math.rs"]
mod bounds_math;
#[path = "flat.rs"]
mod flat;
pub use flat::{FlatDagCluster, FlatDagGroup};
#[path = "dag_format.rs"]
pub mod dag_format;
#[path = "page.rs"]
pub mod page;

#[path = "attributes.rs"]
pub mod attributes;
