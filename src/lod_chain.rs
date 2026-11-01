use crate::{
    simplify_generic, simplify_scale, simplify_with_attributes_generic, Attributes, Error, Limits,
    Positions, SimplifyOptions, SimplifySettings, Usage, VertexFlags, Workspace,
};
use alloc::vec::Vec;
use core::fmt;

/// Level targets, expressed as index counts or fractions of the source index count.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum LodTargets<'a> {
    /// Strictly decreasing counts, smaller than the source index count.
    Counts(&'a [usize]),
    /// Strictly decreasing finite fractions in [0, 1) of the source triangle count.
    ///
    /// Each ratio becomes `floor(ratio * source_triangles) * 3` indices, computed in
    /// `f64`. The ratios must decrease strictly, but the rounded counts only need to
    /// be non-increasing: on a small mesh two neighbouring ratios may round to the
    /// same count, which yields a level equal in size to the previous one.
    Ratios(&'a [f32]),
}
/// A single error cap or one cap per level, in the units selected by the simplifier options.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum LodErrors<'a> {
    /// One cap for every level.
    Uniform(f32),
    /// One cap per target.
    PerLevel(&'a [f32]),
}
/// Attribute inputs passed unchanged to the simplifier.
///
/// Per-vertex flags (locks) are set on [`LodChainOptions`] and apply with or
/// without attributes.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LodChainAttributes<'a> {
    /// Attribute view.
    pub attributes: Attributes<'a>,
    /// One weight per attribute component.
    pub weights: &'a [f32],
}
impl<'a> LodChainAttributes<'a> {
    /// Attribute view and one weight per component.
    pub fn new(attributes: Attributes<'a>, weights: &'a [f32]) -> Self {
        Self {
            attributes,
            weights,
        }
    }
}
/// Parameters for the LOD chain builder. Build one with [`Self::new`] and the
/// `with_*` setters; the struct is `#[non_exhaustive]` so options can be added.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LodChainOptions<'a> {
    /// Target index counts or source-relative ratios.
    pub targets: LodTargets<'a>,
    /// Error limits.
    pub errors: LodErrors<'a>,
    /// Simplification flags.
    pub simplify_options: SimplifyOptions,
    /// Optional attribute inputs.
    pub attributes: Option<LodChainAttributes<'a>>,
    /// Optional per-vertex flags (for example locks), one per vertex. They apply
    /// to every level, with or without attributes.
    pub vertex_flags: Option<&'a [VertexFlags]>,
    /// Use the original source for every level instead of the preceding level (default false).
    pub independent: bool,
}
impl<'a> LodChainOptions<'a> {
    /// Create a successive chain with default simplification flags, no attributes
    /// and no vertex flags.
    pub fn new(targets: LodTargets<'a>, errors: LodErrors<'a>) -> Self {
        Self {
            targets,
            errors,
            simplify_options: SimplifyOptions::EMPTY,
            attributes: None,
            vertex_flags: None,
            independent: false,
        }
    }
    /// Set the simplification flags.
    #[must_use]
    pub fn with_simplify_options(mut self, options: SimplifyOptions) -> Self {
        self.simplify_options = options;
        self
    }
    /// Simplify with attributes.
    #[must_use]
    pub fn with_attributes(mut self, attributes: LodChainAttributes<'a>) -> Self {
        self.attributes = Some(attributes);
        self
    }
    /// Apply per-vertex flags to every level, with or without attributes.
    #[must_use]
    pub fn with_vertex_flags(mut self, flags: &'a [VertexFlags]) -> Self {
        self.vertex_flags = Some(flags);
        self
    }
    /// Simplify every level from the original source (`true`) or from the
    /// preceding level (`false`, the default).
    #[must_use]
    pub fn with_independent(mut self, independent: bool) -> Self {
        self.independent = independent;
        self
    }
}
/// One simplification result. Errors describe this call, not cumulative chain error.
///
/// The index buffer is an owned `Vec<u32>` per level; other outputs (meshlets,
/// clusters) would be separate APIs rather than fields here.
#[derive(Debug)]
#[non_exhaustive]
pub struct LodLevelGeneric<I: crate::Index = u32> {
    /// Indices referencing the original vertices.
    pub indices: Vec<I>,
    /// The error the simplifier returned for this level, unmodified. It is per
    /// level: in successive mode it is relative to this level's input (the
    /// previous level), not to the source and not cumulative. With attributes it
    /// includes the attribute terms. Units follow `ERROR_ABSOLUTE`: relative to
    /// `scale` unless that flag is set, absolute position units otherwise.
    pub error: f32,
    /// Position extent (largest axis range) used to normalize this level's error:
    /// all positions, or only the vertices this level's input references with
    /// `SPARSE`.
    pub scale: f32,
    /// Number of triangles.
    pub triangle_count: usize,
    /// Whether the requested target was reached. A missed target can also result from topology constraints.
    pub target_reached: bool,
    /// The simplifier stopped because its next collapse candidate exceeded this
    /// level's error limit before the target was reached. This does not prove
    /// that a looser limit would reach the target; topology can still block it.
    pub stopped_at_error_limit: bool,
}
/// The generated levels in target order.
#[derive(Debug)]
#[non_exhaustive]
pub struct LodChainGeneric<I: crate::Index = u32> {
    /// Generated levels.
    pub levels: Vec<LodLevelGeneric<I>>,
}
/// A failed [`build_lod_chain`] call: the underlying [`Error`] and the level it
/// occurred in.
///
/// The crate's other fallible calls return a bare [`Error`]; a chain has several
/// independent steps, so this wrapper adds only the failing level. The result is
/// atomic: no partial chain accompanies the error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct LodChainError {
    /// Zero-based index into the targets (and into `LodChain::levels`) of the
    /// level that failed, or `None` for a failure of the call as a whole (an empty
    /// or mismatched target list, or the chain metadata allocation).
    pub level: Option<usize>,
    /// The underlying failure.
    pub error: Error,
}
impl fmt::Display for LodChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.level {
            Some(level) => write!(f, "LOD level {level}: {}", self.error),
            None => write!(f, "LOD chain: {}", self.error),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for LodChainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// Whole-call meter: counted work shared by the builder's scans and every level,
/// and peak bytes across the chain's own storage plus workspace scratch.
struct Meter {
    limit: u64,
    spent: u64,
    peak: usize,
}
impl Meter {
    fn charge(&mut self, visits: usize) -> Result<(), Error> {
        let visits = u64::try_from(visits).map_err(|_| Error::SizeOverflow)?;
        self.spent = self.spent.saturating_add(visits);
        if self.spent > self.limit {
            return Err(Error::LimitExceeded);
        }
        Ok(())
    }
}

/// Build a LOD chain. This extension is not in C++ meshoptimizer.
///
/// Each level is one [`simplify`](crate::simplify) (or [`simplify_with_attributes`](crate::simplify_with_attributes)) call, so its
/// indices, error and limits behave exactly as the direct call on that level's
/// input. Ratios are relative to the source, and rounded down to whole triangles
/// (see [`LodTargets::Ratios`]). In chain mode a target is clamped to the previous
/// result's size when necessary.
///
/// **Limits and usage.** The workspace limits apply to the whole call: counted
/// work covers the target and cap validation (one visit per level), every level's
/// simplification, and the scale scans (one visit per vertex of the mesh, once, or
/// per referenced index of each level's input with `SPARSE`). Bytes cover the chain
/// metadata and every retained level output together with the workspace scratch.
/// [`Workspace::usage`] afterwards reports the whole call's work total and peak
/// bytes, on success and on failure.
///
/// **Atomicity.** On failure no partial chain is returned; [`LodChainError::level`]
/// names the failing level.
///
/// ```
/// use meshoptimizer_rs::{build_lod_chain, LodChainOptions, LodTargets, LodErrors, Positions, Workspace};
/// let p = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.], [1.,1.,0.]];
/// let options = LodChainOptions::new(LodTargets::Counts(&[3]), LodErrors::Uniform(1.0));
/// let chain = build_lod_chain(&[0,1,2,2,1,3], Positions::from_packed(&p), &options, &mut Workspace::default())?;
/// assert_eq!(chain.levels.len(), 1);
/// # Ok::<(), meshoptimizer_rs::LodChainError>(())
/// ```
/// # Errors
/// [`LodChainError`] with `InvalidParameter` for invalid targets or caps, and the
/// simplification, allocation and limit errors of the failing level.
pub fn build_lod_chain_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    options: &LodChainOptions<'_>,
    workspace: &mut Workspace,
) -> Result<LodChainGeneric<I>, LodChainError> {
    let limits = workspace.limits();
    let mut meter = Meter {
        limit: limits.max_work,
        spent: 0,
        peak: 0,
    };
    let result = build(indices, positions, options, workspace, limits, &mut meter);
    workspace.set_lod_limits(limits);
    workspace.set_lod_usage(Usage {
        bytes: meter.peak,
        work: meter.spent,
    });
    result
}

