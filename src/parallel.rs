//! Ordered batches of independent sequential operations, using Rayon.
//!
//! Every successful item has exactly the sequential operation's meaningful
//! bytes, including float bits. There are no parallel reductions. Results stay
//! in input order, and an item's [`Error`] does not cancel other items. The outer
//! [`BatchResult`] error concerns only allocation of the result slots, before
//! any item runs. Panics follow Rayon's usual propagation rules.
//!
//! Each executing thread has a private [`Workspace`]. Scratch is cleared before
//! and after each item, so an earlier item's retained capacity cannot change a
//! later item's limit outcome. The supplied [`Limits`] apply independently to
//! each item. A LOD chain includes all its live output, scratch and cumulative
//! work in that budget. The batch result slots, caller inputs and Rayon's pool
//! infrastructure are excluded. There is no aggregate batch memory limit: all
//! successful outputs remain live until the caller drops them.
//!
//! Work uses the current Rayon pool (normally the global pool). Use
//! [`rayon::ThreadPool::install`] to select an existing pool; the crate does not
//! explicitly create pools. Rayon lazily initializes the global pool on first
//! use if none was configured; initialization failure panics inside Rayon,
//! rather than returning a per-item [`Error`]. Native threads are required for
//! parallel execution.
//!
//! ```
//! use meshoptimizer_rs::{parallel::{encode_buffers_batch, EncodeInput}, codec::IndexEncoding, Limits};
//! let indices = [0, 1, 2, 2, 1, 3];
//! let views = [EncodeInput::Triangles { indices: &indices, encoding: IndexEncoding::DEFAULT }];
//! let outputs = encode_buffers_batch(&views, Limits::default())?;
//! assert!(outputs[0].is_ok());
//! # Ok::<(), meshoptimizer_rs::Error>(())
//! ```
//!
//! ```
//! use meshoptimizer_rs::{parallel::{build_meshlets_batch, MeshletInput, MeshletBuilder},
//!     Limits, MeshletSettings, Positions};
//! let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
//! let indices = [0, 1, 2];
//! let inputs = [MeshletInput { indices: &indices, positions: Positions::from_packed(&positions),
//!     settings: MeshletSettings::default(), builder: MeshletBuilder::Scan }];
//! let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build()?;
//! let outputs = pool.install(|| build_meshlets_batch(&inputs, Limits::default()))?;
//! assert_eq!(outputs[0].as_ref().unwrap().meshlets.len(), 1);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::codec::{self, BufferView, IndexEncoding, VertexEncoding};
use crate::workspace::checked_bytes;
use crate::{
    Attributes, Error, Limits, MeshletSettings, Meshlets, MeshletsGeneric, Positions,
    SimplifiedMesh, SimplifiedMeshGeneric, SimplifySettings, VertexFlags, Workspace,
};
use alloc::vec::Vec;
use core::mem::size_of;
use rayon::prelude::*;
use std::cell::RefCell;

/// Ordered per-item results, or a result-slot allocation error before execution.
pub type BatchResult<T> = Result<Vec<Result<T, Error>>, Error>;

std::thread_local! {
    static WORKSPACE: RefCell<Workspace> = RefCell::new(Workspace::default());
}

fn slots<T>(count: usize) -> BatchResult<T> {
    checked_bytes(count, size_of::<Result<T, Error>>())?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    result.resize_with(count, || Err(Error::AllocationFailed));
    Ok(result)
}

fn item<T>(
    limits: Limits,
    run: impl FnOnce(&mut Workspace) -> Result<T, Error>,
) -> Result<T, Error> {
    WORKSPACE.with(|cell| {
        let mut workspace = cell.borrow_mut();
        workspace.set_limits(limits);
        let result = run(&mut workspace);
        workspace.clear();
        result
    })
}

