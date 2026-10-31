/// One emitted cluster with topology in the flat index prefix.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct FlatDagCluster {
    /// Offset into the returned index prefix.
    pub first_index: u32,
    /// Number of triangle indices.
    pub index_count: u32,
    /// Sequential callback group identifier.
    pub group: u32,
    /// Refined group identifier, or -1 for original geometry.
    pub refined: i32,
    /// Culling sphere center.
    pub center: [f32; 3],
    /// Culling sphere radius.
    pub radius: f32,
}
/// One group in callback order, with monotonic selection bounds.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct FlatDagGroup {
    /// Simplified-group sphere center.
    pub center: [f32; 3],
    /// Simplified-group sphere radius.
    pub radius: f32,
    /// Accumulated error; terminal groups use `f32::MAX`.
    pub error: f32,
    /// DAG level.
    pub depth: i32,
    /// Offset into the returned cluster prefix.
    pub first_cluster: u32,
    /// Number of consecutive clusters.
    pub cluster_count: u32,
}
