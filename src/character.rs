//! Character simplification, an extension not in C++ meshoptimizer.
use crate::{
    Attributes, Error, Limits, LodChain, LodChainAttributes, LodChainOptions, LodQuality,
    LodQualityMesh, LodQualityOptions, Positions, SimplifiedMesh, SimplifySettings, Usage,
    Workspace,
};
use alloc::vec::Vec;

/// Up to eight linear blend skinning influences; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CharacterSkin<'a> {
    /// Flat vertex-major joint indices. Duplicate joints are summed.
    pub joints: &'a [u32],
    /// Flat vertex-major nonnegative weights, summing to one within 0.001.
    pub weights: &'a [f32],
    /// Four or eight entries per vertex (zero weights pad unused entries).
    pub influences: usize,
    /// One finite nonnegative importance per bone, including unused bones.
    pub importance: &'a [f32],
}
impl<'a> CharacterSkin<'a> {
    /// Borrow skin data. Validation occurs when simplifying.
    pub fn new(
        joints: &'a [u32],
        weights: &'a [f32],
        influences: usize,
        importance: &'a [f32],
    ) -> Self {
        Self {
            joints,
            weights,
            influences,
            importance,
        }
    }
}
/// Borrowed character data; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CharacterInputs<'a> {
    /// Optional skin; absent for morph-only meshes.
    pub skin: Option<CharacterSkin<'a>>,
    /// One vertex-length delta array per full-weight blend shape.
    pub blend_shapes: &'a [&'a [[f32; 3]]],
    /// Optional sample poses: one model-space affine skin matrix per bone.
    /// Matrices already include inverse bind transforms; rows multiply [x,y,z,1].
    /// These poses are measured separately, without blend shapes.
    pub joint_poses: &'a [&'a [[[f32; 4]; 3]]],
}
impl<'a> CharacterInputs<'a> {
    /// Empty deformation inputs.
    pub fn new() -> Self {
        Self {
            skin: None,
            blend_shapes: &[],
            joint_poses: &[],
        }
    }
}
impl Default for CharacterInputs<'_> {
    fn default() -> Self {
        Self::new()
    }
}
/// Metric weights; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CharacterOptions {
    /// Global multiplier on each bone channel (also multiplied by importance).
    pub skin_weight: f32,
    /// Multiplier on normalized full-target position channels.
    pub blend_shape_weight: f32,
}
impl Default for CharacterOptions {
    fn default() -> Self {
        Self {
            skin_weight: 1.0,
            blend_shape_weight: 1.0,
        }
    }
}
/// Character levels and sampled pose quality; not in C++ meshoptimizer.
#[derive(Debug)]
#[non_exhaustive]
pub struct CharacterLodChain {
    /// Indices reference the original vertices and deformation streams.
    pub chain: LodChain,
    /// Pose-major, then level-major F9 reports: bind, each blend shape, each joint pose.
    pub quality: Vec<Vec<LodQuality>>,
}

