//! Opt-in local repair; not in C++ meshoptimizer.
use crate::budget::Budget;
use crate::sanitize::{sort_work, steps};
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Positions, Workspace};
use alloc::vec::Vec;

const NONE: u32 = u32::MAX;

/// Explicit seam policy for welding; not in C++ meshoptimizer.
#[derive(Clone, Copy)]
pub enum WeldPolicy<'a> {
    /// Compare positions alone, potentially destroying attribute seams.
    PositionsOnly,
    /// Also require caller attribute equality between representative and vertex.
    /// The callback must be a deterministic equivalence relation. It is called
    /// only for geometrically eligible original indices, in deterministic order.
    AttributeEquality(&'a dyn Fn(u32, u32) -> bool),
}

/// Counts from welding; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct WeldReport {
    /// All input vertices, including unreferenced vertices.
    pub input_vertices: usize,
    /// Vertices retained as representatives.
    pub representatives: usize,
    /// Vertices mapped to an earlier representative.
    pub merged_vertices: usize,
}

/// Old-to-original-representative mapping; not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct WeldedVertices {
    /// One original index per input vertex; this is not a compact numbering.
    pub remap: Vec<u32>,
    /// Exact change counts.
    pub report: WeldReport,
}

/// Weld positions within a finite nonnegative Euclidean distance tolerance.
/// This extension is not in C++ meshoptimizer. No vertex is moved.
///
/// Visit original indices in ascending order. Choose the lowest original index
/// among eligible *representatives*, never an already merged vertex. Thus
/// merging is non-transitive: every member is within tolerance of its own
/// representative. At zero tolerance numeric equality is used (signed zeros
/// equal). Apply the remap to indices, then rerun [`crate::sanitize`], because
/// welding can create repeated corners and duplicate faces.
///
/// A sorted spatial grid and fixed neighbor order make selection deterministic.
/// Grid cells widen for extreme coordinate/tolerance ratios to keep cell keys
/// representable. Dense cells can cause quadratic candidate visits; every
/// visit, binary search and sort is charged to `max_work`. All scratch and
/// output capacity is charged to `max_bytes`; scratch is call-local.
///
/// # Errors
/// `InvalidParameter` for a negative/non-finite tolerance, `NumericalFailure`
/// for any non-finite input position, and `SizeOverflow`, `LimitExceeded` or
/// `AllocationFailed` for resource failures. Returns no partial mapping.
pub fn weld_vertices(
    positions: Positions<'_>,
    tolerance: f32,
    policy: WeldPolicy<'_>,
    workspace: &mut Workspace,
) -> Result<WeldedVertices, Error> {
    let mut work = workspace.begin();
    let result = weld_plan(
        positions,
        tolerance,
        policy,
        &mut Budget::new(workspace),
        &mut work,
    );
    workspace.finish(&work);
    result
}

/// Mapping length required by [`weld_vertices_into`]; not in C++ meshoptimizer.
pub const fn weld_vertices_bound(vertex_count: usize) -> usize {
    vertex_count
}

/// [`weld_vertices`] into caller storage; not in C++ meshoptimizer.
/// On success writes `positions.len()` entries and preserves the tail.
/// Every error leaves the entire destination unchanged. Stages the mapping in
/// budgeted scratch, so the caller-buffer form still requires heap storage.
/// # Errors
/// Same as [`weld_vertices`], plus `BufferTooSmall` after planning the mapping.
#[inline]
pub fn weld_vertices_into_generic<I: crate::Index>(
    destination: &mut [I],
    positions: Positions<'_>,
    tolerance: f32,
    policy: WeldPolicy<'_>,
    workspace: &mut Workspace,
) -> Result<WeldReport, Error> {
    I::with_mut(destination, workspace, |destination, workspace| {
        weld_vertices_into_u32(destination, positions, tolerance, policy, workspace)
    })
}

fn weld_vertices_into_u32(
    destination: &mut [u32],
    positions: Positions<'_>,
    tolerance: f32,
    policy: WeldPolicy<'_>,
    workspace: &mut Workspace,
) -> Result<WeldReport, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let plan = weld_plan(
            positions,
            tolerance,
            policy,
            &mut Budget::new(workspace),
            &mut work,
        )?;
        let out = destination
            .get_mut(..plan.remap.len())
            .ok_or(Error::BufferTooSmall)?;
        work.add(out.len())?;
        out.copy_from_slice(&plan.remap);
        Ok(plan.report)
    })();
    workspace.finish(&work);
    result
}