fn batch<I: Sync, T: Send>(
    inputs: &[I],
    limits: Limits,
    run: impl Fn(&I, &mut Workspace) -> Result<T, Error> + Sync,
) -> BatchResult<T> {
    let mut result = slots(inputs.len())?;
    result
        .par_iter_mut()
        .zip(inputs.par_iter())
        .for_each(|(out, input)| {
            *out = item(limits, |workspace| run(input, workspace));
        });
    Ok(result)
}

/// Optional attributes and flags used at every level of a LOD chain.
#[derive(Clone, Copy, Debug)]
pub struct LodAttributes<'a> {
    /// Attribute records for the unchanged full vertex array.
    pub attributes: Attributes<'a>,
    /// One weight per component.
    pub weights: &'a [f32],
    /// Optional per-vertex simplifier flags.
    pub vertex_flags: Option<&'a [VertexFlags]>,
}

/// One chain. Each level simplifies the preceding level's topology.
#[derive(Clone, Copy, Debug)]
pub struct LodChainInputGeneric<'a, I: crate::Index = u32> {
    /// Original triangle topology.
    pub indices: &'a [I],
    /// Full vertex positions; returned indices keep their original references.
    pub positions: Positions<'a>,
    /// Per-level settings, in execution order. Targets are index counts relative
    /// to that level's input. Error values are returned unchanged, not accumulated.
    pub levels: &'a [SimplifySettings],
    /// None selects plain simplification; Some selects attribute simplification.
    pub attributes: Option<LodAttributes<'a>>,
}

/// Sequential reference for [`simplify_lod_chains_batch`].
///
/// Calls [`crate::simplify`] or [`crate::simplify_with_attributes`] in level
/// order. No cache optimization, compaction or error adjustment is added.
/// Empty settings produce an empty chain without validating unused mesh data.
/// A failed level drops the entire chain. The workspace reports cumulative work
/// (including one visit per level) and peak live storage for the whole chain.
///
/// ```
/// use meshoptimizer_rs::{parallel::{simplify_lod_chain, LodChainInput}, Positions,
///     SimplifySettings, SimplifyOptions, Workspace};
/// let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
/// let indices = [0, 1, 2];
/// let levels = [SimplifySettings { target_index_count: 3, target_error: 0.01,
///     options: SimplifyOptions::EMPTY }];
/// let chain = simplify_lod_chain(&LodChainInput { indices: &indices,
///     positions: Positions::from_packed(&p), levels: &levels, attributes: None },
///     &mut Workspace::default())?;
/// assert_eq!(chain[0].indices, indices);
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
///
/// # Errors
/// Same errors as [`crate::simplify_with_attributes`]. Limits apply to cumulative work and peak live storage for the chain.
pub fn simplify_lod_chain_generic<I: crate::Index>(
    input: &LodChainInputGeneric<'_, I>,
    workspace: &mut Workspace,
) -> Result<Vec<SimplifiedMeshGeneric<I>>, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let limits = workspace.limits();
        workspace.release_unused();
        let mut levels = crate::budget::Budget::new(workspace)
            .reserved::<SimplifiedMeshGeneric<I>>(input.levels.len())?;
        let mut live = checked_bytes(levels.capacity(), size_of::<SimplifiedMeshGeneric<I>>())?;
        let mut spent = 0u64;
        for &settings in input.levels {
            work.add(1)?;
            spent = spent.checked_add(1).ok_or(Error::SizeOverflow)?;
            let mut child = Workspace::new(Limits {
                max_bytes: limits
                    .max_bytes
                    .checked_sub(live)
                    .ok_or(Error::LimitExceeded)?,
                max_work: limits
                    .max_work
                    .checked_sub(spent)
                    .ok_or(Error::LimitExceeded)?,
            });
            let indices = levels.last().map_or(input.indices, |level| &level.indices);
            let result = match input.attributes {
                None => crate::simplify_generic(indices, input.positions, settings, &mut child),
                Some(a) => crate::simplify_with_attributes_generic(
                    indices,
                    input.positions,
                    a.attributes,
                    a.weights,
                    a.vertex_flags,
                    settings,
                    &mut child,
                ),
            };
            work.add_processing(child.usage().work)?;
            spent = spent
                .checked_add(child.usage().work)
                .ok_or(Error::SizeOverflow)?;
            workspace.charge_owned(
                live.checked_add(child.usage().bytes)
                    .ok_or(Error::SizeOverflow)?,
            )?;
            let level = result?;
            live = live
                .checked_add(checked_bytes(level.indices.capacity(), size_of::<I>())?)
                .ok_or(Error::SizeOverflow)?;
            levels.push(level);
        }
        Ok(levels)
    })();
    workspace.finish(&work);
    result
}