// Whole-call limits include prepared attributes, retained levels, pose buffers,
// report storage, and every nested algorithm's scratch and work.
struct Meter {
    limits: Limits,
    bytes: usize,
    peak: usize,
    work: u64,
}
impl Meter {
    fn charge(&mut self, n: usize) -> Result<(), Error> {
        self.work = self
            .work
            .checked_add(u64::try_from(n).map_err(|_| Error::SizeOverflow)?)
            .ok_or(Error::SizeOverflow)?;
        if self.work > self.limits.max_work {
            return Err(Error::LimitExceeded);
        }
        Ok(())
    }
    fn alloc<T>(&mut self, n: usize, ws: &mut Workspace) -> Result<Vec<T>, Error> {
        let bytes = n
            .checked_mul(core::mem::size_of::<T>())
            .ok_or(Error::SizeOverflow)?;
        let total = self.bytes.checked_add(bytes).ok_or(Error::SizeOverflow)?;
        ws.charge_owned(total)?;
        let mut v = Vec::new();
        v.try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        self.bytes = self
            .bytes
            .checked_add(
                v.capacity()
                    .checked_mul(core::mem::size_of::<T>())
                    .ok_or(Error::SizeOverflow)?,
            )
            .ok_or(Error::SizeOverflow)?;
        ws.charge_owned(self.bytes)?;
        self.peak = self.peak.max(ws.usage().bytes).max(self.bytes);
        Ok(v)
    }
    fn call<T>(
        &mut self,
        ws: &mut Workspace,
        f: impl FnOnce(&mut Workspace) -> Result<T, Error>,
    ) -> Result<T, Error> {
        ws.set_lod_limits(Limits {
            max_bytes: self
                .limits
                .max_bytes
                .checked_sub(self.bytes)
                .ok_or(Error::LimitExceeded)?,
            max_work: self
                .limits
                .max_work
                .checked_sub(self.work)
                .ok_or(Error::LimitExceeded)?,
        });
        let r = f(ws);
        let u = ws.usage();
        ws.set_lod_limits(self.limits);
        self.work = self.work.checked_add(u.work).ok_or(Error::SizeOverflow)?;
        self.peak = self
            .peak
            .max(self.bytes.checked_add(u.bytes).ok_or(Error::SizeOverflow)?);
        r
    }
    fn retain<T>(&mut self, v: &Vec<T>) -> Result<(), Error> {
        self.bytes = self
            .bytes
            .checked_add(
                v.capacity()
                    .checked_mul(core::mem::size_of::<T>())
                    .ok_or(Error::SizeOverflow)?,
            )
            .ok_or(Error::SizeOverflow)?;
        Ok(())
    }
}
fn run<T>(
    ws: &mut Workspace,
    f: impl FnOnce(&mut Meter, &mut Workspace) -> Result<T, Error>,
) -> Result<T, Error> {
    let _ = ws.begin();
    let mut m = Meter {
        limits: ws.limits(),
        bytes: 0,
        peak: 0,
        work: 0,
    };
    let r = f(&mut m, ws);
    ws.set_lod_limits(m.limits);
    ws.set_lod_usage(Usage {
        bytes: m.peak,
        work: m.work,
    });
    r
}
fn finite(x: f32) -> Result<f32, Error> {
    if x.is_finite() {
        Ok(x)
    } else {
        Err(Error::NumericalFailure)
    }
}
fn prepare(
    p: Positions<'_>,
    data: &CharacterInputs<'_>,
    o: &CharacterOptions,
    m: &mut Meter,
    ws: &mut Workspace,
) -> Result<(Vec<f32>, usize), Error> {
    if !o.skin_weight.is_finite()
        || o.skin_weight < 0.0
        || !o.blend_shape_weight.is_finite()
        || o.blend_shape_weight < 0.0
    {
        return Err(Error::InvalidParameter);
    }
    let n = p.len();
    let bones = data.skin.map_or(0, |s| s.importance.len());
    let width = bones
        .checked_add(
            data.blend_shapes
                .len()
                .checked_mul(3)
                .ok_or(Error::SizeOverflow)?,
        )
        .ok_or(Error::SizeOverflow)?;
    if width > 32 {
        return Err(Error::InvalidLayout);
    }
    m.charge(
        n.checked_mul(width.checked_add(1).ok_or(Error::SizeOverflow)?)
            .ok_or(Error::SizeOverflow)?,
    )?;
    if let Some(s) = data.skin {
        if !matches!(s.influences, 4 | 8)
            || s.joints.len() != n.checked_mul(s.influences).ok_or(Error::SizeOverflow)?
            || s.weights.len() != s.joints.len()
        {
            return Err(Error::InvalidLayout);
        }
        for &v in s.importance {
            if !v.is_finite() || v < 0.0 {
                return Err(Error::InvalidParameter);
            }
        }
    } else if !data.joint_poses.is_empty() {
        return Err(Error::InvalidParameter);
    }
    for shape in data.blend_shapes {
        if shape.len() != n {
            return Err(Error::InvalidLayout);
        }
    }
    for pose in data.joint_poses {
        m.charge(pose.len())?;
        if pose.len() != bones {
            return Err(Error::InvalidLayout);
        }
        for matrix in *pose {
            for row in matrix {
                for &v in row {
                    finite(v)?;
                }
            }
        }
    }
    let scale = crate::simplify_scale(p)?;
    let scale = if scale == 0.0 { 1.0 } else { scale };
    let mut a = m.alloc(n.checked_mul(width).ok_or(Error::SizeOverflow)?, ws)?;
    for i in 0..n {
        let base = a.len();
        a.resize(base.checked_add(width).ok_or(Error::SizeOverflow)?, 0.0);
        if let Some(s) = data.skin {
            m.charge(s.influences)?;
            let mut sum = 0.0;
            for k in 0..s.influences {
                let j = i * s.influences + k;
                let b = usize::try_from(s.joints[j]).map_err(|_| Error::SizeOverflow)?;
                let w = s.weights[j];
                if b >= bones || !w.is_finite() || w < 0.0 {
                    return Err(Error::InvalidParameter);
                }
                sum += w;
                a[base + b] = finite(a[base + b] + w * s.importance[b] * o.skin_weight)?;
            }
            if (sum - 1.0).abs() > 0.001 {
                return Err(Error::InvalidParameter);
            }
        }
        let rest = p.get(i).ok_or(Error::InvalidLayout)?;
        for (j, shape) in data.blend_shapes.iter().enumerate() {
            for k in 0..3 {
                // Absolute full-target positions include rest-pose residuals.
                let target = finite(rest[k] + finite(shape[i][k])?)?;
                a[base + bones + j * 3 + k] = finite(target / scale * o.blend_shape_weight)?;
            }
        }
    }
    Ok((a, width))
}
fn view(a: &[f32], n: usize, width: usize) -> Result<Attributes<'_>, Error> {
    Attributes::from_interleaved(a, n, width, width, 0)
}
/// Simplify with exact dense bone-weight and full blend-target position channels.
/// This extension is not in C++ meshoptimizer. The summed attribute energy is a
/// pose-aware surrogate, not a maximum-pose geometric error guarantee. Bone IDs
/// are channel identities, never numeric attributes. Influence order is irrelevant
/// except for floating-point summation of duplicates. No streams are rewritten.
/// At most 32 channels (bone count + 3 * blend count) are supported; excess is
/// rejected, never projected or truncated. Empty deformation inputs use plain simplify.
///
/// ```
/// use meshoptimizer_rs::*;
/// let p = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.]];
/// let result = simplify_character(&[0,1,2], Positions::from_packed(&p),
///     &CharacterInputs::new(), &CharacterOptions::default(),
///     SimplifySettings { target_index_count: 3, target_error: 1., options: SimplifyOptions::EMPTY },
///     &mut Workspace::default())?;
/// assert_eq!(result.indices.len(), 3);
/// # Ok::<(), Error>(())
/// ```
/// # Errors
/// Layout, parameter, numerical, size and resource errors; underlying simplify errors.
pub fn simplify_character(
    indices: &[u32],
    positions: Positions<'_>,
    data: &CharacterInputs<'_>,
    options: &CharacterOptions,
    settings: SimplifySettings,
    workspace: &mut Workspace,
) -> Result<SimplifiedMesh, Error> {
    run(workspace, |m, ws| {
        let (a, width) = prepare(positions, data, options, m, ws)?;
        m.call(ws, |ws| {
            if width == 0 {
                crate::simplify(indices, positions, settings, ws)
            } else {
                crate::simplify_with_attributes(
                    indices,
                    positions,
                    view(&a, positions.len(), width)?,
                    &[1.; 32][..width],
                    None,
                    settings,
                    ws,
                )
            }
        })
    })
}
/// Build character LODs and F9 reports in bind, full blend, and caller joint poses.
/// This extension is not in C++ meshoptimizer. Joint poses use linear blend
/// skinning of rest positions. Blend targets and joint poses are sampled separately;
/// combined morph/animation extrema are not implied. Quality is sampled, not a bound.
/// Limits cover the whole call. Failure returns no partial chain.
/// # Errors
/// Same as [`simplify_character`], chain and quality errors. Existing `attributes`
/// in chain options are rejected (character channels occupy that metric).
pub fn build_character_lod_chain(
    indices: &[u32],
    positions: Positions<'_>,
    data: &CharacterInputs<'_>,
    options: &CharacterOptions,
    chain_options: &LodChainOptions<'_>,
    quality_options: &LodQualityOptions,
    workspace: &mut Workspace,
) -> Result<CharacterLodChain, Error> {
    run(workspace, |m, ws| {
        if chain_options.attributes.is_some() {
            return Err(Error::InvalidParameter);
        }
        let (a, width) = prepare(positions, data, options, m, ws)?;
        let mut co = *chain_options;
        let weights = [1.; 32];
        if width != 0 {
            co.attributes = Some(LodChainAttributes::new(
                view(&a, positions.len(), width)?,
                &weights[..width],
            ));
        }
        let chain = m.call(ws, |ws| {
            crate::build_lod_chain(indices, positions, &co, ws).map_err(|e| e.error)
        })?;
        m.retain(&chain.levels)?;
        for level in &chain.levels {
            m.retain(&level.indices)?;
        }
        let count = 1usize
            .checked_add(data.blend_shapes.len())
            .and_then(|v| v.checked_add(data.joint_poses.len()))
            .ok_or(Error::SizeOverflow)?;
        let mut quality = m.alloc(count, ws)?;
        let mut posed = m.alloc(positions.len(), ws)?;
        for pose in 0..count {
            posed.clear();
            for i in 0..positions.len() {
                m.charge(1)?;
                let rest = positions.get(i).ok_or(Error::InvalidLayout)?;
                let mut p = rest;
                if pose > 0 && pose <= data.blend_shapes.len() {
                    for k in 0..3 {
                        p[k] = finite(rest[k] + data.blend_shapes[pose - 1][i][k])?;
                    }
                } else if pose > data.blend_shapes.len() {
                    let s = data.skin.ok_or(Error::InvalidParameter)?;
                    let matrices = data.joint_poses[pose - data.blend_shapes.len() - 1];
                    p = [0.; 3];
                    m.charge(s.influences)?;
                    for k in 0..s.influences {
                        let j = i * s.influences + k;
                        let matrix = matrices
                            [usize::try_from(s.joints[j]).map_err(|_| Error::SizeOverflow)?];
                        for r in 0..3 {
                            p[r] = finite(
                                p[r] + s.weights[j]
                                    * (matrix[r][0] * rest[0]
                                        + matrix[r][1] * rest[1]
                                        + matrix[r][2] * rest[2]
                                        + matrix[r][3]),
                            )?;
                        }
                    }
                }
                posed.push(p);
            }
            let mut reports = m.alloc(chain.levels.len(), ws)?;
            for level in &chain.levels {
                let p = Positions::from_packed(&posed);
                reports.push(m.call(ws, |ws| {
                    crate::measure_lod_quality(
                        LodQualityMesh::new(indices, p),
                        LodQualityMesh::new(&level.indices, p),
                        quality_options,
                        ws,
                    )
                })?);
            }
            quality.push(reports);
        }
        Ok(CharacterLodChain { chain, quality })
    })
}