#[derive(Clone, Copy)]
struct Cell {
    key: [i64; 3],
    vertex: u32,
}
fn cell(p: [f32; 3], width: f64) -> [i64; 3] {
    if width == 0.0 {
        p.map(|v| if v == 0.0 { 0 } else { i64::from(v.to_bits()) })
    } else {
        p.map(|v| libm::floor(f64::from(v) / width) as i64)
    }
}
fn weld_plan(
    positions: Positions<'_>,
    tolerance: f32,
    policy: WeldPolicy<'_>,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<WeldedVertices, Error> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(Error::InvalidParameter);
    }
    let n = positions.len();
    u32::try_from(n).map_err(|_| Error::SizeOverflow)?;
    budget.ws.charge_retained()?;
    let mut largest = 0.0f64;
    for i in 0..n {
        work.add(1)?;
        for v in positions.get(i).ok_or(Error::InvalidParameter)? {
            if !v.is_finite() {
                return Err(Error::NumericalFailure);
            }
            largest = largest.max(f64::from(v).abs());
        }
    }
    // The factor two leaves room for division rounding at cell boundaries;
    // |key| <= 2^50 bounds that rounding below one quarter of a cell.
    let width = if tolerance == 0.0 {
        0.0
    } else {
        (2.0 * f64::from(tolerance)).max(largest / ((1u64 << 50) as f64))
    };
    work.add(n)?;
    work.add(sort_work(n)?)?;
    let mut cells = budget.reserved::<Cell>(n)?;
    for i in 0..n {
        cells.push(Cell {
            // Infallible: the scan above returned for any missing position.
            key: cell(positions.get(i).ok_or(Error::InvalidParameter)?, width),
            vertex: i as u32,
        });
    }
    cells.sort_unstable_by_key(|c| (c.key, c.vertex));
    let mut remap = budget.reserved::<u32>(n)?;
    let mut merged = 0;
    let radius = if tolerance == 0.0 { 0 } else { 1 };
    let squared = f64::from(tolerance) * f64::from(tolerance);
    for i in 0..n {
        work.add(1)?;
        let p = positions.get(i).ok_or(Error::InvalidParameter)?;
        let key = cell(p, width);
        let mut representative = i as u32;
        for x in -radius..=radius {
            for y in -radius..=radius {
                for z in -radius..=radius {
                    let neighbor = [key[0] + x, key[1] + y, key[2] + z];
                    work.add(steps(n))?;
                    let start = cells.partition_point(|c| c.key < neighbor);
                    for c in &cells[start..] {
                        work.add(1)?;
                        if c.key != neighbor {
                            break;
                        }
                        let j = c.vertex as usize;
                        if j >= representative as usize || remap[j] != c.vertex {
                            continue;
                        }
                        let q = positions.get(j).ok_or(Error::InvalidParameter)?;
                        let d = [0, 1, 2].map(|axis| f64::from(p[axis]) - f64::from(q[axis]));
                        if d[0] * d[0] + d[1] * d[1] + d[2] * d[2] <= squared
                            && match policy {
                                WeldPolicy::PositionsOnly => true,
                                WeldPolicy::AttributeEquality(eq) => eq(c.vertex, i as u32),
                            }
                        {
                            representative = c.vertex;
                        }
                    }
                }
            }
        }
        merged += usize::from(representative != i as u32);
        remap.push(representative);
    }
    Ok(WeldedVertices {
        remap,
        report: WeldReport {
            input_vertices: n,
            representatives: n - merged,
            merged_vertices: merged,
        },
    })
}

/// Orientation changes and residuals; not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct OrientReport {
    /// Components joined exclusively across two-face edges, including isolated faces.
    pub components: usize,
    /// Original face indices flipped, in ascending order.
    pub flipped_faces: Vec<u32>,
    /// Two-face edges still inconsistent after propagation, sorted by endpoints.
    /// These witness contradictory orientation constraints (non-orientability).
    pub conflicts: Vec<[u32; 2]>,
    /// Lowest face indices of components touching non-manifold edges or containing
    /// contradictory constraints, in ascending order.
    pub ambiguous_components: Vec<u32>,
}
/// Oriented indices with original face order; not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct OrientedMeshGeneric<I: crate::Index = u32> {
    /// Original triangles, with corners 1 and 2 exchanged for reported flips.
    pub indices: Vec<I>,
    /// Changes and unresolved components.
    pub report: OrientReport,
}

/// Propagate winding across edges with exactly two incident faces.
/// Not in C++ meshoptimizer. Each component is seeded by its lowest face
/// index, which stays unflipped. Breadth-first visits use original corner order.
/// Non-manifold edges never provide adjacency or orientation constraints;
/// their incident components are reported as ambiguous. A face incident to such
/// an edge can still flip due to a different, two-face edge. Contradictory
/// components retain their deterministic spanning-tree assignment and report
/// the residual conflict edges. No outward orientation is attempted.
///
/// # Errors
/// `InvalidTopology` for incomplete or repeated-index triangles,
/// `IndexOutOfBounds` for invalid references, or resource errors as in
/// [`weld_vertices`]. Input topology should first be sanitized. Scratch and
/// reports are linear in index count; sorted edges cost `O(n log n)` work.
#[inline]
pub fn orient_faces_generic<I: crate::Index>(
    indices: &[I],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<OrientedMeshGeneric<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        orient_faces_u32(indices, vertex_count, workspace)
            .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
    })
}