/// Simplify independent LOD chains in parallel, returning one result per chain.
/// Levels within each chain retain the sequential reference's order and budget.
///
/// # Errors
/// Outer `SizeOverflow` or `AllocationFailed` if result-slot allocation fails;
/// per-item errors are those of [`simplify_lod_chain`] and do not cancel other items.
///
/// # Panics
/// Panics if Rayon cannot initialize its thread pool or a worker panics.
pub fn simplify_lod_chains_batch_generic<I: crate::Index>(
    inputs: &[LodChainInputGeneric<'_, I>],
    limits: Limits,
) -> BatchResult<Vec<SimplifiedMeshGeneric<I>>> {
    batch(inputs, limits, simplify_lod_chain_generic)
}

/// An independent buffer to encode. Configuration is local to each descriptor.
#[derive(Clone, Copy, Debug)]
pub enum EncodeInputGeneric<'a, I: crate::Index = u32> {
    /// Initialized vertex bytes, including supplied padding.
    Vertices {
        /// Exact count times stride bytes.
        vertices: &'a [u8],
        /// Number of vertex records.
        count: usize,
        /// Record stride in bytes.
        stride: usize,
        /// Format version and compression level.
        encoding: VertexEncoding,
    },
    /// Triangle-list indices.
    Triangles {
        /// Topology in the selected index width.
        indices: &'a [I],
        /// Format version.
        encoding: IndexEncoding,
    },
    /// General index sequence.
    Sequence {
        /// Values in the selected index width.
        indices: &'a [I],
        /// Format version.
        encoding: IndexEncoding,
    },
}
impl<I: crate::Index> EncodeInputGeneric<'_, I> {
    /// Encode this descriptor through the corresponding sequential codec API.
    ///
    /// # Errors
    /// The selected [`crate::codec`] encoder's errors: `InvalidLayout`, `InvalidTopology`,
    /// `SizeOverflow`, `LimitExceeded` or `AllocationFailed`, for invalid input or exhausted resources.
    pub fn encode_generic(self, workspace: &mut Workspace) -> Result<Vec<u8>, Error> {
        match self {
            Self::Vertices {
                vertices,
                count,
                stride,
                encoding,
            } => codec::encode_vertex_buffer(vertices, count, stride, encoding, workspace),
            Self::Triangles { indices, encoding } => {
                codec::encode_index_buffer_generic(indices, encoding, workspace)
            }
            Self::Sequence { indices, encoding } => {
                codec::encode_index_sequence_generic(indices, encoding, workspace)
            }
        }
    }
}

/// Encode vertex, triangle and sequence buffers in input order.
///
/// # Errors
/// Outer `SizeOverflow` or `AllocationFailed` if result-slot allocation fails;
/// per-item errors are those of [`EncodeInput::encode`] and do not cancel other items.
///
/// # Panics
/// Panics if Rayon cannot initialize its thread pool or a worker panics.
pub fn encode_buffers_batch_generic<I: crate::Index>(
    inputs: &[EncodeInputGeneric<'_, I>],
    limits: Limits,
) -> BatchResult<Vec<u8>> {
    batch(inputs, limits, |input, workspace| {
        input.encode_generic(workspace)
    })
}

/// One checked EXT buffer view and its compressed bytes.
#[derive(Clone, Copy, Debug)]
pub struct DecodeInput<'a> {
    /// Mode, filter, count and stride; construct through [`BufferView::new`].
    pub view: BufferView,
    /// Compressed source bytes.
    pub source: &'a [u8],
}

