//! Structural cooker gate over a built DAG or an opened MDAG 2 container. It
//! inspects stored errors, pins and triangle counts; it does not certify
//! sampled geometric accuracy.
use super::super::dag_format::v2::Reader;
use super::*;

/// Why a group stopped simplifying. Serialized containers do not retain these
/// reasons; [`health_from_reader`] infers only [`GroupStatus::Root`] and
/// [`GroupStatus::UnknownPinned`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GroupStatus {
    /// An accepted simplification step.
    #[default]
    Simplified,
    /// An accepted step that removed triangles but retained more than the
    /// configured threshold. Consecutive partial steps must each strictly
    /// remove triangles.
    PartialReduction,
    /// Final parentless root, expected to remain pinned.
    Root,
    /// Output retained too many input triangles.
    RetainedRatio,
    /// Simplification produced no triangles.
    Empty,
    /// Replacement contained a triangle spanning material slots.
    MixedMaterial,
    /// Replacement failed the alpha coverage guard.
    AlphaCoverage,
    /// Pinned group whose build-time reason was not retained.
    UnknownPinned,
}

/// Build-time error terms of one group, before the writer adds quantization
/// displacement. The original-source term includes previously measured error
/// floors, so it is not an independent measurement of raw deviation.
#[derive(Clone, Copy, Debug, Default)]
pub struct GroupDiagnostics {
    /// Acceptance or pin reason.
    pub status: GroupStatus,
    /// Simplifier's error estimate, including its configured multipliers.
    pub simplifier_error: f32,
    /// Independently sampled symmetric deviation of an accepted step;
    /// rejected steps retain the simplifier estimate.
    pub step_error: f32,
    /// Sampled original-source deviation, including the existing step floor.
    pub source_error: f32,
    /// Largest child error inherited by this group.
    pub inherited_error: f32,
    /// The simplifier estimate came from the grid-based sloppy fallback.
    pub sloppy_fallback: bool,
}

/// Error and triangle totals for one occupied depth.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DepthHealth {
    /// Fine-to-coarse depth.
    pub depth: u32,
    /// All groups at this depth.
    pub groups: usize,
    /// Triangles stored at this depth, including pinned groups.
    pub triangles: u64,
    /// Minimum finite non-pinned error; zero if none.
    pub error_min: f32,
    /// Upper median finite non-pinned error; zero if none.
    pub error_median: f32,
    /// Maximum finite non-pinned error; zero if none.
    pub error_max: f32,
    /// Pinned groups at this depth, including final roots.
    pub pinned: usize,
    /// Groups whose simplifier estimate came from the sloppy fallback; zero
    /// without build diagnostics.
    pub sloppy: usize,
    /// Accepted partial-reduction steps; zero without build diagnostics.
    pub partial: usize,
}

/// A finite positive equal-error path with falling triangle counts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqualErrorChain {
    /// Coarsest group of this path, in table order.
    pub group: u32,
    /// Number of consecutive depths in this path.
    pub levels: u32,
    /// Exactly equal stored error throughout the path.
    pub error: f32,
}

/// A pinned group with its recorded or inferred reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinnedGroup {
    /// Group in table order.
    pub group: u32,
    /// Root or rejection reason.
    pub reason: GroupStatus,
}

/// Structural quality report suitable for a cooker admission record.
#[derive(Clone, Debug, Default)]
pub struct HealthReport {
    /// Per-depth table, including terminal pinned geometry.
    pub depths: Vec<DepthHealth>,
    /// Equal-error paths at or above the requested minimum length.
    pub equal_error_chains: Vec<EqualErrorChain>,
    /// All pinned groups, including expected roots.
    pub pinned_groups: Vec<PinnedGroup>,
    /// True when there are no reported chains and every pinned group is a root.
    pub passed: bool,
}