fn orient_faces_u32(
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<OrientedMeshGeneric, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mut budget = Budget::new(workspace);
        let (flip, report) = orient_plan(indices, vertex_count, &mut budget, &mut work)?;
        work.add(indices.len())?;
        let mut out = budget.copied(indices)?;
        apply_flips(&mut out, &flip);
        Ok(OrientedMeshGeneric {
            indices: out,
            report,
        })
    })();
    workspace.finish(&work);
    result
}
/// Index count required by [`orient_faces_into`]; not in C++ meshoptimizer.
pub const fn orient_faces_bound(index_count: usize) -> usize {
    index_count
}
/// [`orient_faces`] into caller storage; not in C++ meshoptimizer.
/// Writes the input index count, preserves the tail, and leaves all destination
/// entries unchanged on every error. Reports also use budgeted heap storage.
/// # Errors
/// Same as [`orient_faces`], plus `BufferTooSmall` after orientation planning.
#[inline]
pub fn orient_faces_into_generic<I: crate::Index>(
    destination: &mut [I],
    indices: &[I],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<OrientReport, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        I::with_mut(destination, workspace, |destination, workspace| {
            orient_faces_into_u32(destination, indices, vertex_count, workspace)
        })
    })
}

fn orient_faces_into_u32(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<OrientReport, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let (flip, report) = orient_plan(
            indices,
            vertex_count,
            &mut Budget::new(workspace),
            &mut work,
        )?;
        let out = destination
            .get_mut(..indices.len())
            .ok_or(Error::BufferTooSmall)?;
        work.add(indices.len())?;
        out.copy_from_slice(indices);
        apply_flips(out, &flip);
        Ok(report)
    })();
    workspace.finish(&work);
    result
}
fn apply_flips(out: &mut [u32], flip: &[i8]) {
    for (t, &f) in out.as_chunks_mut::<3>().0.iter_mut().zip(flip) {
        if f == 1 {
            t.swap(1, 2);
        }
    }
}