/// Decode complete independent EXT views through [`BufferView::decode`].
///
/// # Errors
/// Outer `SizeOverflow` or `AllocationFailed` if result-slot allocation fails;
/// per-item errors are those of [`crate::codec::BufferView::decode`] and do not cancel other items.
///
/// # Panics
/// Panics if Rayon cannot initialize its thread pool or a worker panics.
pub fn decode_buffer_views_batch(
    inputs: &[DecodeInput<'_>],
    limits: Limits,
) -> BatchResult<Vec<u8>> {
    batch(inputs, limits, |input, workspace| {
        input.view.decode(input.source, workspace)
    })
}

/// Sequential builder selection for a meshlet batch item.
#[derive(Clone, Copy, Debug)]
pub enum MeshletBuilder {
    /// Consume triangles in source order; only the position count is used.
    Scan,
    /// Standard connectivity and cone-aware builder.
    Standard {
        /// Cone score weight.
        cone_weight: f32,
    },
    /// Flexible connectivity builder.
    Flex {
        /// Minimum triangles per meshlet.
        min_triangles: usize,
        /// Cone score weight.
        cone_weight: f32,
        /// Spatial split factor.
        split_factor: f32,
    },
    /// Surface-area heuristic spatial builder.
    Spatial {
        /// Minimum triangles per meshlet.
        min_triangles: usize,
        /// Fill weight.
        fill_weight: f32,
    },
}

/// Mesh and builder settings for one independent clustering operation.
#[derive(Clone, Copy, Debug)]
pub struct MeshletInputGeneric<'a, I: crate::Index = u32> {
    /// Triangle topology.
    pub indices: &'a [I],
    /// Full vertex positions.
    pub positions: Positions<'a>,
    /// Maximum vertices and triangles per meshlet.
    pub settings: MeshletSettings,
    /// Sequential algorithm and its parameters.
    pub builder: MeshletBuilder,
}
impl<I: crate::Index> MeshletInputGeneric<'_, I> {
    /// Run this descriptor through the selected sequential meshlet builder.
    ///
    /// # Errors
    /// Same errors as [`crate::build_meshlets`]. The selected builder validates its own parameters.
    pub fn build_generic(self, workspace: &mut Workspace) -> Result<MeshletsGeneric<I>, Error> {
        let Self {
            indices,
            positions,
            settings,
            builder,
        } = self;
        match builder {
            MeshletBuilder::Scan => {
                crate::build_meshlets_scan_generic(indices, positions.len(), settings, workspace)
            }
            MeshletBuilder::Standard { cone_weight } => {
                crate::build_meshlets_generic(indices, positions, settings, cone_weight, workspace)
            }
            MeshletBuilder::Flex {
                min_triangles,
                cone_weight,
                split_factor,
            } => crate::build_meshlets_flex_generic(
                indices,
                positions,
                settings,
                min_triangles,
                cone_weight,
                split_factor,
                workspace,
            ),
            MeshletBuilder::Spatial {
                min_triangles,
                fill_weight,
            } => crate::build_meshlets_spatial_generic(
                indices,
                positions,
                settings,
                min_triangles,
                fill_weight,
                workspace,
            ),
        }
    }
}

/// Build meshlets for independent meshes, preserving descriptors and packed bytes.
///
/// # Errors
/// Outer `SizeOverflow` or `AllocationFailed` if result-slot allocation fails;
/// per-item errors are those of [`MeshletInput::build`] and do not cancel other items.
///
/// # Panics
/// Panics if Rayon cannot initialize its thread pool or a worker panics.
pub fn build_meshlets_batch_generic<I: crate::Index>(
    inputs: &[MeshletInputGeneric<'_, I>],
    limits: Limits,
) -> BatchResult<MeshletsGeneric<I>> {
    batch(inputs, limits, |input, workspace| {
        input.build_generic(workspace)
    })
}

/// One independent cluster-LOD build, including its exclusive mutable positions.
#[cfg(feature = "clusterlod")]
pub struct ClusterLodInputGeneric<'a, I: crate::Index = u32> {
    /// Complete pinned-demo configuration.
    pub config: crate::clusterlod::Config,
    /// Mesh input; dilation can mutate positions even when this item fails.
    pub mesh: crate::clusterlod::MeshGeneric<'a, I>,
}