fn build<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    options: &LodChainOptions<'_>,
    workspace: &mut Workspace,
    limits: Limits,
    meter: &mut Meter,
) -> Result<LodChainGeneric<I>, LodChainError> {
    let whole = |error: Error| LodChainError { level: None, error };
    let at = |level: usize| {
        move |error: Error| LodChainError {
            level: Some(level),
            error,
        }
    };
    let len = match options.targets {
        LodTargets::Counts(v) => v.len(),
        LodTargets::Ratios(v) => v.len(),
    };
    if len == 0 {
        return Err(whole(Error::InvalidParameter));
    }
    if let LodErrors::PerLevel(v) = options.errors {
        if v.len() != len {
            return Err(whole(Error::InvalidParameter));
        }
    }
    // One visit per level for the target and cap validation below.
    meter.charge(len).map_err(whole)?;
    let target = |i: usize| match options.targets {
        LodTargets::Counts(v) => v[i],
        // The float-to-integer cast saturates, so a huge ratio only needs the
        // multiplication to saturate too; the order check below then rejects it.
        LodTargets::Ratios(v) => {
            ((v[i] as f64 * (indices.len() / 3) as f64) as usize).saturating_mul(3)
        }
    };
    let cap = |i: usize| match options.errors {
        LodErrors::Uniform(v) => v,
        LodErrors::PerLevel(v) => v[i],
    };
    let mut prev = indices.len();
    let mut prev_ratio = 1.0f32;
    for i in 0..len {
        let t = target(i);
        let e = cap(i);
        let order_ok = match options.targets {
            // Strictly decreasing counts below the source count.
            LodTargets::Counts(_) => t < prev,
            // Ratios must decrease strictly; their rounded counts may coincide.
            LodTargets::Ratios(v) => {
                let r = v[i];
                let ok = r.is_finite() && r >= 0.0 && r < prev_ratio && t <= prev;
                prev_ratio = r;
                ok
            }
        };
        if !order_ok || !e.is_finite() || e < 0.0 {
            return Err(at(i)(Error::InvalidParameter));
        }
        prev = t;
    }
    let mut levels: Vec<LodLevelGeneric<I>> = Vec::new();
    let metadata = len
        .checked_mul(core::mem::size_of::<LodLevelGeneric<I>>())
        .ok_or(Error::SizeOverflow)
        .map_err(whole)?;
    if metadata > limits.max_bytes {
        meter.peak = metadata;
        return Err(whole(Error::LimitExceeded));
    }
    levels
        .try_reserve_exact(len)
        .map_err(|_| whole(Error::AllocationFailed))?;
    // Chain metadata plus every retained level output; workspace scratch is added
    // by each level's own accounting under the reduced limit below.
    let mut used = levels
        .capacity()
        .checked_mul(core::mem::size_of::<LodLevelGeneric<I>>())
        .ok_or(Error::SizeOverflow)
        .map_err(whole)?;
    meter.peak = used;
    let sparse = options.simplify_options.contains(SimplifyOptions::SPARSE);
    let mut dense_scale: Option<f32> = None;
    for i in 0..len {
        let input = if options.independent || i == 0 {
            indices
        } else {
            &levels[i - 1].indices
        };
        let settings = SimplifySettings {
            target_index_count: target(i).min(input.len()),
            target_error: cap(i),
            options: options.simplify_options,
        };
        workspace.set_lod_limits(Limits {
            max_bytes: limits
                .max_bytes
                .checked_sub(used)
                .ok_or(Error::LimitExceeded)
                .map_err(at(i))?,
            max_work: limits
                .max_work
                .checked_sub(meter.spent)
                .ok_or(Error::LimitExceeded)
                .map_err(at(i))?,
        });
        let result = match (options.attributes, options.vertex_flags) {
            (Some(a), flags) => simplify_with_attributes_generic(
                input,
                positions,
                a.attributes,
                a.weights,
                flags,
                settings,
                workspace,
            ),
            (None, Some(flags)) => I::with_input(input, workspace, |input, workspace| {
                crate::simplify::simplify_flagged(input, positions, flags, settings, workspace)
                    .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
            }),
            (None, None) => simplify_generic(input, positions, settings, workspace),
        };
        let error_limited = workspace.lod_error_limited;
        let usage = workspace.usage();
        meter.spent = meter.spent.saturating_add(usage.work);
        meter.peak = meter.peak.max(used.saturating_add(usage.bytes));
        let result = result.map_err(at(i))?;
        // The scale is computed only after the simplifier accepted the input, so
        // it cannot add a failure the direct call does not have: a sparse call
        // never looks at unreferenced vertices, and a dense call has already
        // validated the same finite extent.
        let scale = if sparse {
            meter.charge(input.len()).map_err(at(i))?;
            sparse_scale(input, positions).map_err(at(i))?
        } else if let Some(scale) = dense_scale {
            scale
        } else {
            meter.charge(positions.len()).map_err(at(i))?;
            let scale = simplify_scale(positions).map_err(at(i))?;
            dense_scale = Some(scale);
            scale
        };
        used = used
            .checked_add(
                result
                    .indices
                    .capacity()
                    .checked_mul(core::mem::size_of::<I>())
                    .ok_or(Error::SizeOverflow)
                    .map_err(at(i))?,
            )
            .ok_or(Error::SizeOverflow)
            .map_err(at(i))?;
        levels.push(LodLevelGeneric {
            triangle_count: result.indices.len() / 3,
            target_reached: result.indices.len() <= target(i),
            stopped_at_error_limit: result.indices.len() > target(i) && error_limited,
            indices: result.indices,
            error: result.error,
            scale,
        });
    }
    Ok(LodChainGeneric { levels })
}