#[derive(Clone, Copy)]
struct Edge {
    key: [u32; 2],
    face: u32,
    corner: usize,
    forward: bool,
}
fn edges(
    indices: &[u32],
    vertex_count: usize,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<Vec<Edge>, Error> {
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    u32::try_from(vertex_count).map_err(|_| Error::SizeOverflow)?;
    checked_bytes(indices.len(), 4)?;
    u32::try_from(indices.len() / 3).map_err(|_| Error::SizeOverflow)?;
    budget.ws.charge_retained()?;
    for t in indices.as_chunks::<3>().0 {
        work.add(1)?;
        if t.iter().any(|&v| v as usize >= vertex_count) {
            return Err(Error::IndexOutOfBounds);
        }
        if t[0] == t[1] || t[1] == t[2] || t[2] == t[0] {
            return Err(Error::InvalidTopology);
        }
    }
    work.add(indices.len())?;
    work.add(sort_work(indices.len())?)?;
    let mut edges = budget.reserved::<Edge>(indices.len())?;
    for (f, t) in indices.as_chunks::<3>().0.iter().enumerate() {
        for c in 0..3 {
            let (a, b) = (t[c], t[(c + 1) % 3]);
            edges.push(Edge {
                key: [a.min(b), a.max(b)],
                face: f as u32,
                corner: c,
                forward: a < b,
            });
        }
    }
    edges.sort_unstable_by_key(|e| (e.key, e.face, e.corner));
    Ok(edges)
}
fn groups(edges: &[Edge]) -> impl Iterator<Item = &[Edge]> {
    let mut rest = edges;
    core::iter::from_fn(move || {
        if rest.is_empty() {
            return None;
        }
        let n = rest.partition_point(|e| e.key == rest[0].key);
        let (first, tail) = rest.split_at(n);
        rest = tail;
        Some(first)
    })
}
#[derive(Clone, Copy)]
struct Link {
    face: u32,
    parity: i8,
}
fn orient_plan(
    indices: &[u32],
    vertex_count: usize,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(Vec<i8>, OrientReport), Error> {
    let edges = edges(indices, vertex_count, budget, work)?;
    let n = indices.len() / 3;
    work.add(sort_work(edges.len())?)?; // group discovery plus all edge visits
    let mut links = budget.filled(
        n,
        [Link {
            face: NONE,
            parity: 0,
        }; 3],
    )?;
    let mut blocked = budget.filled(n, false)?;
    for group in groups(&edges) {
        if let [a, b] = group {
            let parity = i8::from(a.forward == b.forward);
            links[a.face as usize][a.corner] = Link {
                face: b.face,
                parity,
            };
            links[b.face as usize][b.corner] = Link {
                face: a.face,
                parity,
            };
        } else if group.len() > 2 {
            for e in group {
                blocked[e.face as usize] = true;
            }
        }
    }
    let mut flip = budget.filled(n, -1i8)?;
    let mut seeds = budget.filled(n, NONE)?;
    let mut ambiguous = budget.filled(n, false)?;
    let mut queue = budget.reserved::<u32>(n)?;
    let mut report = OrientReport {
        flipped_faces: budget.reserved(n)?,
        conflicts: budget.reserved(edges.len() / 2)?,
        ambiguous_components: budget.reserved(n)?,
        ..OrientReport::default()
    };
    for seed in 0..n {
        work.add(1)?;
        if flip[seed] != -1 {
            continue;
        }
        report.components += 1;
        queue.clear();
        queue.push(seed as u32);
        flip[seed] = 0;
        let mut head = 0;
        while head < queue.len() {
            work.add(1)?;
            let f = queue[head] as usize;
            head += 1;
            seeds[f] = seed as u32;
            ambiguous[seed] |= blocked[f];
            for link in links[f] {
                work.add(1)?;
                if link.face != NONE && flip[link.face as usize] == -1 {
                    flip[link.face as usize] = flip[f] ^ link.parity;
                    queue.push(link.face);
                }
            }
        }
    }
    work.add(sort_work(edges.len())?)?;
    for group in groups(&edges) {
        if let [a, b] = group {
            if (flip[a.face as usize] ^ flip[b.face as usize]) != i8::from(a.forward == b.forward) {
                report.conflicts.push(a.key);
                ambiguous[seeds[a.face as usize] as usize] = true;
            }
        }
    }
    work.add(n)?;
    for f in 0..n {
        if flip[f] == 1 {
            report.flipped_faces.push(f as u32);
        }
        if ambiguous[f] {
            report.ambiguous_components.push(f as u32);
        }
    }
    Ok((flip, report))
}

/// One boundary component decision; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct HoleLoop {
    /// Lowest original vertex index in this boundary component.
    pub vertex: u32,
    /// Number of boundary edges in the component.
    pub edges: usize,
}
/// Hole filling changes and refusals; not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct FillHolesReport {
    /// Original triangle count.
    pub input_triangles: usize,
    /// Triangle count after appending fills.
    pub output_triangles: usize,
    /// Filled loops, ordered by lowest vertex index.
    pub filled: Vec<HoleLoop>,
    /// Components exceeding the caller's maximum edge count.
    pub too_large: Vec<HoleLoop>,
    /// Projected loops with intersections, touching edges, coincident corners,
    /// zero area, or no nondegenerate ear triangulation.
    pub non_simple: Vec<HoleLoop>,
    /// Branching/inconsistently directed boundaries, boundaries touching defective
    /// edges, vertices with disconnected face fans, or loops outside the documented planarity tolerance.
    pub ambiguous: Vec<HoleLoop>,
}
/// Original indices plus filled faces; not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct FilledMeshGeneric<I: crate::Index = u32> {
    /// Original triangles followed by fills, using original vertices only.
    pub indices: Vec<I>,
    /// Exact decisions for each boundary component.
    pub report: FillHolesReport,
}

/// Fill small simple boundary loops with existing vertices only.
/// Not in C++ meshoptimizer. `max_edges` is inclusive; values below three fill
/// nothing. Original triangles and vertex streams are unchanged.
///
/// Components are considered in lowest-vertex order. Refusal precedence is:
/// too large, ambiguous topology, non-simple projection, nonplanarity. Each
/// boundary vertex must have one incoming and one outgoing boundary edge and
/// touch no non-manifold vertex, non-manifold edge, or inconsistent two-face edge. Start at the smallest
/// index and follow existing winding; append triangles in opposite winding.
///
/// Uses independently implemented [ear clipping (Eberly, sections 1–2)](
/// https://www.geometrictools.com/Documentation/TriangulationByEarClipping.pdf):
/// project onto the dominant plane of the loop's area normal, reject touching
/// or intersecting nonadjacent segments, and remove the first eligible ear in
/// loop order. Predicates use `f64`, not adaptive exact arithmetic. Distance
/// from the plane through the first vertex must be at most `1e-6` times the
/// largest coordinate extent. Nonplanar loops are reported as ambiguous.
/// Filling does not test intersections with other mesh surfaces, infer intended
/// openings, or promise watertightness. Orient/sanitize first as appropriate.
///
/// Work is bounded by `Limits`, including component searches, intersection
/// pairs, ear candidates and containment tests (worst case cubic in loop size).
/// Storage is linear in input indices plus appended indices and reports.
/// # Errors
/// Same topology/resource errors as [`orient_faces`], plus `NumericalFailure`
/// for non-finite referenced positions. Refused loops are successful report
/// entries; other loops may still be filled. An error returns no partial mesh.
#[inline]
pub fn fill_holes_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    max_edges: usize,
    workspace: &mut Workspace,
) -> Result<FilledMeshGeneric<I>, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        fill_holes_u32(indices, positions, max_edges, workspace)
            .and_then(|value| crate::index::Convert::<I>::convert(value, workspace))
    })
}

