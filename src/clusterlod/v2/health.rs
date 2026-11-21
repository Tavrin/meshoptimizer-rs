//! Structural cooker gate over a built DAG or an opened MDAG 2 container. It
//! inspects stored errors, pins and triangle counts; it does not certify
//! sampled geometric accuracy.
use super::super::dag_format::v2::{Reader, MAX_LEVELS};
use super::*;

/// Why a group stopped simplifying. Serialized containers do not retain these
/// reasons; [`health_from_reader`] infers only [`GroupStatus::Root`] and
/// [`GroupStatus::UnknownPinned`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
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
    /// Remaining geometry pinned in the last available format level.
    DepthLimit,
    /// Pinned group whose build-time reason was not retained.
    UnknownPinned,
}

/// Build-time error terms of one group, before the writer adds quantization
/// displacement. The original-source term includes the accumulated step floor,
/// so it is not an independent measurement of raw deviation.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct GroupDiagnostics {
    /// Acceptance or pin reason.
    pub status: GroupStatus,
    /// Simplifier's error estimate in mesh units.
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
    /// The final stored error is a real deviation: an unfloored
    /// original-source geometric sample reaches it, or it is a measured
    /// child's error inherited unchanged. False when anything else sets it:
    /// the step deviation against the step's input, the regular quadric floor,
    /// the normal-loss price or `simplify_error_merge_additive`. Never set on
    /// a pinned group.
    pub error_measured: bool,
}

/// Error and triangle totals for one occupied depth.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct EqualErrorChain {
    /// Coarsest group of this path, in table order.
    pub group: u32,
    /// Number of consecutive depths in this path.
    pub levels: u32,
    /// Exactly equal stored error throughout the path.
    pub error: f32,
    /// Build diagnostics show the coarsest group's error is a sampled
    /// deviation ([`GroupDiagnostics::error_measured`]). Such a plateau is
    /// carried by monotone inheritance of a real deviation and does not fail
    /// the report. Always false without diagnostics.
    pub measured: bool,
}

/// A pinned group with its recorded or inferred reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PinnedGroup {
    /// Group in table order.
    pub group: u32,
    /// Root or rejection reason.
    pub reason: GroupStatus,
}

/// Structural quality report suitable for a cooker admission record.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct HealthReport {
    /// Per-depth table, including terminal pinned geometry.
    pub depths: Vec<DepthHealth>,
    /// Equal-error paths at or above the requested minimum length.
    pub equal_error_chains: Vec<EqualErrorChain>,
    /// All pinned groups, including expected roots.
    pub pinned_groups: Vec<PinnedGroup>,
    /// True when every reported chain is measured and every pinned group is a
    /// root.
    pub passed: bool,
}

/// Inspect a built DAG without changing its errors or topology.
///
/// The rule: the report fails on any pinned group that is not a final root,
/// and on any unmeasured *equal-error chain*: a finite positive stored error that is
/// exactly equal along at least `minimum_levels` consecutive linked depths
/// (each link joins a group to a parent one depth coarser). A link reduces
/// triangles when the parent stores fewer than the sum over its distinct
/// children. Intermediate levels on a plateau have no error interval of
/// their own; the coarsest level becomes eligible at the shared threshold.
/// Zero-error chains (including exact planar reductions) and the final root
/// pass. Three levels is the recommended cooker rule. Reported chains can
/// overlap: each reports its coarsest group and its length.
///
/// Stored errors alone cannot separate a plateau started by an inflated
/// heuristic estimate from a real sampled deviation that monotone inheritance
/// must carry to coarser groups. Without diagnostics every chain fails. With
/// them ([`health_with_diagnostics`]), a chain whose coarsest error is
/// [`GroupDiagnostics::error_measured`] is still reported, marked
/// [`EqualErrorChain::measured`], and does not fail the report.
///
/// The final root is inferred from topology as for [`health_from_reader`].
/// Use [`health_with_diagnostics`] to include recorded pin reasons and
/// per-depth counts of partial and sloppy steps.
///
/// This is a structural check on stored values. It does not prove geometric
/// accuracy, and a failing report is evidence to inspect the asset and the
/// build policy, not a request to raise errors artificially.
///
/// # Errors
/// `InvalidParameter` if `minimum_levels` is outside `2..=64`;
/// `InvalidTopology` for malformed tables; allocation and workspace limit
/// errors.
pub fn health(
    built: &Built,
    minimum_levels: u32,
    workspace: &mut Workspace,
) -> Result<HealthReport, Error> {
    health_with_diagnostics(built, &[], minimum_levels, workspace)
}

