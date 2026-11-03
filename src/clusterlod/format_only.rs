//! Stable DAG serialization, available without the optional DAG builder.
#[path = "flat.rs"]
mod flat;
pub use flat::{FlatDagCluster, FlatDagGroup};
#[path = "dag_format.rs"]
pub mod dag_format;