fn fill_holes_u32(
    indices: &[u32],
    positions: Positions<'_>,
    max_edges: usize,
    workspace: &mut Workspace,
) -> Result<FilledMeshGeneric, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let mut budget = Budget::new(workspace);
        let (added, report) = fill_plan(indices, positions, max_edges, &mut budget, &mut work)?;
        let n = indices
            .len()
            .checked_add(added.len())
            .ok_or(Error::SizeOverflow)?;
        work.add(n)?;
        let mut out = budget.reserved(n)?;
        out.extend_from_slice(indices);
        out.extend_from_slice(&added);
        Ok(FilledMeshGeneric {
            indices: out,
            report,
        })
    })();
    workspace.finish(&work);
    result
}
/// Conservative output index bound for [`fill_holes_into`]; not in C++ meshoptimizer.
/// Each boundary edge contributes at most three added indices.
/// # Errors
/// `SizeOverflow` if four times the input index count is not representable.
pub fn fill_holes_bound(index_count: usize) -> Result<usize, Error> {
    let n = index_count.checked_mul(4).ok_or(Error::SizeOverflow)?;
    checked_bytes(n, 4)?;
    Ok(n)
}
/// [`fill_holes`] into caller storage; not in C++ meshoptimizer.
/// Writes `report.output_triangles * 3` indices and preserves the tail. A
/// destination of [`fill_holes_bound`] entries suffices. All errors, including
/// insufficient capacity, leave the entire destination unchanged. Added faces
/// and reports are staged in budgeted heap storage before any write.
/// # Errors
/// Same as [`fill_holes`], plus `BufferTooSmall` after planning all loops.
#[inline]
pub fn fill_holes_into_generic<I: crate::Index>(
    destination: &mut [I],
    indices: &[I],
    positions: Positions<'_>,
    max_edges: usize,
    workspace: &mut Workspace,
) -> Result<FillHolesReport, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        I::with_mut(destination, workspace, |destination, workspace| {
            fill_holes_into_u32(destination, indices, positions, max_edges, workspace)
        })
    })
}

fn fill_holes_into_u32(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    max_edges: usize,
    workspace: &mut Workspace,
) -> Result<FillHolesReport, Error> {
    let mut work = workspace.begin();
    let result = (|| {
        let (added, report) = fill_plan(
            indices,
            positions,
            max_edges,
            &mut Budget::new(workspace),
            &mut work,
        )?;
        let n = indices
            .len()
            .checked_add(added.len())
            .ok_or(Error::SizeOverflow)?;
        let out = destination.get_mut(..n).ok_or(Error::BufferTooSmall)?;
        work.add(n)?;
        out[..indices.len()].copy_from_slice(indices);
        out[indices.len()..].copy_from_slice(&added);
        Ok(report)
    })();
    workspace.finish(&work);
    result
}

