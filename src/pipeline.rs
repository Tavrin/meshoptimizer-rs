//! One-call asset processing, not in C++ meshoptimizer.
use crate::*;
use alloc::vec::Vec;
use core::{fmt, mem::size_of};

/// A pipeline operation, not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProcessStage {
    /// Input validation and owned storage.
    Validate,
    /// Triangle sanitation.
    Sanitize,
    /// Binary record welding and remapping.
    Weld,
    /// Face orientation.
    Orient,
    /// Bounded hole filling.
    FillHoles,
    /// Static or character LOD chain.
    Lod,
    /// Vertex cache ordering.
    Cache,
    /// Overdraw ordering.
    Overdraw,
    /// First-use vertex ordering.
    Fetch,
    /// Sampled surface quality.
    Quality,
    /// Meshlet construction.
    Meshlets,
    /// Cluster hierarchy construction.
    Clusters,
    /// Index and vertex encoding.
    Encode,
}
/// Stage failure, not in C++ meshoptimizer. No partial output is returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ProcessError {
    /// Failed operation.
    pub stage: ProcessStage,
    /// Level, with zero denoting the processed source.
    pub level: Option<usize>,
    /// Underlying checked failure.
    pub error: Error,
}
impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} level {:?}: {}", self.stage, self.level, self.error)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}
/// Machine-readable stage measurement, not in C++ meshoptimizer.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ProcessStageReport {
    /// Operation.
    pub stage: ProcessStage,
    /// Level, if applicable.
    pub level: Option<usize>,
    /// Input index and vertex counts.
    pub input: [usize; 2],
    /// Output index and vertex counts.
    pub output: [usize; 2],
    /// Elapsed nanoseconds with a std clock; absent on no_std and wasm32-unknown-unknown.
    /// Timings are nondeterministic.
    pub elapsed_ns: Option<u128>,
    /// Counted work charged by this operation.
    pub work: u64,
}
/// Whole-call report, not in C++ meshoptimizer.
#[derive(Debug)]
#[non_exhaustive]
pub struct ProcessReport {
    /// Execution order, including repeated per-level operations.
    pub stages: Vec<ProcessStageReport>,
    /// Whole-call work and conservative peak byte accounting.
    pub usage: Usage,
    /// F9 source-relative quality, in output level order. Empty surfaces yield None.
    pub quality: Vec<Option<LodQuality>>,
}
/// Skin fields within each complete vertex record, not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ProcessSkin<'a> {
    /// Byte offset of four or eight little-endian u32 joints.
    pub joints_offset: usize,
    /// Byte offset of four or eight little-endian f32 weights.
    pub weights_offset: usize,
    /// Four or eight influences.
    pub influences: usize,
    /// Importance for every bone.
    pub importance: &'a [f32],
    /// Character simplification options.
    pub options: CharacterOptions,
}
impl<'a> ProcessSkin<'a> {
    /// Define skin fields; validated by process_mesh.
    pub fn new(
        joints_offset: usize,
        weights_offset: usize,
        influences: usize,
        importance: &'a [f32],
    ) -> Self {
        Self {
            joints_offset,
            weights_offset,
            influences,
            importance,
            options: CharacterOptions::default(),
        }
    }
}
/// Pipeline settings, not in C++ meshoptimizer.
/// Default: sanitize, weld complete records, two successive LODs at 50%/25%
/// with 1% error, cache/overdraw (1.05)/fetch, and sampled F9 quality.
/// Repair, meshlets, clusters and encoding are opt-in.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ProcessOptions<'a> {
    /// None validates topology without dropping triangles.
    pub sanitize: Option<SanitizeOptions>,
    /// Weld bit-identical complete records, including all attributes and skin fields.
    pub weld: bool,
    /// Orient faces before LOD generation.
    pub orient: bool,
    /// Maximum hole boundary edges; None disables filling.
    pub fill_holes: Option<usize>,
    /// Optional LOD chain. Attributes/flags must be absent: use attribute fields below.
    pub lod: Option<LodChainOptions<'a>>,
    /// Byte offset and component weights for little-endian f32 simplification attributes.
    pub attributes: Option<(usize, &'a [f32])>,
    /// Optional skin fields; selects the character chain builder.
    pub skin: Option<ProcessSkin<'a>>,
    /// Enable cache optimization.
    pub cache: bool,
    /// Overdraw threshold; None disables optimization.
    pub overdraw: Option<f32>,
    /// Enable vertex fetch optimization independently for each level.
    pub fetch: bool,
    /// F9 sampling settings.
    pub quality: LodQualityOptions,
    /// Meshlet settings and cone weight.
    pub meshlets: Option<(MeshletSettings, f32)>,
    /// Cluster builder configuration (requires clusterlod).
    #[cfg(feature = "clusterlod")]
    pub clusters: Option<clusterlod::Config>,
    /// Optional index codec settings.
    pub index_encoding: Option<codec::IndexEncoding>,
    /// Optional vertex codec settings.
    pub vertex_encoding: Option<codec::VertexEncoding>,
}
impl Default for ProcessOptions<'_> {
    fn default() -> Self {
        Self {
            sanitize: Some(SanitizeOptions::default()),
            weld: true,
            orient: false,
            fill_holes: None,
            lod: Some(LodChainOptions::new(
                LodTargets::Ratios(&[0.5, 0.25]),
                LodErrors::Uniform(0.01),
            )),
            attributes: None,
            skin: None,
            cache: true,
            overdraw: Some(1.05),
            fetch: true,
            quality: LodQualityOptions::default(),
            meshlets: None,
            #[cfg(feature = "clusterlod")]
            clusters: None,
            index_encoding: None,
            vertex_encoding: None,
        }
    }
}
/// Cluster output with its builder-owned positions, not in C++ meshoptimizer.
/// Cluster indices reference this array. Dilation can change it independently
/// of the optimized level's complete vertex records.
#[cfg(feature = "clusterlod")]
#[derive(Debug)]
#[non_exhaustive]
pub struct ProcessClusters {
    /// Positions returned by the mutable cluster builder, including dilation.
    pub positions: Vec<[f32; 3]>,
    /// Hierarchy groups in builder emission order.
    pub groups: Vec<clusterlod::GroupOutput>,
}
/// One optimized level, not in C++ meshoptimizer.
#[derive(Debug)]
#[non_exhaustive]
pub struct ProcessLevel {
    /// Triangle indices into this level's vertices.
    pub indices: Vec<u32>,
    /// Complete tightly packed vertex records, preserving every attribute.
    pub vertices: Vec<u8>,
    /// Optional meshlets over this level.
    pub meshlets: Option<Meshlets>,
    /// Optional cluster hierarchy and its potentially dilated positions.
    #[cfg(feature = "clusterlod")]
    pub clusters: Option<ProcessClusters>,
    /// Optional encoded index bytes.
    pub encoded_indices: Option<Vec<u8>>,
    /// Optional encoded vertex bytes.
    pub encoded_vertices: Option<Vec<u8>>,
}
/// Atomic pipeline output, not in C++ meshoptimizer.
#[derive(Debug)]
#[non_exhaustive]
pub struct ProcessedMesh {
    /// Processed source followed by requested LODs.
    pub levels: Vec<ProcessLevel>,
    /// Combined measurements.
    pub report: ProcessReport,
}