/// Build independent cluster-LOD DAGs in parallel using [`crate::clusterlod::build`].
///
/// Each mesh retains the demo's group/cluster order and refinement identifiers.
/// Dilation has the same partial-mutation contract as the sequential builder.
/// Groups within a mesh stay sequential: shared dilation and later refinement
/// consume earlier results. Parallelism is across meshes with disjoint positions.
///
/// # Errors
/// Outer `SizeOverflow` or `AllocationFailed` if result-slot allocation fails;
/// per-item errors are those of [`crate::clusterlod::build`] and do not cancel other items.
///
/// # Panics
/// Panics if Rayon cannot initialize its thread pool or a worker panics.
#[cfg(feature = "clusterlod")]
pub fn build_cluster_lod_batch_generic<I: crate::Index>(
    inputs: &mut [ClusterLodInputGeneric<'_, I>],
    limits: Limits,
) -> BatchResult<Vec<crate::clusterlod::GroupOutputGeneric<I>>> {
    let mut result = slots(inputs.len())?;
    result
        .par_iter_mut()
        .zip(inputs.par_iter_mut())
        .for_each(|(out, input)| {
            *out = item(limits, |workspace| {
                crate::clusterlod::build_generic(
                    input.config,
                    crate::clusterlod::MeshGeneric {
                        indices: input.mesh.indices,
                        positions: &mut *input.mesh.positions,
                        attributes: input.mesh.attributes,
                        vertex_lock: input.mesh.vertex_lock,
                        attribute_weights: input.mesh.attribute_weights,
                        attribute_protect_mask: input.mesh.attribute_protect_mask,
                    },
                    workspace,
                )
            });
        });
    Ok(result)
}

/// One independent cluster-LOD spatial forest.
#[cfg(feature = "clusterlod")]
#[derive(Clone, Copy, Debug)]
pub struct ClusterHierarchyInput<'a> {
    /// Groups in sequential callback order.
    pub groups: &'a [crate::clusterlod::Group],
    /// Maximum children per node, at least two.
    pub node_width: usize,
    /// Number of nonempty DAG levels.
    pub level_count: usize,
}

/// Build independent spatial forests using [`crate::clusterlod::build_hierarchy`].
///
/// Small batches (at most 256 total groups and 2048 group-by-level visits) run
/// sequentially on the calling thread to avoid dispatch overhead. Larger batches
/// use the current Rayon pool. Ordering, workspace reset and per-item limits are
/// identical on both paths. This cutoff is a workload heuristic, not a speed guarantee.
///
/// # Errors
/// Outer `SizeOverflow` or `AllocationFailed` if result-slot allocation fails;
/// per-item errors are those of [`crate::clusterlod::build_hierarchy`] and do not cancel other items.
///
/// # Panics
/// Panics if Rayon cannot initialize its thread pool or a worker panics.
#[cfg(feature = "clusterlod")]
pub fn build_cluster_hierarchies_batch(
    inputs: &[ClusterHierarchyInput<'_>],
    limits: Limits,
) -> BatchResult<Vec<crate::clusterlod::Node>> {
    // D158: the measured 231-group/1848-visit family does not amortize
    // Rayon dispatch. Saturate this heuristic; validation stays per item.
    let (groups, visits) = inputs.iter().fold((0usize, 0usize), |(groups, visits), i| {
        (
            groups.saturating_add(i.groups.len()),
            visits.saturating_add(i.groups.len().saturating_mul(i.level_count.max(1))),
        )
    });
    let run = |input: &ClusterHierarchyInput<'_>, workspace: &mut Workspace| {
        crate::clusterlod::build_hierarchy(
            input.groups,
            input.node_width,
            input.level_count,
            workspace,
        )
    };
    if groups <= 256 && visits <= 2048 {
        let mut result = slots(inputs.len())?;
        for (out, input) in result.iter_mut().zip(inputs) {
            *out = item(limits, |workspace| run(input, workspace));
        }
        Ok(result)
    } else {
        batch(inputs, limits, run)
    }
}