/// Inspect a built DAG without changing its errors or topology.
///
/// The rule: the report fails on any pinned group that is not a final root,
/// and on any *equal-error chain*: a finite positive stored error that is
/// exactly equal along at least `minimum_levels` consecutive linked depths
/// (each link joins a group to a parent one depth coarser) while the stored
/// triangle count strictly falls at every link. Such a chain means a coarser
/// level claims no more error than a finer one, so a pixel-threshold cut can
/// never select it. Zero-error chains (exact planar reductions) and the final
/// roots pass. Three levels is the recommended cooker rule; two is the
/// smallest accepted. Reported chains can overlap: each reports its coarsest
/// group and its length.
///
/// This is a structural check on stored values. It does not prove geometric
/// accuracy, and a failing report is evidence to inspect the asset and the
/// build policy, not a request to raise errors artificially.
///
/// # Errors
/// `InvalidParameter` if `minimum_levels < 2`; `InvalidTopology` for malformed
/// tables, a diagnostics table of the wrong length, or a recorded root with
/// parents; allocation and workspace limit errors.
pub fn health(
    built: &Built,
    minimum_levels: u32,
    workspace: &mut Workspace,
) -> Result<HealthReport, Error> {
    let mut ctx = Context::new(workspace)?;
    report(
        &built.groups,
        &built.clusters,
        &built.diagnostics,
        minimum_levels,
        &mut ctx,
    )
}

/// Inspect an opened MDAG 2 container with the rule of [`health()`].
///
/// `cluster_triangles` holds the triangle count of every cluster in container
/// order, as `ClusterDesc::triangle_count` from the transcoded pages. Stored
/// errors include the writer's quantization displacement. Build-time reasons
/// are not stored: pinned groups at the coarsest depth report
/// [`GroupStatus::Root`], every other pin reports [`GroupStatus::UnknownPinned`]
/// and fails the report.
///
/// # Errors
/// `InvalidParameter` if `minimum_levels < 2` or the triangle table length
/// differs from the container's cluster count; `InvalidTopology` for a group
/// or cluster record the reader cannot return; allocation and workspace limit
/// errors.
pub fn health_from_reader(
    reader: &Reader<'_>,
    cluster_triangles: &[u32],
    minimum_levels: u32,
    workspace: &mut Workspace,
) -> Result<HealthReport, Error> {
    let mesh = reader.mesh();
    if cluster_triangles.len() != mesh.cluster_count as usize {
        return Err(Error::InvalidParameter);
    }
    let mut ctx = Context::new(workspace)?;
    let mut groups = ctx.alloc::<FlatDagGroup>(mesh.group_count as usize)?;
    let mut clusters = ctx.alloc::<FlatDagCluster>(cluster_triangles.len())?;
    for (g, group) in groups.iter_mut().enumerate() {
        ctx.tick(1)?;
        let record = reader.group(g).map_err(|_| Error::InvalidTopology)?;
        *group = FlatDagGroup {
            center: record.center,
            radius: record.radius,
            error: record.error,
            depth: i32::try_from(record.depth).map_err(|_| Error::InvalidTopology)?,
            first_cluster: record.first_cluster,
            cluster_count: record.cluster_count,
        };
        let end = (record.first_cluster as usize)
            .checked_add(record.cluster_count as usize)
            .ok_or(Error::SizeOverflow)?;
        let owned = clusters
            .get_mut(record.first_cluster as usize..end)
            .ok_or(Error::InvalidTopology)?;
        for cluster in owned {
            cluster.group = g as u32;
        }
    }
    for (c, cluster) in clusters.iter_mut().enumerate() {
        ctx.tick(1)?;
        cluster.refined = reader
            .cluster_refined(c)
            .map_err(|_| Error::InvalidTopology)?;
        cluster.index_count = cluster_triangles[c]
            .checked_mul(3)
            .ok_or(Error::SizeOverflow)?;
    }
    let result = report(&groups, &clusters, &[], minimum_levels, &mut ctx);
    ctx.free(groups)?;
    ctx.free(clusters)?;
    result
}