/// Inspect a built DAG with its optional per-group diagnostics, in emission
/// order as returned by [`super::build_with_diagnostics`]. This is the
/// cooker admission report: only it can pass a measured plateau.
///
/// # Errors
/// Same as [`health()`], plus `InvalidTopology` for a nonempty diagnostics
/// table of the wrong length or a recorded root inconsistent with the graph.
pub fn health_with_diagnostics(
    built: &Built,
    diagnostics: &[GroupDiagnostics],
    minimum_levels: u32,
    workspace: &mut Workspace,
) -> Result<HealthReport, Error> {
    validate_minimum_levels(minimum_levels)?;
    let mut ctx = Context::new(workspace)?;
    report(
        &built.groups,
        &built.clusters,
        diagnostics,
        minimum_levels,
        0.0,
        &mut ctx,
    )
}

/// Inspect an opened MDAG 2 container with the rule of [`health()`].
///
/// `cluster_triangles` holds the triangle count of every cluster in container
/// order, as `ClusterDesc::triangle_count` from the transcoded pages. Stored
/// errors include the writer's quantization displacement. Errors at or below
/// that displacement count as zero for the chain test; depth statistics still
/// show the stored values. Containers do not record whether an error was
/// measured, so every chain fails, as in [`health()`]. The builder emits its root only when a single
/// cluster remains, alone at a new depth (or depth zero for a one-cluster
/// input). A pin must be the sole coarsest group, have one cluster and have no
/// parent to be inferred as a root. Other pins lack a recorded reason and
/// report [`GroupStatus::UnknownPinned`].
///
/// # Errors
/// `InvalidParameter` if `minimum_levels` is outside `2..=64` or the triangle table length
/// differs from the container's cluster count; `InvalidTopology` for a group
/// or cluster record the reader cannot return; allocation and workspace limit
/// errors.
pub fn health_from_reader(
    reader: &Reader<'_>,
    cluster_triangles: &[u32],
    minimum_levels: u32,
    workspace: &mut Workspace,
) -> Result<HealthReport, Error> {
    validate_minimum_levels(minimum_levels)?;
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
    let result = report(
        &groups,
        &clusters,
        &[],
        minimum_levels,
        mesh.max_displacement,
        &mut ctx,
    );
    ctx.free(groups)?;
    ctx.free(clusters)?;
    result
}

fn validate_minimum_levels(minimum_levels: u32) -> Result<(), Error> {
    if !(2..=MAX_LEVELS).contains(&minimum_levels) {
        return Err(Error::InvalidParameter);
    }
    Ok(())
}