/// One canonical caller-buffer DAG build, including exclusive positions and output.
#[cfg(feature = "clusterlod")]
pub struct DagBuildInputGeneric<'a, I: crate::Index = u32> {
    /// Complete pinned-demo configuration.
    pub config: crate::clusterlod::Config,
    /// Mesh input; dilation may mutate positions even on failure.
    pub mesh: crate::clusterlod::MeshGeneric<'a, I>,
    /// Maximum children per spatial node, at least two.
    pub node_width: usize,
    /// Caller storage; publish only successful returned prefixes.
    pub destination: crate::clusterlod::DagBuffersGeneric<'a, I>,
}

/// Build independent DAGs into caller buffers, retaining ordered item results.
///
/// Slot allocation fails before any mesh executes. Item errors do not cancel
/// peers. Each worker uses one workspace; each item shares its limits across
/// construction, flattening and hierarchy. Caller buffers, result slots and
/// Rayon infrastructure are excluded; there is no aggregate memory guarantee.
/// Late errors can write output prefixes and mutate positions through dilation.
///
/// # Errors
/// Outer `SizeOverflow` or `AllocationFailed` if result-slot allocation fails;
/// per-item errors are those of [`crate::clusterlod::build_dag_into`] and do not cancel other items.
///
/// # Panics
/// Panics if Rayon cannot initialize its thread pool or a worker panics.
#[cfg(feature = "clusterlod")]
pub fn build_dags_batch_into_generic<I: crate::Index>(
    inputs: &mut [DagBuildInputGeneric<'_, I>],
    limits: Limits,
) -> BatchResult<crate::clusterlod::DagCounts> {
    let mut result = slots(inputs.len())?;
    result
        .par_iter_mut()
        .zip(inputs.par_iter_mut())
        .for_each(|(out, input)| {
            *out = item(limits, |workspace| {
                crate::clusterlod::build_dag_into_generic(
                    input.config,
                    crate::clusterlod::MeshGeneric {
                        indices: input.mesh.indices,
                        positions: &mut *input.mesh.positions,
                        attributes: input.mesh.attributes,
                        vertex_lock: input.mesh.vertex_lock,
                        attribute_weights: input.mesh.attribute_weights,
                        attribute_protect_mask: input.mesh.attribute_protect_mask,
                    },
                    input.node_width,
                    crate::clusterlod::DagBuffersGeneric {
                        vertex_source: &mut *input.destination.vertex_source,
                        vertex_positions: &mut *input.destination.vertex_positions,
                        indices: &mut *input.destination.indices,
                        clusters: &mut *input.destination.clusters,
                        groups: &mut *input.destination.groups,
                        nodes: &mut *input.destination.nodes,
                    },
                    workspace,
                )
            });
        });
    Ok(result)
}

/// The source-compatible u32 entry point; see [`simplify_lod_chain_generic`] for selectable index widths.
#[inline]
pub fn simplify_lod_chain(
    input: &LodChainInput<'_>,
    workspace: &mut Workspace,
) -> Result<Vec<SimplifiedMesh>, Error> {
    simplify_lod_chain_generic::<u32>(input, workspace)
}

/// The source-compatible u32 entry point; see [`simplify_lod_chains_batch_generic`] for selectable index widths.
#[inline]
pub fn simplify_lod_chains_batch(
    inputs: &[LodChainInput<'_>],
    limits: Limits,
) -> BatchResult<Vec<SimplifiedMesh>> {
    simplify_lod_chains_batch_generic::<u32>(inputs, limits)
}

/// The source-compatible u32 entry point; see [`encode_buffers_batch_generic`] for selectable index widths.
#[inline]
pub fn encode_buffers_batch(inputs: &[EncodeInput<'_>], limits: Limits) -> BatchResult<Vec<u8>> {
    batch(inputs, limits, |input, workspace| input.encode(workspace))
}

/// The source-compatible u32 entry point; see [`build_meshlets_batch_generic`] for selectable index widths.
#[inline]
pub fn build_meshlets_batch(inputs: &[MeshletInput<'_>], limits: Limits) -> BatchResult<Meshlets> {
    build_meshlets_batch_generic::<u32>(inputs, limits)
}

/// The source-compatible u32 entry point; see [`build_cluster_lod_batch_generic`] for selectable index widths.
#[cfg(feature = "clusterlod")]
#[inline]
pub fn build_cluster_lod_batch(
    inputs: &mut [ClusterLodInput<'_>],
    limits: Limits,
) -> BatchResult<Vec<crate::clusterlod::GroupOutput>> {
    build_cluster_lod_batch_generic::<u32>(inputs, limits)
}

/// The source-compatible u32 entry point; see [`build_dags_batch_into_generic`] for selectable index widths.
#[cfg(feature = "clusterlod")]
#[inline]
pub fn build_dags_batch_into(
    inputs: &mut [DagBuildInput<'_>],
    limits: Limits,
) -> BatchResult<crate::clusterlod::DagCounts> {
    build_dags_batch_into_generic::<u32>(inputs, limits)
}

impl MeshletInputGeneric<'_, u32> {
    /// Source-compatible u32 operation.
    #[inline]
    pub fn build(self, workspace: &mut Workspace) -> Result<Meshlets, Error> {
        self.build_generic(workspace)
    }
}

/// The source-compatible u32 form of [`LodChainInputGeneric`].
pub type LodChainInput<'a> = LodChainInputGeneric<'a, u32>;

/// The source-compatible u32 form of [`MeshletInputGeneric`].
pub type MeshletInput<'a> = MeshletInputGeneric<'a, u32>;

/// The source-compatible u32 form of [`ClusterLodInputGeneric`].
#[cfg(feature = "clusterlod")]
pub type ClusterLodInput<'a> = ClusterLodInputGeneric<'a, u32>;

/// The source-compatible u32 form of [`DagBuildInputGeneric`].
#[cfg(feature = "clusterlod")]
pub type DagBuildInput<'a> = DagBuildInputGeneric<'a, u32>;

/// An independent u32 buffer to encode; the source-compatible descriptor.
#[derive(Clone, Copy, Debug)]
pub enum EncodeInput<'a> {
    /// Initialized vertex bytes, including supplied padding.
    Vertices {
        /// Exact count times stride bytes.
        vertices: &'a [u8],
        /// Number of vertex records.
        count: usize,
        /// Record stride in bytes.
        stride: usize,
        /// Format version and compression level.
        encoding: VertexEncoding,
    },
    /// Triangle-list indices.
    Triangles {
        /// Topology in u32 form.
        indices: &'a [u32],
        /// Format version.
        encoding: IndexEncoding,
    },
    /// General index sequence.
    Sequence {
        /// Values in u32 form.
        indices: &'a [u32],
        /// Format version.
        encoding: IndexEncoding,
    },
}

impl EncodeInput<'_> {
    /// Encode this u32 descriptor through the shared generic implementation.
    pub fn encode(self, workspace: &mut Workspace) -> Result<Vec<u8>, Error> {
        let generic = match self {
            Self::Vertices {
                vertices,
                count,
                stride,
                encoding,
            } => EncodeInputGeneric::Vertices {
                vertices,
                count,
                stride,
                encoding,
            },
            Self::Triangles { indices, encoding } => {
                EncodeInputGeneric::Triangles { indices, encoding }
            }
            Self::Sequence { indices, encoding } => {
                EncodeInputGeneric::Sequence { indices, encoding }
            }
        };
        generic.encode_generic(workspace)
    }
}