#[derive(Clone, Copy)]
struct End {
    vertex: u32,
    edge: usize,
}
fn end_range(ends: &[End], vertex: u32, work: &mut Work) -> Result<core::ops::Range<usize>, Error> {
    work.add(
        steps(ends.len())
            .checked_mul(2)
            .ok_or(Error::SizeOverflow)?,
    )?;
    Ok(ends.partition_point(|e| e.vertex < vertex)..ends.partition_point(|e| e.vertex <= vertex))
}
fn directed(e: &Edge) -> [u32; 2] {
    if e.forward {
        e.key
    } else {
        [e.key[1], e.key[0]]
    }
}
fn fill_plan(
    indices: &[u32],
    positions: Positions<'_>,
    max_edges: usize,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(Vec<u32>, FillHolesReport), Error> {
    let edges = edges(indices, positions.len(), budget, work)?;
    for &v in indices {
        work.add(1)?;
        // Infallible: `edges` returned `IndexOutOfBounds` for any index >= len.
        if !positions
            .get(v as usize)
            .ok_or(Error::IndexOutOfBounds)?
            .iter()
            .all(|x| x.is_finite())
        {
            return Err(Error::NumericalFailure);
        }
    }
    work.add(sort_work(edges.len())?)?;
    let mut boundary = budget.reserved::<Edge>(edges.len())?;
    let mut bad_vertices =
        budget.reserved::<u32>(edges.len().checked_mul(2).ok_or(Error::SizeOverflow)?)?;
    for group in groups(&edges) {
        match group {
            [e] => boundary.push(*e),
            [a, b] if a.forward != b.forward => (),
            _ => bad_vertices.extend_from_slice(&group[0].key),
        }
    }
    mark_disconnected_fans(&edges, indices, &mut bad_vertices, budget, work)?;
    work.add(sort_work(bad_vertices.len())?)?;
    bad_vertices.sort_unstable();
    let b = boundary.len();
    let end_count = b.checked_mul(2).ok_or(Error::SizeOverflow)?;
    work.add(b)?;
    work.add(sort_work(end_count)?)?;
    let mut ends = budget.reserved::<End>(end_count)?;
    for (i, e) in boundary.iter().enumerate() {
        for vertex in e.key {
            ends.push(End { vertex, edge: i });
        }
    }
    ends.sort_unstable_by_key(|e| (e.vertex, e.edge));
    let mut visited = budget.filled(b, false)?;
    let mut queue = budget.reserved::<usize>(b)?;
    let mut polygon = budget.reserved::<u32>(b)?;
    let mut projected = budget.reserved::<[f64; 2]>(b)?;
    let mut active = budget.reserved::<usize>(b)?;
    let mut added = budget.reserved::<u32>(b.checked_mul(3).ok_or(Error::SizeOverflow)?)?;
    let mut report = FillHolesReport {
        input_triangles: indices.len() / 3,
        output_triangles: indices.len() / 3,
        filled: budget.reserved(b)?,
        too_large: budget.reserved(b)?,
        non_simple: budget.reserved(b)?,
        ambiguous: budget.reserved(b)?,
    };
    for start in 0..b {
        work.add(1)?;
        if visited[start] {
            continue;
        }
        queue.clear();
        queue.push(start);
        visited[start] = true;
        let mut head = 0;
        let mut ambiguous = false;
        let mut vertex = u32::MAX;
        while head < queue.len() {
            work.add(1)?;
            let e = boundary[queue[head]];
            head += 1;
            for v in e.key {
                vertex = vertex.min(v);
                work.add(steps(bad_vertices.len()))?;
                ambiguous |= bad_vertices.binary_search(&v).is_ok();
                let range = end_range(&ends, v, work)?;
                ambiguous |= range.len() != 2;
                let mut incoming = 0;
                for end in &ends[range] {
                    work.add(1)?;
                    incoming += usize::from(directed(&boundary[end.edge])[1] == v);
                    if !visited[end.edge] {
                        visited[end.edge] = true;
                        queue.push(end.edge);
                    }
                }
                ambiguous |= incoming != 1;
            }
        }
        let decision = HoleLoop {
            vertex,
            edges: queue.len(),
        };
        if queue.len() > max_edges {
            report.too_large.push(decision);
            continue;
        }
        if ambiguous || queue.len() < 3 {
            report.ambiguous.push(decision);
            continue;
        }
        polygon.clear();
        let mut current = vertex;
        for _ in 0..queue.len() {
            work.add(1)?;
            polygon.push(current);
            let range = end_range(&ends, current, work)?;
            let e = ends[range]
                .iter()
                .find(|e| directed(&boundary[e.edge])[0] == current)
                // Infallible: each loop vertex has two ends and one incoming edge.
                .ok_or(Error::InvalidTopology)?;
            current = directed(&boundary[e.edge])[1];
        }
        let old = added.len();
        match triangulate(
            &polygon,
            positions,
            &mut projected,
            &mut active,
            &mut added,
            work,
        )? {
            LoopResult::Filled => report.filled.push(decision),
            LoopResult::NonSimple => {
                added.truncate(old);
                report.non_simple.push(decision);
            }
            LoopResult::Ambiguous => {
                added.truncate(old);
                report.ambiguous.push(decision);
            }
        }
    }
    report.output_triangles += added.len() / 3;
    Ok((added, report))
}

// A corner is a node in its vertex's face fan. Union by size and path
// compression keep adversarial chains bounded; all parent visits are metered.
fn fan_root(parent: &mut [usize], mut node: usize, work: &mut Work) -> Result<usize, Error> {
    loop {
        work.add(1)?;
        let next = parent[node];
        if next == node {
            return Ok(node);
        }
        parent[node] = parent[next];
        node = next;
    }
}
fn mark_disconnected_fans(
    edges: &[Edge],
    indices: &[u32],
    bad: &mut Vec<u32>,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(), Error> {
    let mut parent = budget.reserved::<usize>(indices.len())?;
    work.add(indices.len())?;
    parent.extend(0..indices.len());
    let mut size = budget.filled(indices.len(), 1usize)?;
    work.add(sort_work(edges.len())?)?;
    for group in groups(edges) {
        for pair in group.windows(2) {
            for vertex in group[0].key {
                let corner = |e: &Edge| {
                    e.face as usize * 3
                        + if directed(e)[0] == vertex {
                            e.corner
                        } else {
                            (e.corner + 1) % 3
                        }
                };
                let mut a = fan_root(&mut parent, corner(&pair[0]), work)?;
                let mut b = fan_root(&mut parent, corner(&pair[1]), work)?;
                if a != b {
                    if size[a] < size[b] {
                        core::mem::swap(&mut a, &mut b);
                    }
                    parent[b] = a;
                    size[a] += size[b];
                }
            }
        }
    }
    let mut corners = budget.reserved::<(u32, usize)>(indices.len())?;
    for (node, &v) in indices.iter().enumerate() {
        corners.push((v, fan_root(&mut parent, node, work)?));
    }
    work.add(sort_work(corners.len())?)?;
    corners.sort_unstable();
    let mut previous = None;
    let mut flagged = false;
    for (v, root) in corners.iter().copied() {
        work.add(1)?;
        match previous {
            Some((pv, pr)) if pv == v => {
                if root != pr && !flagged {
                    bad.push(v);
                    flagged = true;
                }
            }
            _ => {
                previous = Some((v, root));
                flagged = false;
            }
        }
    }
    budget.release(parent);
    budget.release(size);
    budget.release(corners);
    Ok(())
}

enum LoopResult {
    Filled,
    NonSimple,
    Ambiguous,
}
fn cross(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}
fn on_segment(a: [f64; 2], b: [f64; 2], p: [f64; 2]) -> bool {
    cross(a, b, p) == 0.0 && (0..2).all(|k| p[k] >= a[k].min(b[k]) && p[k] <= a[k].max(b[k]))
}
fn intersects(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    let (x, y, z, w) = (
        cross(a, b, c),
        cross(a, b, d),
        cross(c, d, a),
        cross(c, d, b),
    );
    ((x > 0.0 && y < 0.0 || x < 0.0 && y > 0.0) && (z > 0.0 && w < 0.0 || z < 0.0 && w > 0.0))
        || on_segment(a, b, c)
        || on_segment(a, b, d)
        || on_segment(c, d, a)
        || on_segment(c, d, b)
}
fn triangulate(
    polygon: &[u32],
    positions: Positions<'_>,
    projected: &mut Vec<[f64; 2]>,
    active: &mut Vec<usize>,
    added: &mut Vec<u32>,
    work: &mut Work,
) -> Result<LoopResult, Error> {
    // Infallible: polygon vertices are indices that `edges` bounds-checked.
    let point = |i: usize| {
        positions
            .get(polygon[i] as usize)
            .map(|p| p.map(f64::from))
            .ok_or(Error::IndexOutOfBounds)
    };
    let n = polygon.len();
    let origin = point(0)?;
    let mut normal = [0.0; 3];
    let mut extent = 0.0f64;
    let mut lo = origin;
    let mut hi = origin;
    for i in 0..n {
        work.add(1)?;
        let a = point(i)?;
        let b = point((i + 1) % n)?;
        let a = [0, 1, 2].map(|k| {
            lo[k] = lo[k].min(a[k]);
            hi[k] = hi[k].max(a[k]);
            a[k] - origin[k]
        });
        let b = [0, 1, 2].map(|k| b[k] - origin[k]);
        normal[0] += a[1] * b[2] - a[2] * b[1];
        normal[1] += a[2] * b[0] - a[0] * b[2];
        normal[2] += a[0] * b[1] - a[1] * b[0];
    }
    let mut axis = 0;
    for k in 0..3 {
        if normal[k].abs() > normal[axis].abs() {
            axis = k;
        }
        extent = extent.max(hi[k] - lo[k]);
    }
    if normal[axis] == 0.0 {
        return Ok(LoopResult::NonSimple);
    }
    projected.clear();
    active.clear();
    for i in 0..n {
        work.add(1)?;
        let p = point(i)?;
        projected.push([p[(axis + 1) % 3], p[(axis + 2) % 3]]);
        active.push(i);
    }
    for i in 0..n {
        for j in i + 1..n {
            work.add(1)?;
            if projected[i] == projected[j] {
                return Ok(LoopResult::NonSimple);
            }
            if j != i + 1
                && !(i == 0 && j == n - 1)
                && intersects(
                    projected[i],
                    projected[(i + 1) % n],
                    projected[j],
                    projected[(j + 1) % n],
                )
            {
                return Ok(LoopResult::NonSimple);
            }
        }
    }
    let length = libm::sqrt(normal.iter().map(|v| v * v).sum());
    for i in 0..n {
        work.add(1)?;
        let p = point(i)?;
        let distance = (0..3)
            .map(|k| (p[k] - origin[k]) * normal[k])
            .sum::<f64>()
            .abs()
            / length;
        if distance > 1e-6 * extent {
            return Ok(LoopResult::Ambiguous);
        }
    }
    let sign = if normal[axis] > 0.0 { 1.0 } else { -1.0 };
    while active.len() > 2 {
        let m = active.len();
        let mut ear = None;
        for i in 0..m {
            work.add(1)?;
            let (a, b, c) = (active[(i + m - 1) % m], active[i], active[(i + 1) % m]);
            if cross(projected[a], projected[b], projected[c]) * sign <= 0.0 {
                continue;
            }
            let mut contains = false;
            for &p in active.iter() {
                work.add(1)?;
                if p != a
                    && p != b
                    && p != c
                    && cross(projected[a], projected[b], projected[p]) * sign >= 0.0
                    && cross(projected[b], projected[c], projected[p]) * sign >= 0.0
                    && cross(projected[c], projected[a], projected[p]) * sign >= 0.0
                {
                    contains = true;
                    break;
                }
            }
            if !contains {
                ear = Some((i, a, b, c));
                break;
            }
        }
        let Some((i, a, b, c)) = ear else {
            return Ok(LoopResult::NonSimple);
        };
        work.add(active.len())?; // removal shifts and triangle emission
        added.extend_from_slice(&[polygon[a], polygon[c], polygon[b]]);
        active.remove(i);
    }
    Ok(LoopResult::Filled)
}

/// The source-compatible u32 entry point; see [`weld_vertices_into_generic`] for selectable index widths.
#[inline]
pub fn weld_vertices_into(
    destination: &mut [u32],
    positions: Positions<'_>,
    tolerance: f32,
    policy: WeldPolicy<'_>,
    workspace: &mut Workspace,
) -> Result<WeldReport, Error> {
    weld_vertices_into_generic::<u32>(destination, positions, tolerance, policy, workspace)
}

/// The source-compatible u32 entry point; see [`orient_faces_generic`] for selectable index widths.
#[inline]
pub fn orient_faces(
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<OrientedMesh, Error> {
    orient_faces_generic::<u32>(indices, vertex_count, workspace).map(Into::into)
}

/// The source-compatible u32 entry point; see [`orient_faces_into_generic`] for selectable index widths.
#[inline]
pub fn orient_faces_into(
    destination: &mut [u32],
    indices: &[u32],
    vertex_count: usize,
    workspace: &mut Workspace,
) -> Result<OrientReport, Error> {
    orient_faces_into_generic::<u32>(destination, indices, vertex_count, workspace)
}

/// The source-compatible u32 entry point; see [`fill_holes_generic`] for selectable index widths.
#[inline]
pub fn fill_holes(
    indices: &[u32],
    positions: Positions<'_>,
    max_edges: usize,
    workspace: &mut Workspace,
) -> Result<FilledMesh, Error> {
    fill_holes_generic::<u32>(indices, positions, max_edges, workspace).map(Into::into)
}

/// The source-compatible u32 entry point; see [`fill_holes_into_generic`] for selectable index widths.
#[inline]
pub fn fill_holes_into(
    destination: &mut [u32],
    indices: &[u32],
    positions: Positions<'_>,
    max_edges: usize,
    workspace: &mut Workspace,
) -> Result<FillHolesReport, Error> {
    fill_holes_into_generic::<u32>(destination, indices, positions, max_edges, workspace)
}

/// The source-compatible u32 form of [`OrientedMeshGeneric`].
/// Oriented indices with original face order; not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct OrientedMesh {
    /// Original triangles, with corners 1 and 2 exchanged for reported flips.
    pub indices: Vec<u32>,
    /// Changes and unresolved components.
    pub report: OrientReport,
}
impl From<OrientedMeshGeneric<u32>> for OrientedMesh {
    #[inline]
    fn from(value: OrientedMeshGeneric<u32>) -> Self {
        Self {
            indices: value.indices,
            report: value.report,
        }
    }
}

impl From<OrientedMesh> for OrientedMeshGeneric<u32> {
    #[inline]
    fn from(value: OrientedMesh) -> Self {
        Self {
            indices: value.indices,
            report: value.report,
        }
    }
}

/// The source-compatible u32 form of [`FilledMeshGeneric`].
/// Original indices plus filled faces; not in C++ meshoptimizer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct FilledMesh {
    /// Original triangles followed by fills, using original vertices only.
    pub indices: Vec<u32>,
    /// Exact decisions for each boundary component.
    pub report: FillHolesReport,
}
impl From<FilledMeshGeneric<u32>> for FilledMesh {
    #[inline]
    fn from(value: FilledMeshGeneric<u32>) -> Self {
        Self {
            indices: value.indices,
            report: value.report,
        }
    }
}

impl From<FilledMesh> for FilledMeshGeneric<u32> {
    #[inline]
    fn from(value: FilledMesh) -> Self {
        Self {
            indices: value.indices,
            report: value.report,
        }
    }
}