struct Meter {
    limits: Limits,
    held: usize,
    peak: usize,
    work: u64,
    stages: Vec<ProcessStageReport>,
    stage: ProcessStage,
    level: Option<usize>,
}
impl Meter {
    fn hold<T>(&mut self, v: &Vec<T>) -> Result<(), Error> {
        self.held = self
            .held
            .checked_add(
                v.capacity()
                    .checked_mul(size_of::<T>())
                    .ok_or(Error::SizeOverflow)?,
            )
            .ok_or(Error::SizeOverflow)?;
        if self.held > self.limits.max_bytes {
            return Err(Error::LimitExceeded);
        }
        Ok(())
    }
    fn call<T>(
        &mut self,
        stage: ProcessStage,
        level: Option<usize>,
        input: [usize; 2],
        ws: &mut Workspace,
        f: impl FnOnce(&mut Workspace) -> Result<T, Error>,
    ) -> Result<T, Error> {
        self.stage = stage;
        self.level = level;
        ws.set_lod_limits(Limits {
            max_bytes: self
                .limits
                .max_bytes
                .checked_sub(self.held)
                .ok_or(Error::LimitExceeded)?,
            max_work: self
                .limits
                .max_work
                .checked_sub(self.work)
                .ok_or(Error::LimitExceeded)?,
        });
        #[cfg(all(
            feature = "std",
            not(all(target_arch = "wasm32", target_os = "unknown"))
        ))]
        let start = std::time::Instant::now();
        // Reset measurements even for layout-only validation failures.
        let work = ws.begin();
        ws.finish(&work);
        let r = f(ws);
        let usage = ws.usage();
        self.work = self
            .work
            .checked_add(usage.work)
            .ok_or(Error::SizeOverflow)?;
        self.peak = self.peak.max(
            self.held
                .checked_add(usage.bytes)
                .ok_or(Error::SizeOverflow)?,
        );
        #[cfg(all(
            feature = "std",
            not(all(target_arch = "wasm32", target_os = "unknown"))
        ))]
        let elapsed_ns = Some(start.elapsed().as_nanos());
        #[cfg(any(
            not(feature = "std"),
            all(target_arch = "wasm32", target_os = "unknown")
        ))]
        let elapsed_ns = None;
        if self.stages.len() == self.stages.capacity() {
            return Err(Error::LimitExceeded);
        }
        self.stages.push(ProcessStageReport {
            stage,
            level,
            input,
            output: input,
            elapsed_ns,
            work: usage.work,
        });
        r
    }
    fn counts(&mut self, output: [usize; 2]) {
        if let Some(s) = self.stages.last_mut() {
            s.output = output;
        }
    }
}
fn positions(bytes: &[u8], stride: usize, offset: usize) -> Result<Positions<'_>, Error> {
    let max_offset = stride.checked_sub(12).ok_or(Error::InvalidLayout)?;
    if offset > max_offset {
        return Err(Error::InvalidLayout);
    }
    // No record is addressed after fetch compacts an empty level.
    let offset = if bytes.is_empty() { 0 } else { offset };
    Positions::from_bytes(
        bytes,
        bytes.len() / stride,
        stride,
        offset,
        ByteOrder::LittleEndian,
    )
}
fn copied<T: Copy>(source: &[T], ws: &mut Workspace) -> Result<Vec<T>, Error> {
    let mut work = ws.begin();
    let r = (|| {
        work.add(source.len())?;
        crate::budget::Budget::new(ws).copied(source)
    })();
    ws.finish(&work);
    r
}
/// Run the common asset path on complete tightly packed little-endian records.
/// This extension is not in C++ meshoptimizer. Position offset addresses three f32s.
/// Record size is 12..=256 bytes. All attributes (including skin) participate in
/// binary welding; padding must be initialized deterministically. Attribute/skin
/// offsets are preserved through remapping and fetch. Source level is always included.
///
/// Stages call existing public functions in order, with one shared workspace.
/// Limits cover total counted work and all pipeline allocations plus scratch;
/// bytes conservatively retain charges for released intermediates. On failure
/// limits are restored and usage includes work attempted. Empty surfaces have no
/// quality measurement. Timing is the only nondeterministic report field.
/// # Errors
/// Returns the failing stage and level. Invalid layouts/options, malformed input,
/// arithmetic overflow, allocation failure and exhausted limits are checked.
pub fn process_mesh(
    indices: &[u32],
    vertices: &[u8],
    stride: usize,
    position_offset: usize,
    options: &ProcessOptions<'_>,
    ws: &mut Workspace,
) -> Result<ProcessedMesh, ProcessError> {
    let limits = ws.limits();
    let mut m = Meter {
        limits,
        held: 0,
        peak: 0,
        work: 0,
        stages: Vec::new(),
        stage: ProcessStage::Validate,
        level: None,
    };
    let result = process(
        indices,
        vertices,
        stride,
        position_offset,
        options,
        ws,
        &mut m,
    );
    ws.set_lod_limits(limits);
    ws.set_lod_usage(Usage {
        bytes: m.peak.max(m.held),
        work: m.work,
    });
    result.map_err(|error| ProcessError {
        stage: m.stage,
        level: m.level,
        error,
    })
}
fn process(
    indices: &[u32],
    input: &[u8],
    stride: usize,
    offset: usize,
    o: &ProcessOptions<'_>,
    ws: &mut Workspace,
    m: &mut Meter,
) -> Result<ProcessedMesh, Error> {
    let initial_work = ws.begin();
    m.peak = ws.retained_bytes()?;
    if !(12..=256).contains(&stride) || !input.len().is_multiple_of(stride) {
        return Err(Error::InvalidLayout);
    }
    let source_positions = positions(input, stride, offset)?;
    if o.lod
        .is_some_and(|l| l.attributes.is_some() || l.vertex_flags.is_some())
    {
        return Err(Error::InvalidParameter);
    }
    let targets = o.lod.map_or(0, |l| match l.targets {
        LodTargets::Counts(v) => v.len(),
        LodTargets::Ratios(v) => v.len(),
    });
    let levels_count = targets.checked_add(1).ok_or(Error::SizeOverflow)?;
    let records = levels_count
        .checked_mul(12)
        .and_then(|n| n.checked_add(20))
        .ok_or(Error::SizeOverflow)?;
    m.stages = crate::budget::Budget::new(ws).reserved(records)?;
    let stage_bytes = m
        .stages
        .capacity()
        .checked_mul(size_of::<ProcessStageReport>())
        .ok_or(Error::SizeOverflow)?;
    m.held = stage_bytes;
    ws.finish(&initial_work);
    m.peak = m.peak.max(
        m.held
            .checked_add(ws.retained_bytes()?)
            .ok_or(Error::SizeOverflow)?,
    );
    let mut indices = if let Some(s) = o.sanitize {
        let r = m.call(
            ProcessStage::Sanitize,
            None,
            [indices.len(), source_positions.len()],
            ws,
            |ws| sanitize(indices, source_positions, s, ws),
        )?;
        m.counts([r.indices.len(), source_positions.len()]);
        r.indices
    } else {
        m.call(
            ProcessStage::Validate,
            None,
            [indices.len(), source_positions.len()],
            ws,
            |ws| {
                let mut work = ws.begin();
                let r = (|| {
                    crate::workspace::topology(indices, source_positions.len(), &mut work)?;
                    for &i in indices {
                        work.add(1)?;
                        let p = source_positions
                            .get(usize::try_from(i).map_err(|_| Error::SizeOverflow)?)
                            .ok_or(Error::IndexOutOfBounds)?;
                        if p.iter().any(|x| !x.is_finite()) {
                            return Err(Error::NumericalFailure);
                        }
                    }
                    work.add(indices.len())?;
                    crate::budget::Budget::new(ws).copied(indices)
                })();
                ws.finish(&work);
                r
            },
        )?
    };
    m.hold(&indices)?;
    let mut vertices = if o.weld {
        let remap = m.call(
            ProcessStage::Weld,
            None,
            [indices.len(), source_positions.len()],
            ws,
            |ws| generate_vertex_remap(Some(&indices), VertexStream::packed(input, stride)?, ws),
        )?;
        m.hold(&remap.remap)?;
        indices = m.call(
            ProcessStage::Weld,
            None,
            [indices.len(), source_positions.len()],
            ws,
            |ws| remap_index_buffer(Some(&indices), &remap.remap, ws),
        )?;
        m.hold(&indices)?;
        let v = m.call(
            ProcessStage::Weld,
            None,
            [indices.len(), source_positions.len()],
            ws,
            |ws| remap_vertex_buffer(VertexStream::packed(input, stride)?, &remap.remap, ws),
        )?;
        m.counts([indices.len(), v.len() / stride]);
        v
    } else {
        m.call(
            ProcessStage::Validate,
            None,
            [indices.len(), source_positions.len()],
            ws,
            |ws| copied(input, ws),
        )?
    };
    m.hold(&vertices)?;
    if o.orient {
        indices = m.call(
            ProcessStage::Orient,
            None,
            [indices.len(), vertices.len() / stride],
            ws,
            |ws| orient_faces(&indices, vertices.len() / stride, ws).map(|r| r.indices),
        )?;
        m.hold(&indices)?;
    }
    if let Some(max_edges) = o.fill_holes {
        indices = m.call(
            ProcessStage::FillHoles,
            None,
            [indices.len(), vertices.len() / stride],
            ws,
            |ws| {
                fill_holes(
                    &indices,
                    positions(&vertices, stride, offset)?,
                    max_edges,
                    ws,
                )
                .map(|r| r.indices)
            },
        )?;
        m.counts([indices.len(), vertices.len() / stride]);
        m.hold(&indices)?;
    }
    let mut chain = if let Some(mut settings) = o.lod {
        m.stage = ProcessStage::Lod;
        m.level = None;
        let p = positions(&vertices, stride, offset)?;
        if let Some((a, weights)) = o.attributes {
            settings.attributes = Some(LodChainAttributes::new(
                Attributes::from_bytes(
                    &vertices,
                    p.len(),
                    weights.len(),
                    stride,
                    a,
                    ByteOrder::LittleEndian,
                )?,
                weights,
            ));
        }
        if let Some(skin) = o.skin {
            let (joints, weights) = m.call(
                ProcessStage::Lod,
                None,
                [indices.len(), p.len()],
                ws,
                |ws| {
                    if skin.influences != 4 && skin.influences != 8 {
                        return Err(Error::InvalidParameter);
                    }
                    let width = skin.influences.checked_mul(4).ok_or(Error::SizeOverflow)?;
                    if skin
                        .joints_offset
                        .checked_add(width)
                        .ok_or(Error::SizeOverflow)?
                        > stride
                        || skin
                            .weights_offset
                            .checked_add(width)
                            .ok_or(Error::SizeOverflow)?
                            > stride
                    {
                        return Err(Error::InvalidLayout);
                    }
                    let mut work = ws.begin();
                    let r = (|| {
                        let n = p
                            .len()
                            .checked_mul(skin.influences)
                            .ok_or(Error::SizeOverflow)?;
                        let mut b = crate::budget::Budget::new(ws);
                        let mut j = b.reserved(n)?;
                        let mut w = b.reserved(n)?;
                        for record in vertices.chunks_exact(stride) {
                            for k in 0..skin.influences {
                                work.add(1)?;
                                let a = skin.joints_offset + k * 4;
                                let c = skin.weights_offset + k * 4;
                                j.push(u32::from_le_bytes(
                                    record[a..a + 4]
                                        .try_into()
                                        .map_err(|_| Error::InvalidLayout)?,
                                ));
                                w.push(f32::from_le_bytes(
                                    record[c..c + 4]
                                        .try_into()
                                        .map_err(|_| Error::InvalidLayout)?,
                                ));
                            }
                        }
                        Ok((j, w))
                    })();
                    ws.finish(&work);
                    r
                },
            )?;
            m.hold(&joints)?;
            m.hold(&weights)?;
            let mut data = CharacterInputs::new();
            data.skin = Some(CharacterSkin::new(
                &joints,
                &weights,
                skin.influences,
                skin.importance,
            ));
            m.call(
                ProcessStage::Lod,
                None,
                [indices.len(), p.len()],
                ws,
                |ws| {
                    build_character_lod_chain(
                        &indices,
                        p,
                        &data,
                        &skin.options,
                        &settings,
                        &o.quality,
                        ws,
                    )
                    .map(|r| r.chain)
                },
            )?
        } else {
            m.call(
                ProcessStage::Lod,
                None,
                [indices.len(), p.len()],
                ws,
                |ws| build_lod_chain(&indices, p, &settings, ws).map_err(|e| e.error),
            )?
        }
    } else {
        LodChain { levels: Vec::new() }
    };
    if o.lod.is_some() {
        let total = chain.levels.iter().try_fold(0usize, |n, l| {
            n.checked_add(l.indices.len()).ok_or(Error::SizeOverflow)
        })?;
        m.counts([total, vertices.len() / stride]);
    }
    m.hold(&chain.levels)?;
    for level in &chain.levels {
        m.hold(&level.indices)?;
    }
    let (mut levels, mut quality) = m.call(
        ProcessStage::Validate,
        None,
        [indices.len(), vertices.len() / stride],
        ws,
        |ws| {
            let mut b = crate::budget::Budget::new(ws);
            Ok((b.reserved(levels_count)?, b.reserved(levels_count)?))
        },
    )?;
    m.hold(&levels)?;
    m.hold(&quality)?;
    // Preserve the pre-LOD surface for source-relative F9 measurements.
    let source_indices = m.call(
        ProcessStage::Validate,
        None,
        [indices.len(), vertices.len() / stride],
        ws,
        |ws| copied(&indices, ws),
    )?;
    m.hold(&source_indices)?;
    let source_vertices = m.call(
        ProcessStage::Validate,
        None,
        [indices.len(), vertices.len() / stride],
        ws,
        |ws| copied(&vertices, ws),
    )?;
    m.hold(&source_vertices)?;
    for level in 0..levels_count {
        if level > 0 {
            indices = core::mem::take(&mut chain.levels[level - 1].indices);
            vertices = m.call(
                ProcessStage::Validate,
                Some(level),
                [indices.len(), source_vertices.len() / stride],
                ws,
                |ws| copied(&source_vertices, ws),
            )?;
            m.hold(&vertices)?;
        }
        if o.cache {
            indices = m.call(
                ProcessStage::Cache,
                Some(level),
                [indices.len(), vertices.len() / stride],
                ws,
                |ws| optimize_vertex_cache(&indices, vertices.len() / stride, ws),
            )?;
            m.hold(&indices)?;
        }
        if let Some(t) = o.overdraw {
            indices = m.call(
                ProcessStage::Overdraw,
                Some(level),
                [indices.len(), vertices.len() / stride],
                ws,
                |ws| optimize_overdraw(&indices, positions(&vertices, stride, offset)?, t, ws),
            )?;
            m.hold(&indices)?;
        }
        if o.fetch {
            let fetched = m.call(
                ProcessStage::Fetch,
                Some(level),
                [indices.len(), vertices.len() / stride],
                ws,
                |ws| optimize_vertex_fetch(&indices, VertexStream::packed(&vertices, stride)?, ws),
            )?;
            indices = fetched.indices;
            vertices = fetched.vertices;
            m.counts([indices.len(), vertices.len() / stride]);
            m.hold(&indices)?;
            m.hold(&vertices)?;
        }
        let p = positions(&vertices, stride, offset)?;
        quality.push(if indices.is_empty() || source_indices.is_empty() {
            None
        } else {
            Some(m.call(
                ProcessStage::Quality,
                Some(level),
                [indices.len(), p.len()],
                ws,
                |ws| {
                    measure_lod_quality(
                        LodQualityMesh::new(
                            &source_indices,
                            positions(&source_vertices, stride, offset)?,
                        ),
                        LodQualityMesh::new(&indices, p),
                        &o.quality,
                        ws,
                    )
                },
            )?)
        });
        let meshlets = if let Some((settings, cone)) = o.meshlets {
            let r = m.call(
                ProcessStage::Meshlets,
                Some(level),
                [indices.len(), p.len()],
                ws,
                |ws| build_meshlets(&indices, p, settings, cone, ws),
            )?;
            m.hold(&r.meshlets)?;
            m.hold(&r.vertices)?;
            m.hold(&r.triangles)?;
            Some(r)
        } else {
            None
        };
        #[cfg(feature = "clusterlod")]
        let clusters = if let Some(config) = o.clusters {
            let mut packed = m.call(
                ProcessStage::Clusters,
                Some(level),
                [indices.len(), p.len()],
                ws,
                |ws| {
                    let mut work = ws.begin();
                    let r = (|| {
                        let mut b = crate::budget::Budget::new(ws);
                        let mut v = b.reserved(p.len())?;
                        for i in 0..p.len() {
                            work.add(1)?;
                            v.push(p.get(i).ok_or(Error::IndexOutOfBounds)?);
                        }
                        Ok(v)
                    })();
                    ws.finish(&work);
                    r
                },
            )?;
            m.hold(&packed)?;
            let r = m.call(
                ProcessStage::Clusters,
                Some(level),
                [indices.len(), p.len()],
                ws,
                |ws| {
                    let attributes = o
                        .attributes
                        .map(|(offset, weights)| {
                            Attributes::from_bytes(
                                &vertices,
                                p.len(),
                                weights.len(),
                                stride,
                                offset,
                                ByteOrder::LittleEndian,
                            )
                        })
                        .transpose()?;
                    clusterlod::build(
                        config,
                        clusterlod::Mesh {
                            indices: &indices,
                            positions: &mut packed,
                            attributes,
                            vertex_lock: None,
                            attribute_weights: o.attributes.map_or(&[], |(_, weights)| weights),
                            attribute_protect_mask: 0,
                        },
                        ws,
                    )
                },
            )?;
            m.hold(&r)?;
            for g in &r {
                m.hold(&g.clusters)?;
                for c in &g.clusters {
                    m.hold(&c.indices)?;
                }
            }
            Some(ProcessClusters {
                positions: packed,
                groups: r,
            })
        } else {
            None
        };
        let encoded_indices = if let Some(e) = o.index_encoding {
            let r = m.call(
                ProcessStage::Encode,
                Some(level),
                [indices.len(), p.len()],
                ws,
                |ws| codec::encode_index_buffer(&indices, e, ws),
            )?;
            m.hold(&r)?;
            Some(r)
        } else {
            None
        };
        let encoded_vertices = if let Some(e) = o.vertex_encoding {
            let r = m.call(
                ProcessStage::Encode,
                Some(level),
                [indices.len(), p.len()],
                ws,
                |ws| codec::encode_vertex_buffer(&vertices, p.len(), stride, e, ws),
            )?;
            m.hold(&r)?;
            Some(r)
        } else {
            None
        };
        levels.push(ProcessLevel {
            indices: core::mem::take(&mut indices),
            vertices: core::mem::take(&mut vertices),
            meshlets,
            #[cfg(feature = "clusterlod")]
            clusters,
            encoded_indices,
            encoded_vertices,
        });
    }
    let usage = Usage {
        bytes: m.peak.max(m.held),
        work: m.work,
    };
    Ok(ProcessedMesh {
        levels,
        report: ProcessReport {
            stages: core::mem::take(&mut m.stages),
            usage,
            quality,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_validation_reports_retained_storage() -> Result<(), Error> {
        let mut indices = [0u32; 300];
        for (i, value) in indices.iter_mut().enumerate() {
            *value = u32::try_from(i).map_err(|_| Error::SizeOverflow)?;
        }
        let mut ws = Workspace::default();
        let _ = optimize_vertex_cache(&indices, 300, &mut ws)?;
        let retained = ws.retained_bytes()?;
        assert!(retained > 0);
        let limits = ws.limits();
        let error = match process_mesh(&[], &[], 0, 0, &ProcessOptions::default(), &mut ws) {
            Err(error) => error,
            Ok(_) => return Err(Error::InvalidParameter),
        };
        assert_eq!(error.stage, ProcessStage::Validate);
        assert_eq!(
            ws.usage(),
            Usage {
                bytes: retained,
                work: 0
            }
        );
        assert_eq!(ws.limits(), limits);
        Ok(())
    }
}