fn report(
    groups: &[FlatDagGroup],
    clusters: &[FlatDagCluster],
    diagnostics: &[GroupDiagnostics],
    minimum_levels: u32,
    zero_error: f32,
    ctx: &mut Context<'_>,
) -> Result<HealthReport, Error> {
    let n = groups.len();
    if !diagnostics.is_empty() && diagnostics.len() != n {
        return Err(Error::InvalidTopology);
    }
    let mut triangles = ctx.alloc::<u64>(n)?;
    let mut parents = ctx.alloc::<bool>(n)?;
    let mut order = ctx.alloc::<u32>(n)?;
    let mut max_depth = 0;
    let mut coarsest_groups = 0;
    for (id, g) in groups.iter().enumerate() {
        ctx.tick(1)?;
        if g.depth < 0 || g.depth as u32 >= MAX_LEVELS || !g.error.is_finite() || g.error < 0.0 {
            return Err(Error::InvalidTopology);
        }
        // A measured claim must name the stored value, or it is a forged table.
        if let Some(d) = diagnostics.get(id).filter(|d| d.error_measured) {
            if g.error == f32::MAX
                || ![d.step_error, d.source_error, d.inherited_error].contains(&g.error)
            {
                return Err(Error::InvalidTopology);
            }
        }
        if g.depth as u32 > max_depth {
            max_depth = g.depth as u32;
            coarsest_groups = 0;
        }
        coarsest_groups += usize::from(g.depth as u32 == max_depth);
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
    // Several replacement clusters can point to the same child. Count its
    // input triangles once per parent, including children off the plateau.
    let mut seen_in_parent = ctx.alloc::<usize>(n)?;
    let mut report = HealthReport::default();
    for &id in order.iter() {
        let id = id as usize;
        let g = &groups[id];
        levels[id] = 1;
        if g.error <= zero_error || g.error == f32::MAX {
            continue;
        }
        let mut child_triangles = 0u64;
        let mut previous_levels = 0;
        let span = g.first_cluster as usize..g.first_cluster as usize + g.cluster_count as usize;
        for c in clusters[span].iter().filter(|c| c.refined >= 0) {
            ctx.tick(1)?;
            let child = c.refined as usize;
            if seen_in_parent[child] != id + 1 {
                seen_in_parent[child] = id + 1;
                child_triangles += triangles[child];
                if groups[child].depth + 1 == g.depth && groups[child].error == g.error {
                    previous_levels = previous_levels.max(levels[child]);
                }
            }
        }
        if child_triangles > triangles[id] {
            levels[id] += previous_levels;
        }
        if levels[id] >= minimum_levels {
            grow_output(ctx, &mut report.equal_error_chains, 1)?;
            report.equal_error_chains.push(EqualErrorChain {
                group: id as u32,
                levels: levels[id],
                error: g.error,
                measured: diagnostics.get(id).is_some_and(|d| d.error_measured),
            });
        }
    }
    ctx.free(seen_in_parent)?;
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
                let root = depth == max_depth
                    && coarsest_groups == 1
                    && g.cluster_count == 1
                    && !parents[id];
                let reason = match diagnostic {
                    Some(d) => d.status,
                    None if root => GroupStatus::Root,
                    None => GroupStatus::UnknownPinned,
                };
                if reason == GroupStatus::Root && !root {
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
    report.passed = report.equal_error_chains.iter().all(|c| c.measured)
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
    }

    #[test]
    fn only_diagnostics_can_pass_a_measured_plateau() {
        let b = chain(1.0);
        let mut diagnostics = [GroupDiagnostics::default(); 4];
        diagnostics[3].status = GroupStatus::Root;
        let report =
            health_with_diagnostics(&b, &diagnostics, 3, &mut Workspace::default()).expect("valid");
        assert!(!report.passed && !report.equal_error_chains[0].measured);
        diagnostics[2].error_measured = true;
        diagnostics[2].source_error = 1.0;
        let report =
            health_with_diagnostics(&b, &diagnostics, 3, &mut Workspace::default()).expect("valid");
        assert_eq!(report.equal_error_chains.len(), 1);
        assert!(report.passed && report.equal_error_chains[0].measured);
        assert!(
            !health(&b, 3, &mut Workspace::default())
                .expect("valid")
                .passed
        );
    }

    /// Diagnostics as the builder commits them: groups in emission order, each
    /// with its unfloored geometric `sample` and the terms it stored.
    fn diagnose(b: &Built, samples: &[f32], steps: &[f32]) -> Vec<GroupDiagnostics> {
        let mut out: Vec<GroupDiagnostics> = Vec::new();
        for (id, g) in b.groups.iter().enumerate() {
            let span = g.first_cluster as usize..(g.first_cluster + g.cluster_count) as usize;
            let children: Vec<(f32, bool)> = b.clusters[span]
                .iter()
                .filter(|c| c.refined >= 0)
                .map(|c| {
                    let child = c.refined as usize;
                    (b.groups[child].error, out[child].error_measured)
                })
                .collect();
            let inherited = children.iter().fold(0f32, |m, &(e, _)| m.max(e));
            out.push(GroupDiagnostics {
                step_error: steps[id],
                source_error: samples[id].max(steps[id]),
                inherited_error: inherited,
                error_measured: super::error_is_measured(
                    g.error,
                    samples[id],
                    inherited,
                    &children,
                ),
                ..GroupDiagnostics::default()
            });
        }
        out[3].status = GroupStatus::Root;
        out
    }

    fn passes(b: &Built, d: &[GroupDiagnostics], levels: u32) -> bool {
        health_with_diagnostics(b, d, levels, &mut Workspace::default())
            .expect("valid")
            .passed
    }

    #[test]
    fn quadric_floor_origin_stays_unmeasured_through_inheritance() {
        // Every sample is far below the stored 1.0: the origin is a floor, and
        // the groups that inherit it have no measured child.
        let b = chain(1.0);
        let d = diagnose(&b, &[0.3, 0.3, 0.3, 0.0], &[0.3, 0.3, 0.3, 0.0]);
        assert!(d.iter().all(|d| !d.error_measured));
        assert!(!passes(&b, &d, 3));
        // The same chain whose origin was sampled at the stored value passes.
        let d = diagnose(&b, &[1.0, 0.3, 0.3, 0.0], &[0.3, 0.3, 0.3, 0.0]);
        assert!(d[0].error_measured && d[1].error_measured && d[2].error_measured);
        assert!(passes(&b, &d, 3));
    }

    #[test]
    fn inheritance_needs_a_sampled_child_with_the_equal_error() {
        let measured = super::super::error_is_measured;
        // An equal but unsampled child, beside a sampled child with less error.
        assert!(!measured(1.0, 0.5, 1.0, &[(1.0, false), (0.5, true)]));
        assert!(measured(1.0, 0.5, 1.0, &[(1.0, true), (0.5, false)]));
        // Equal child error is not inheritance when the group stores more.
        assert!(!measured(1.5, 0.5, 1.0, &[(1.0, true)]));
        assert!(!measured(f32::MAX, f32::MAX, f32::MAX, &[(f32::MAX, true)]));
    }

    #[test]
    fn a_step_inflated_plateau_below_the_first_depth_fails() {
        // Depth 1 stores its step deviation against the step's input (1.0),
        // while the original-source sample is 0.4; depth 2 inherits it.
        let mut b = chain(1.0);
        b.groups[0].error = 0.2;
        let steps = [0.2, 1.0, 0.3, 0.0];
        let d = diagnose(&b, &[0.2, 0.4, 0.3, 0.0], &steps);
        assert!(d[0].error_measured && !d[1].error_measured && !d[2].error_measured);
        assert!(!passes(&b, &d, 2));
        // When the original source reaches the stored value, the plateau passes.
        let d = diagnose(&b, &[0.2, 1.0, 0.3, 0.0], &steps);
        assert!(d[1].error_measured && d[2].error_measured);
        assert!(passes(&b, &d, 2));
    }

    #[test]
    fn measured_claims_must_name_the_stored_error() {
        let b = chain(1.0);
        let mut d = [GroupDiagnostics::default(); 4];
        d[3].status = GroupStatus::Root;
        d[2].error_measured = true;
        let check = |d: &[GroupDiagnostics], b: &Built| {
            health_with_diagnostics(b, d, 3, &mut Workspace::default()).map(|r| r.passed)
        };
        assert_eq!(check(&d, &b), Err(Error::InvalidTopology));
        d[2].inherited_error = 1.0;
        assert_eq!(check(&d, &b), Ok(true));
        // A pinned group cannot be measured, even at an equal-looking value.
        d[3].error_measured = true;
        d[3].step_error = f32::MAX;
        assert_eq!(check(&d, &b), Err(Error::InvalidTopology));
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
        let mut diagnostics = [GroupDiagnostics::default(); 4];
        diagnostics[1].status = GroupStatus::AlphaCoverage;
        diagnostics[3].status = GroupStatus::Root;
        let report =
            health_with_diagnostics(&b, &diagnostics, 3, &mut Workspace::default()).expect("valid");
        assert!(!report.passed);
        assert_eq!(report.pinned_groups[0].reason, GroupStatus::AlphaCoverage);
        let report = health(&b, 3, &mut Workspace::default()).expect("valid");
        assert_eq!(report.pinned_groups[0].reason, GroupStatus::UnknownPinned);
        assert_eq!(report.pinned_groups[1].reason, GroupStatus::Root);
        assert!(!report.passed);
    }
}