/// The largest axis range of the vertices `input` references, as sparse
/// simplification computes it. An index outside `positions` cannot occur after a
/// successful simplify call.
fn sparse_scale<I: crate::Index>(input: &[I], positions: Positions<'_>) -> Result<f32, Error> {
    if input.is_empty() {
        return Ok(0.0);
    }
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for &index in input {
        let p = positions
            .get(index.to_u32() as usize)
            .ok_or(Error::IndexOutOfBounds)?;
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    Ok((hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2]))
}

/// The source-compatible u32 entry point; see [`build_lod_chain_generic`] for selectable index widths.
#[inline]
pub fn build_lod_chain(
    indices: &[u32],
    positions: Positions<'_>,
    options: &LodChainOptions<'_>,
    workspace: &mut Workspace,
) -> Result<LodChain, LodChainError> {
    build_lod_chain_generic::<u32>(indices, positions, options, workspace).map(Into::into)
}

/// The source-compatible u32 form of [`LodLevelGeneric`].
/// One simplification result. Errors describe this call, not cumulative chain error.
///
/// The index buffer is an owned `Vec<u32>` per level; other outputs (meshlets,
/// clusters) would be separate APIs rather than fields here.
#[derive(Debug)]
#[non_exhaustive]
pub struct LodLevel {
    /// Indices referencing the original vertices.
    pub indices: Vec<u32>,
    /// The error the simplifier returned for this level, unmodified. It is per
    /// level: in successive mode it is relative to this level's input (the
    /// previous level), not to the source and not cumulative. With attributes it
    /// includes the attribute terms. Units follow `ERROR_ABSOLUTE`: relative to
    /// `scale` unless that flag is set, absolute position units otherwise.
    pub error: f32,
    /// Position extent (largest axis range) used to normalize this level's error:
    /// all positions, or only the vertices this level's input references with
    /// `SPARSE`.
    pub scale: f32,
    /// Number of triangles.
    pub triangle_count: usize,
    /// Whether the requested target was reached. A missed target can also result from topology constraints.
    pub target_reached: bool,
    /// The simplifier stopped because its next collapse candidate exceeded this
    /// level's error limit before the target was reached. This does not prove
    /// that a looser limit would reach the target; topology can still block it.
    pub stopped_at_error_limit: bool,
}
impl From<LodLevelGeneric<u32>> for LodLevel {
    #[inline]
    fn from(value: LodLevelGeneric<u32>) -> Self {
        Self {
            indices: value.indices,
            error: value.error,
            scale: value.scale,
            triangle_count: value.triangle_count,
            target_reached: value.target_reached,
            stopped_at_error_limit: value.stopped_at_error_limit,
        }
    }
}

impl From<LodLevel> for LodLevelGeneric<u32> {
    #[inline]
    fn from(value: LodLevel) -> Self {
        Self {
            indices: value.indices,
            error: value.error,
            scale: value.scale,
            triangle_count: value.triangle_count,
            target_reached: value.target_reached,
            stopped_at_error_limit: value.stopped_at_error_limit,
        }
    }
}

/// The source-compatible u32 form of [`LodChainGeneric`].
/// The generated levels in target order.
#[derive(Debug)]
#[non_exhaustive]
pub struct LodChain {
    /// Generated levels.
    pub levels: Vec<LodLevel>,
}
impl From<LodChainGeneric<u32>> for LodChain {
    #[inline]
    fn from(value: LodChainGeneric<u32>) -> Self {
        Self {
            levels: value.levels.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<LodChain> for LodChainGeneric<u32> {
    #[inline]
    fn from(value: LodChain) -> Self {
        Self {
            levels: value.levels.into_iter().map(Into::into).collect(),
        }
    }
}
