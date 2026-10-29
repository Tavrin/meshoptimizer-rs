use crate::{
    simplify, simplify_scale, simplify_with_attributes, Attributes, Error, Positions,
    SimplifyOptions, SimplifySettings, VertexFlags, Workspace,
};
use alloc::vec::Vec;

/// Level targets, expressed as index counts or fractions of the source index count.
#[derive(Clone, Copy, Debug)]
pub enum LodTargets<'a> {
    /// Strictly decreasing counts, smaller than the source index count.
    Counts(&'a [usize]),
    /// Strictly decreasing finite fractions in [0, 1); counts are rounded down to triangles.
    Ratios(&'a [f32]),
}
/// A single error cap or one cap per level, in the units selected by the simplifier options.
#[derive(Clone, Copy, Debug)]
pub enum LodErrors<'a> {
    /// One cap for every level.
    Uniform(f32),
    /// One cap per target.
    PerLevel(&'a [f32]),
}
/// Optional attributes passed unchanged to the simplifier.
#[derive(Clone, Copy, Debug)]
pub struct LodChainAttributes<'a> {
    /// Attribute view.
    pub attributes: Attributes<'a>,
    /// One weight per attribute component.
    pub weights: &'a [f32],
    /// Optional per-vertex flags.
    pub vertex_flags: Option<&'a [VertexFlags]>,
}
/// Parameters for the LOD chain builder.
#[derive(Clone, Copy, Debug)]
pub struct LodChainOptions<'a> {
    /// Target index counts or source-relative ratios.
    pub targets: LodTargets<'a>,
    /// Error limits.
    pub errors: LodErrors<'a>,
    /// Simplification flags.
    pub simplify_options: SimplifyOptions,
    /// Optional attribute inputs.
    pub attributes: Option<LodChainAttributes<'a>>,
    /// Use the original source for every level instead of the preceding level (default false).
    pub independent: bool,
}
impl<'a> LodChainOptions<'a> {
    /// Create a successive chain with default simplification flags and no attributes.
    pub fn new(targets: LodTargets<'a>, errors: LodErrors<'a>) -> Self {
        Self {
            targets,
            errors,
            simplify_options: SimplifyOptions::EMPTY,
            attributes: None,
            independent: false,
        }
    }
}
/// One simplification result. Errors describe this call, not cumulative chain error.
#[derive(Debug)]
#[non_exhaustive]
pub struct LodLevel {
    /// Indices referencing the original vertices.
    pub indices: Vec<u32>,
    /// Relative error returned or converted from absolute units.
    pub relative_error: f32,
    /// Absolute error in position units.
    pub absolute_error: f32,
    /// Number of triangles.
    pub triangle_count: usize,
    /// Whether the requested target was reached. A missed target can also result from topology constraints.
    pub target_reached: bool,
    /// The simplifier encountered the error limit when it stopped before the target.
    pub stopped_by_error: bool,
}
/// The generated levels in target order.
#[derive(Debug)]
#[non_exhaustive]
pub struct LodChain {
    /// Generated levels.
    pub levels: Vec<LodLevel>,
}
/// Build a LOD chain. This extension is not in C++ meshoptimizer.
///
/// Ratios are relative to the source, and rounded down to whole triangles. In chain
/// mode a target is clamped to the previous result's size when necessary. Scale is
/// the full position extent, or the referenced subset extent for sparse mode.
///
/// ```
/// use meshoptimizer_rs::{build_lod_chain, LodChainOptions, LodTargets, LodErrors, Positions, SimplifyOptions, Workspace};
/// let p = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.], [1.,1.,0.]];
/// let options = LodChainOptions { targets: LodTargets::Counts(&[3]), errors: LodErrors::Uniform(1.0), simplify_options: SimplifyOptions::EMPTY, attributes: None, independent: false };
/// let chain = build_lod_chain(&[0,1,2,2,1,3], Positions::from_packed(&p), &options, &mut Workspace::default())?;
/// assert_eq!(chain.levels.len(), 1);
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
/// # Errors
/// Returns `InvalidParameter` for invalid targets or caps, and propagates simplification, allocation and limit errors.
pub fn build_lod_chain(
    indices: &[u32],
    positions: Positions<'_>,
    options: &LodChainOptions<'_>,
    workspace: &mut Workspace,
) -> Result<LodChain, Error> {
    let len = match options.targets {
        LodTargets::Counts(v) => v.len(),
        LodTargets::Ratios(v) => v.len(),
    };
    if len == 0 {
        return Err(Error::InvalidParameter);
    }
    if let LodErrors::PerLevel(v) = options.errors {
        if v.len() != len {
            return Err(Error::InvalidParameter);
        }
    }
    let target = |i: usize| match options.targets {
        LodTargets::Counts(v) => v[i],
        LodTargets::Ratios(v) => ((v[i] as f64 * (indices.len() / 3) as f64) as usize) * 3,
    };
    let cap = |i: usize| match options.errors {
        LodErrors::Uniform(v) => v,
        LodErrors::PerLevel(v) => v[i],
    };
    let mut prev = indices.len();
    let mut prev_ratio = 1.0f32;
    for i in 0..len {
        if let LodTargets::Ratios(v) = options.targets {
            let r = v[i];
            if !r.is_finite() || r < 0.0 || r >= prev_ratio {
                return Err(Error::InvalidParameter);
            }
            prev_ratio = r;
        }
        let t = target(i);
        let e = cap(i);
        if t >= prev || !e.is_finite() || e < 0.0 {
            return Err(Error::InvalidParameter);
        }
        prev = t;
    }
    let scale = simplify_scale(positions)?;
    let mut levels: Vec<LodLevel> = Vec::new();
    let metadata = len
        .checked_mul(core::mem::size_of::<LodLevel>())
        .ok_or(Error::SizeOverflow)?;
    if metadata > workspace.limits().max_bytes {
        return Err(Error::LimitExceeded);
    }
    levels
        .try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    let mut used = levels
        .capacity()
        .checked_mul(core::mem::size_of::<LodLevel>())
        .ok_or(Error::SizeOverflow)?;
    let mut work = 0u64;
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
        let limits = workspace.limits();
        let remaining = crate::Limits {
            max_bytes: limits
                .max_bytes
                .checked_sub(used)
                .ok_or(Error::LimitExceeded)?,
            max_work: limits
                .max_work
                .checked_sub(work)
                .ok_or(Error::LimitExceeded)?,
        };
        workspace.set_lod_limits(remaining);
        let result = if let Some(a) = options.attributes {
            simplify_with_attributes(
                input,
                positions,
                a.attributes,
                a.weights,
                a.vertex_flags,
                settings,
                workspace,
            )
        } else {
            simplify(input, positions, settings, workspace)
        };
        let stopped_by_error = workspace.lod_error_limited;
        let usage = workspace.usage();
        workspace.set_lod_limits(limits);
        work = work.checked_add(usage.work).ok_or(Error::SizeOverflow)?;
        let result = result?;
        let scale = if options.simplify_options.contains(SimplifyOptions::SPARSE) {
            let mut lo = [f32::INFINITY; 3];
            let mut hi = [f32::NEG_INFINITY; 3];
            for &index in input {
                let p = positions
                    .get(index as usize)
                    .ok_or(Error::IndexOutOfBounds)?;
                for k in 0..3 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
            if input.is_empty() {
                0.0
            } else {
                (hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2])
            }
        } else {
            scale
        };
        used = used
            .checked_add(
                result
                    .indices
                    .capacity()
                    .checked_mul(4)
                    .ok_or(Error::SizeOverflow)?,
            )
            .ok_or(Error::SizeOverflow)?;
        let (relative_error, absolute_error) = if options
            .simplify_options
            .contains(SimplifyOptions::ERROR_ABSOLUTE)
        {
            (
                if scale == 0.0 {
                    0.0
                } else {
                    result.error / scale
                },
                result.error,
            )
        } else {
            (result.error, result.error * scale)
        };
        if !relative_error.is_finite() || !absolute_error.is_finite() {
            return Err(Error::NumericalFailure);
        }
        levels.push(LodLevel {
            triangle_count: result.indices.len() / 3,
            target_reached: result.indices.len() <= target(i),
            stopped_by_error: result.indices.len() > target(i) && stopped_by_error,
            indices: result.indices,
            relative_error,
            absolute_error,
        });
    }
    Ok(LodChain { levels })
}