fn report(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    diagnostics: &[GroupDiagnostics],
    minimum_levels: u32,
    ctx: &mut Context<'_>,
) -> Result<HealthReport, Error> {
    if minimum_levels < 2 {
        return Err(Error::InvalidParameter);
    }
    let n = groups.len();
    if !diagnostics.is_empty() && diagnostics.len() != n {
        return Err(Error::InvalidTopology);
    }
    let mut triangles = ctx.alloc::<u64>(n)?;
    let mut parents = ctx.alloc::<bool>(n)?;
    let mut order = ctx.alloc::<u32>(n)?;
    let mut max_depth = 0;
    for (id, g) in groups.iter().enumerate() {
        ctx.tick(1)?;
        if g.depth < 0
            || g.depth as u32 >= super::super::dag_format::v2::MAX_LEVELS
            || !g.error.is_finite()
            || g.error < 0.0
        {
            return Err(Error::InvalidTopology);
        }
        max_depth = max_depth.max(g.depth as u32);
        order[id] = id as u32;
        let end = (g.first_cluster as usize)
            .checked_add(g.cluster_count as usize)
            .ok_or(Error::SizeOverflow)?;
        let owned = clusters
            .get(g.first_cluster as usize..end)
            .ok_or(Error::InvalidTopology)?;
        for c in owned {
            ctx.tick(1)?;
            if c.group as usize != id || !c.index_count.is_multiple_of(3) {
                return Err(Error::InvalidTopology);
            }
            triangles[id] += u64::from(c.index_count / 3);
            if c.refined >= 0 {
                let child = groups
                    .get(c.refined as usize)
                    .ok_or(Error::InvalidTopology)?;
                if child.depth >= g.depth {
                    return Err(Error::InvalidTopology);
                }
                parents[c.refined as usize] = true;
            }
        }
    }
    // Children before parents, whichever table order the source used.
    ctx.tick(n.saturating_mul(usize::BITS as usize))?;
    order.sort_by_key(|&id| groups[id as usize].depth);
    let mut levels = ctx.alloc::<u32>(n)?;
    let mut report = HealthReport::default();
    for &id in order.iter() {
        let id = id as usize;
        let g = &groups[id];
        levels[id] = 1;
        if g.error <= 0.0 || g.error == f32::MAX {
            continue;
        }
        let span = g.first_cluster as usize..g.first_cluster as usize + g.cluster_count as usize;
        for c in clusters[span].iter().filter(|c| c.refined >= 0) {
            ctx.tick(1)?;
            let child = c.refined as usize;
            if groups[child].depth + 1 == g.depth
                && groups[child].error == g.error
                && triangles[child] > triangles[id]
            {
                levels[id] = levels[id].max(levels[child] + 1);
            }
        }
        if levels[id] >= minimum_levels {
            grow_output(ctx, &mut report.equal_error_chains, 1)?;
            report.equal_error_chains.push(EqualErrorChain {
                group: id as u32,
                levels: levels[id],
                error: g.error,
            });
        }
    }
    let mut errors = ctx.alloc::<f32>(n)?;
    for depth in 0..=max_depth {
        let mut row = DepthHealth {
            depth,
            ..DepthHealth::default()
        };
        let mut count = 0;
        for (id, g) in groups.iter().enumerate() {
            ctx.tick(1)?;
            if g.depth as u32 != depth {
                continue;
            }
            row.groups += 1;
            row.triangles += triangles[id];
            let diagnostic = diagnostics.get(id);
            row.sloppy += usize::from(diagnostic.is_some_and(|d| d.sloppy_fallback));
            row.partial +=
                usize::from(diagnostic.is_some_and(|d| d.status == GroupStatus::PartialReduction));
            if g.error == f32::MAX {
                row.pinned += 1;
                let reason = match diagnostic {
                    Some(d) => d.status,
                    None if depth == max_depth => GroupStatus::Root,
                    None => GroupStatus::UnknownPinned,
                };
                if reason == GroupStatus::Root && parents[id] {
                    return Err(Error::InvalidTopology);
                }
                grow_output(ctx, &mut report.pinned_groups, 1)?;
                report.pinned_groups.push(PinnedGroup {
                    group: id as u32,
                    reason,
                });
            } else {
                errors[count] = g.error;
                count += 1;
            }
        }
        if row.groups == 0 {
            continue;
        }
        if count > 0 {
            ctx.tick(count.saturating_mul(usize::BITS as usize))?;
            errors[..count].sort_unstable_by(f32::total_cmp);
            row.error_min = errors[0];
            row.error_median = errors[count / 2];
            row.error_max = errors[count - 1];
        }
        grow_output(ctx, &mut report.depths, 1)?;
        report.depths.push(row);
    }
    report.passed = report.equal_error_chains.is_empty()
        && report
            .pinned_groups
            .iter()
            .all(|g| g.reason == GroupStatus::Root);
    ctx.free(errors)?;
    ctx.free(levels)?;
    ctx.free(order)?;
    ctx.free(triangles)?;
    ctx.free(parents)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(error: f32) -> Built {
        let mut b = Built::default();
        for (id, triangles) in [32, 16, 8, 4].into_iter().enumerate() {
            b.groups.push(FlatDagGroup {
                depth: id as i32,
                error: if id == 3 { f32::MAX } else { error },
                first_cluster: id as u32,
                cluster_count: 1,
                ..FlatDagGroup::default()
            });
            b.clusters.push(FlatDagCluster {
                group: id as u32,
                refined: id as i32 - 1,
                index_count: triangles * 3,
                ..FlatDagCluster::default()
            });
            b.diagnostics.push(GroupDiagnostics {
                status: if id == 3 {
                    GroupStatus::Root
                } else {
                    GroupStatus::Simplified
                },
                ..GroupDiagnostics::default()
            });
        }
        b
    }

    #[test]
    fn detects_linked_reducing_plateau_and_allows_exact_geometry() {
        let b = chain(1.0);
        let report = health(&b, 3, &mut Workspace::default()).expect("valid graph");
        assert!(!report.passed);
        assert_eq!(report.equal_error_chains.len(), 1);
        assert_eq!(report.equal_error_chains[0].levels, 3);
        assert_eq!(report.equal_error_chains[0].group, 2);
        assert_eq!(
            report
                .depths
                .iter()
                .map(|d| d.triangles)
                .collect::<Vec<_>>(),
            [32, 16, 8, 4]
        );
        assert!(
            health(&b, 4, &mut Workspace::default())
                .expect("valid")
                .passed
        );
        assert!(
            health(&chain(0.0), 3, &mut Workspace::default())
                .expect("valid")
                .passed
        );
        assert!(matches!(
            health(&b, 1, &mut Workspace::default()),
            Err(Error::InvalidParameter)
        ));
    }

    #[test]
    fn equal_unrelated_errors_are_not_a_chain() {
        let mut b = chain(1.0);
        b.clusters[2].refined = -1;
        assert!(health(&b, 3, &mut Workspace::default())
            .expect("valid")
            .equal_error_chains
            .is_empty());
        b.clusters[2].refined = 2;
        assert!(matches!(
            health(&b, 3, &mut Workspace::default()),
            Err(Error::InvalidTopology)
        ));
    }

    #[test]
    fn skipped_depths_are_not_consecutive() {
        let mut b = chain(1.0);
        b.groups[2].depth = 3;
        b.groups[3].depth = 4;
        assert!(health(&b, 3, &mut Workspace::default())
            .expect("valid graph")
            .equal_error_chains
            .is_empty());
    }

    #[test]
    fn rising_errors_pass_in_any_table_order() {
        let mut b = chain(1.0);
        for (id, g) in b.groups.iter_mut().enumerate() {
            if g.error != f32::MAX {
                g.error = 1.0 + id as f32;
            }
        }
        assert!(
            health(&b, 2, &mut Workspace::default())
                .expect("valid")
                .passed
        );
        // Coarse-first tables, as a container stores them, give the same answer.
        let mut c = chain(1.0);
        c.groups.reverse();
        c.clusters.reverse();
        c.diagnostics.reverse();
        for (id, g) in c.groups.iter_mut().enumerate() {
            g.first_cluster = id as u32;
        }
        for (id, cl) in c.clusters.iter_mut().enumerate() {
            cl.group = id as u32;
            cl.refined = if id == 3 { -1 } else { id as i32 + 1 };
        }
        let report = health(&c, 3, &mut Workspace::default()).expect("valid");
        assert_eq!(report.equal_error_chains.len(), 1);
        assert_eq!(report.equal_error_chains[0].group, 1);
        assert!(!report.passed);
    }

    #[test]
    fn rejects_pins_with_recorded_or_inferred_reason() {
        let mut b = chain(0.0);
        b.groups[1].error = f32::MAX;
        b.diagnostics[1].status = GroupStatus::AlphaCoverage;
        let report = health(&b, 3, &mut Workspace::default()).expect("valid");
        assert!(!report.passed);
        assert_eq!(report.pinned_groups[0].reason, GroupStatus::AlphaCoverage);
        b.diagnostics.clear();
        let report = health(&b, 3, &mut Workspace::default()).expect("valid");
        assert_eq!(report.pinned_groups[0].reason, GroupStatus::UnknownPinned);
        assert_eq!(report.pinned_groups[1].reason, GroupStatus::Root);
        assert!(!report.passed);
    }
}
