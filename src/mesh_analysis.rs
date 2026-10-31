//! Read-only mesh topology analysis. This module is an extension: it is not in
//! C++ meshoptimizer, and no existing function's behaviour depends on it.

use crate::budget::Budget;
use crate::sanitize::{canonical, screen, sort_work, steps};
use crate::workspace::{checked_bytes, Work};
use crate::{Error, Positions, SanitizeOptions, SanitizeReport, Workspace};
use alloc::vec::Vec;

/// Options for [`analyze_mesh`]. Not in C++ meshoptimizer.
///
/// The default, [`Self::EMPTY`], analyzes index topology (two corners are the
/// same vertex only when they use the same index) and lists nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AnalyzeOptions {
    weld_positions: bool,
    list_limit: usize,
}
impl AnalyzeOptions {
    /// Index topology, counts only.
    pub const EMPTY: Self = Self {
        weld_positions: false,
        list_limit: 0,
    };
    /// Analyze position-coincident topology instead of index topology:
    /// vertices with equal positions count as one vertex, using the same
    /// equality as [`SanitizeOptions::WELD_POSITIONS`] (exact, with `0.0` and
    /// `-0.0` equal). No tolerance is applied and no index is rewritten.
    pub const fn with_weld_positions(self, enabled: bool) -> Self {
        Self {
            weld_positions: enabled,
            ..self
        }
    }
    /// List at most `limit` entries of each kind in [`MeshAnalysis::lists`].
    /// Zero, the default, lists nothing; counts are always complete.
    pub const fn with_list_limit(self, limit: usize) -> Self {
        Self {
            list_limit: limit,
            ..self
        }
    }
    /// Whether position-coincident topology is analyzed.
    pub const fn weld_positions(self) -> bool {
        self.weld_positions
    }
    /// Maximum entries listed of each kind.
    pub const fn list_limit(self) -> usize {
        self.list_limit
    }
}

/// One connected set of boundary edges from [`analyze_mesh`]. Not in C++
/// meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct BoundaryComponent {
    /// Smallest vertex index in the component.
    pub vertex: u32,
    /// Boundary edges in the component.
    pub edges: usize,
}

/// Bounded defect lists from [`analyze_mesh`]. Not in C++ meshoptimizer.
///
/// Each list holds at most [`AnalyzeOptions::list_limit`] entries: the first
/// ones in ascending order of their vertex indices (edges by smaller, then
/// larger, vertex index). With [`AnalyzeOptions::with_weld_positions`], a
/// vertex is named by the smallest referenced index among those sharing its
/// position; unreferenced positions are not scanned.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct AnalysisLists {
    /// Boundary edges as `[from, to]` in the winding of their one face.
    pub boundary_edges: Vec<[u32; 2]>,
    /// Boundary components, ordered by [`BoundaryComponent::vertex`].
    pub boundary_components: Vec<BoundaryComponent>,
    /// Edges with more than two faces, as `[smaller, larger]`.
    pub non_manifold_edges: Vec<[u32; 2]>,
    /// Vertices whose faces form more than one fan.
    pub non_manifold_vertices: Vec<u32>,
    /// Edges with exactly two incident faces using the same direction, as `[smaller, larger]`.
    pub orientation_conflicts: Vec<[u32; 2]>,
}

/// Topology report from [`analyze_mesh`]. Not in C++ meshoptimizer.
///
/// Triangles are first classified as in [`sanitize`](crate::sanitize).
/// Triangles with an out-of-range index, a non-finite referenced position or a
/// repeated index are counted and left out of the topology; so are triangles
/// with coincident corners when positions are welded, since their corners
/// then repeat a vertex. The remaining [`Self::triangles`], duplicates
/// included, form the analyzed topology, so a duplicate face shows up as the
/// edge defects it causes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct MeshAnalysis {
    /// Triangles in the input.
    pub input_triangles: usize,
    /// Left out: an index is at or beyond the vertex count.
    pub out_of_range: usize,
    /// Left out: a referenced position has a NaN or infinite component.
    pub non_finite: usize,
    /// Left out: two corners use the same index.
    pub repeated_index: usize,
    /// Triangles with distinct indices and two corners at equal positions
    /// (signed zeros equal). Left out only with welded positions; otherwise
    /// counted and analyzed.
    pub coincident_corners: usize,
    /// Analyzed triangles with the same corners and winding as an earlier
    /// analyzed triangle, after a cyclic rotation. The first winding is kept;
    /// later copies of either winding count here.
    /// `sanitize` with `IGNORE_ORIENTATION` classifies copies differently.
    pub duplicate: usize,
    /// Analyzed triangles that are the first with the opposite winding of an
    /// earlier analyzed triangle with the same corners. Later copies of that
    /// winding count as `duplicate`.
    pub reversed_duplicate: usize,
    /// Triangles in the analyzed topology.
    pub triangles: usize,
    /// Distinct vertices used by analyzed triangles.
    pub vertices: usize,
    /// Distinct undirected edges of analyzed triangles.
    pub edges: usize,
    /// Edges with exactly one face.
    pub boundary_edges: usize,
    /// Connected sets of boundary edges, joined at shared vertices. Equal to
    /// the number of holes and borders when each boundary vertex has one
    /// incoming and one outgoing boundary edge; borders that touch at a vertex
    /// count as one.
    pub boundary_components: usize,
    /// Edges with more than two faces.
    pub non_manifold_edges: usize,
    /// Vertices whose faces form more than one fan, where two faces around a
    /// vertex are in one fan when a chain of faces sharing edges at that
    /// vertex connects them. A bow-tie vertex has two fans.
    pub non_manifold_vertices: usize,
    /// Edges with exactly two incident faces using the same direction.
    pub orientation_conflicts: usize,
    /// Sets of analyzed triangles connected through shared vertices.
    pub components: usize,
    /// Triangles in the largest component, or zero.
    pub largest_component: usize,
    /// Triangles in the smallest component, or zero.
    pub smallest_component: usize,
    /// Bounded lists of the counted defects.
    pub lists: AnalysisLists,
}

/// Report the topology of a triangle list without changing anything.
/// Not in C++ meshoptimizer.
///
/// Counts boundary edges and components, non-manifold edges and vertices,
/// orientation conflicts, connected components, and the invalid, degenerate
/// and duplicate triangles; [`MeshAnalysis`] defines each count. With
/// [`AnalyzeOptions::with_list_limit`], also lists the first defects of each
/// kind. Index topology is the default; position-coincident topology needs
/// [`AnalyzeOptions::with_weld_positions`]. A boundary is not necessarily a
/// defect: open sheets have one by design.
///
/// ```
/// use meshoptimizer_rs::{analyze_mesh, AnalyzeOptions, Positions, Workspace};
///
/// // A square from two triangles: five edges, four of them on one border.
/// let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
/// let indices = [0, 1, 2, 0, 2, 3];
/// let mut workspace = Workspace::default();
/// let options = AnalyzeOptions::EMPTY.with_list_limit(8);
/// let analysis = analyze_mesh(&indices, Positions::from_packed(&positions), options, &mut workspace)?;
/// assert_eq!((analysis.edges, analysis.boundary_edges, analysis.boundary_components), (5, 4, 1));
/// assert_eq!(analysis.lists.boundary_edges, [[0, 1], [3, 0], [1, 2], [2, 3]]);
/// assert_eq!((analysis.non_manifold_edges, analysis.components), (0, 1));
/// # Ok::<(), meshoptimizer_rs::Error>(())
/// ```
///
/// # Errors
/// `InvalidTopology` if the index count is not a multiple of three.
/// `SizeOverflow` for size overflow, including more than `u32::MAX` indices,
/// `LimitExceeded` for exhausted limits, or `AllocationFailed` for failed
/// reservation. Invalid triangles are counted, never an error.
///
/// Work is `O(n log n)` in the number of triangles whatever the input: edges,
/// duplicates and welded positions are grouped by sorting, not hashing, and
/// fans, components and boundary components use union-find with union by size. Scratch
/// storage is proportional to the analyzed triangles, never to the position
/// count; each list holds at most the list limit.
#[inline]
pub fn analyze_mesh_generic<I: crate::Index>(
    indices: &[I],
    positions: Positions<'_>,
    options: AnalyzeOptions,
    workspace: &mut Workspace,
) -> Result<MeshAnalysis, Error> {
    I::with_input(indices, workspace, |indices, workspace| {
        analyze_mesh_u32(indices, positions, options, workspace)
    })
}

fn analyze_mesh_u32(
    indices: &[u32],
    positions: Positions<'_>,
    options: AnalyzeOptions,
    workspace: &mut Workspace,
) -> Result<MeshAnalysis, Error> {
    let mut work = workspace.begin();
    let result = analyze(
        indices,
        positions,
        options,
        &mut Budget::new(workspace),
        &mut work,
    );
    workspace.finish(&work);
    result
}

fn analyze(
    indices: &[u32],
    positions: Positions<'_>,
    options: AnalyzeOptions,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<MeshAnalysis, Error> {
    if !indices.len().is_multiple_of(3) {
        return Err(Error::InvalidTopology);
    }
    checked_bytes(indices.len(), 4)?;
    // Corners are numbered in u32.
    u32::try_from(indices.len()).map_err(|_| Error::SizeOverflow)?;
    let triangles = indices.as_chunks::<3>().0;
    budget.ws.charge_retained()?;
    let weld = options.weld_positions;
    let mut screened = SanitizeReport::default();
    let policy = if weld {
        SanitizeOptions::COINCIDENT_CORNERS
    } else {
        SanitizeOptions::EMPTY
    };
    let mut candidates = 0;
    for triangle in triangles {
        let mut keep = [false];
        candidates += screen(
            policy,
            core::slice::from_ref(triangle),
            &mut keep,
            &mut screened,
            work,
            |i| positions.get(i as usize),
        )?;
    }
    let mut report = MeshAnalysis {
        input_triangles: triangles.len(),
        out_of_range: screened.out_of_range,
        non_finite: screened.non_finite,
        repeated_index: screened.repeated_index,
        coincident_corners: screened.coincident_corners,
        triangles: candidates,
        ..MeshAnalysis::default()
    };
    if candidates == 0 {
        return Ok(report);
    }
    // Collect only screened triangles; the second screening pass is charged too.
    let mut kept = budget.reserved::<[u32; 3]>(candidates)?;
    let mut ignored = SanitizeReport::default();
    for triangle in triangles {
        let mut keep = [false];
        screen(
            policy,
            core::slice::from_ref(triangle),
            &mut keep,
            &mut ignored,
            work,
            |i| positions.get(i as usize),
        )?;
        if keep[0] {
            kept.push(*triangle);
        }
    }
    if !weld {
        work.add(candidates)?;
        for &[a, b, c] in &kept {
            let [pa, pb, pc] = [a, b, c].map(|i| positions.get(i as usize));
            if pa == pb || pa == pc || pb == pc {
                report.coincident_corners += 1;
            }
        }
    }
    let (faces, names) = number_vertices(&kept, weld, positions, budget, work)?;
    budget.release(kept);
    count_duplicates(&faces, &mut report, budget, work)?;
    let lists = topology(
        &faces,
        &names,
        options.list_limit,
        &mut report,
        budget,
        work,
    )?;
    report.lists = lists;
    budget.release(faces);
    budget.release(names);
    Ok(report)
}

/// Number the vertices of the analyzed triangles `0..unique` in ascending
/// index order, and return each triangle's vertex numbers with the input index
/// of every number. With `weld`, every corner takes the number of the
/// smallest index sharing its position, so numbers stay in index order.
fn number_vertices(
    triangles: &[[u32; 3]],
    weld: bool,
    positions: Positions<'_>,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(Vec<[u32; 3]>, Vec<u32>), Error> {
    let candidates = triangles.len();
    let corner_count = times(candidates, 3)?;
    work.add(corner_count)?;
    work.add(sort_work(corner_count)?)?;
    let mut referenced = budget.reserved::<u32>(corner_count)?;
    for triangle in triangles {
        referenced.extend_from_slice(triangle);
    }
    referenced.sort_unstable();
    referenced.dedup();
    work.add(referenced.len())?;
    let names = budget.copied(&referenced)?;
    budget.release(referenced);
    let welded = if weld {
        Some(weld_numbers(&names, positions, budget, work)?)
    } else {
        None
    };
    work.add(times(corner_count, steps(names.len()))?)?;
    let mut faces = budget.reserved::<[u32; 3]>(candidates)?;
    for triangle in triangles {
        let mut face = [0; 3];
        for (corner, &vertex) in face.iter_mut().zip(triangle) {
            let id = names
                .binary_search(&vertex)
                .map_err(|_| Error::IndexOutOfBounds)?;
            *corner = welded.as_ref().map_or(id as u32, |w| w[id]);
        }
        faces.push(face);
    }
    if let Some(welded) = welded {
        budget.release(welded);
    }
    Ok((faces, names))
}

/// For every vertex number, the smallest number with an equal position.
/// Positions are grouped by sorting their bit patterns with `-0.0` folded
/// into `0.0`; they are finite here, so equal patterns are exactly the equal
/// floats, the equality `sanitize` welds with.
fn weld_numbers(
    names: &[u32],
    positions: Positions<'_>,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<Vec<u32>, Error> {
    let unique = names.len();
    work.add(times(unique, 2)?)?;
    work.add(sort_work(unique)?)?;
    let mut keyed = budget.reserved::<([u32; 3], u32)>(unique)?;
    for (id, &vertex) in names.iter().enumerate() {
        let p = positions
            .get(vertex as usize)
            .ok_or(Error::IndexOutOfBounds)?;
        let key = p.map(|x| if x == 0.0 { 0 } else { x.to_bits() });
        keyed.push((key, id as u32));
    }
    keyed.sort_unstable();
    let mut welded = budget.filled(unique, 0u32)?;
    let mut class: Option<([u32; 3], u32)> = None;
    for &(key, id) in &keyed {
        let first = match class {
            Some((k, first)) if k == key => first,
            _ => {
                class = Some((key, id));
                id
            }
        };
        welded[id as usize] = first;
    }
    budget.release(keyed);
    Ok(welded)
}

/// Count duplicate and reversed-duplicate faces by sorting canonical corners,
/// as `sanitize` does with `IGNORE_ORIENTATION`, keeping every face.
fn count_duplicates(
    faces: &[[u32; 3]],
    report: &mut MeshAnalysis,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<(), Error> {
    let n = faces.len();
    work.add(times(n, 2)?)?;
    work.add(sort_work(n)?)?;
    let mut keyed = budget.reserved::<([u32; 3], u32, bool)>(n)?;
    for (index, &face) in faces.iter().enumerate() {
        let key = canonical(face);
        let mirror = canonical([face[0], face[2], face[1]]);
        keyed.push(if mirror < key {
            (mirror, index as u32, true)
        } else {
            (key, index as u32, false)
        });
    }
    // Input index breaks ties, so each class starts with its earliest face.
    keyed.sort_unstable();
    let mut current = None;
    let mut winding = false;
    let mut reversed_seen = false;
    for &(class, _, mirrored) in &keyed {
        if current == Some(class) {
            if mirrored == winding || reversed_seen {
                report.duplicate += 1;
            } else {
                reversed_seen = true;
                report.reversed_duplicate += 1;
            }
        } else {
            current = Some(class);
            winding = mirrored;
            reversed_seen = false;
        }
    }
    budget.release(keyed);
    Ok(())
}

/// One directed edge of a face, keyed by its undirected vertex pair.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct HalfEdge {
    lo: u32,
    hi: u32,
    /// Corner the edge starts at: `3 * face + k`.
    corner: u32,
    /// Whether the edge runs from `lo` to `hi`.
    forward: bool,
}
impl HalfEdge {
    fn same_edge(&self, other: &Self) -> bool {
        self.lo == other.lo && self.hi == other.hi
    }
    /// The face's corners at `lo` and at `hi`.
    fn ends(self) -> (u32, u32) {
        let k = self.corner % 3;
        let next = self.corner - k + (k + 1) % 3;
        if self.forward {
            (self.corner, next)
        } else {
            (next, self.corner)
        }
    }
    /// Whether this edge has exactly two faces and both use it in the same direction.
    fn conflict(group: &[Self]) -> bool {
        group.len() == 2 && group[0].forward == group[1].forward
    }
}

/// Disjoint sets with union by size and path halving, so a tree is at most
/// `log2(len)` deep and every find visits at most `steps(len)` entries.
struct Sets {
    parent: Vec<u32>,
    size: Vec<u32>,
}
impl Sets {
    fn new(len: usize, budget: &mut Budget<'_>, work: &mut Work) -> Result<Self, Error> {
        work.add(len)?;
        let mut parent = budget.reserved::<u32>(len)?;
        parent.extend(0..len as u32);
        let size = budget.filled(len, 1u32)?;
        Ok(Self { parent, size })
    }
    fn find(&mut self, mut x: u32) -> u32 {
        loop {
            let parent = self.parent[x as usize];
            if parent == x {
                return x;
            }
            let grandparent = self.parent[parent as usize];
            self.parent[x as usize] = grandparent;
            x = grandparent;
        }
    }
    fn union(&mut self, a: u32, b: u32) {
        let (a, b) = (self.find(a), self.find(b));
        if a == b {
            return;
        }
        let (root, child) = if self.size[a as usize] >= self.size[b as usize] {
            (a, b)
        } else {
            (b, a)
        };
        self.parent[child as usize] = root;
        self.size[root as usize] += self.size[child as usize];
    }
    fn release(self, budget: &mut Budget<'_>) {
        budget.release(self.parent);
        budget.release(self.size);
    }
}

fn times(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_mul(b).ok_or(Error::SizeOverflow)
}

/// Edge, fan, component and boundary-component counts, then the bounded lists.
fn topology(
    faces: &[[u32; 3]],
    names: &[u32],
    limit: usize,
    report: &mut MeshAnalysis,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<AnalysisLists, Error> {
    let corners = times(faces.len(), 3)?;
    let unique = names.len();
    let corner_find = steps(corners);
    let vertex_find = steps(unique);
    let vertex = |corner: u32| faces[corner as usize / 3][corner as usize % 3];

    work.add(corners)?;
    work.add(sort_work(corners)?)?;
    let mut edges = budget.reserved::<HalfEdge>(corners)?;
    for (f, face) in faces.iter().enumerate() {
        for k in 0..3 {
            let (from, to) = (face[k], face[(k + 1) % 3]);
            edges.push(HalfEdge {
                lo: from.min(to),
                hi: from.max(to),
                corner: (f * 3 + k) as u32,
                forward: from < to,
            });
        }
    }
    edges.sort_unstable();

    // Corners around a vertex join one fan when their faces share an edge at
    // that vertex; each fan is then one root corner.
    let mut fans = Sets::new(corners, budget, work)?;
    for group in edges.chunk_by(HalfEdge::same_edge) {
        let pairs = group.len() - 1;
        work.add(group.len())?;
        work.add(times(pairs, times(corner_find, 4)?)?)?;
        report.edges += 1;
        if group.len() == 1 {
            report.boundary_edges += 1;
        }
        if group.len() > 2 {
            report.non_manifold_edges += 1;
        }
        if HalfEdge::conflict(group) {
            report.orientation_conflicts += 1;
        }
        for pair in group.windows(2) {
            let ((a_lo, a_hi), (b_lo, b_hi)) = (pair[0].ends(), pair[1].ends());
            fans.union(a_lo, b_lo);
            fans.union(a_hi, b_hi);
        }
    }
    work.add(corners)?;
    work.add(unique)?;
    let mut fan_counts = budget.filled(unique, 0u32)?;
    for (corner, &parent) in fans.parent.iter().enumerate() {
        if parent == corner as u32 {
            fan_counts[vertex(parent) as usize] += 1;
        }
    }
    fans.release(budget);
    report.vertices = fan_counts.iter().filter(|&&n| n > 0).count();
    report.non_manifold_vertices = fan_counts.iter().filter(|&&n| n > 1).count();

    // Components: vertices joined by faces, sized in faces.
    let mut parts = Sets::new(unique, budget, work)?;
    work.add(times(faces.len(), times(vertex_find, 5)?)?)?;
    work.add(times(unique, 2)?)?;
    for face in faces {
        parts.union(face[0], face[1]);
        parts.union(face[0], face[2]);
    }
    parts.size.fill(0);
    for face in faces {
        let root = parts.find(face[0]);
        parts.size[root as usize] += 1;
    }
    let mut smallest = usize::MAX;
    for &n in parts.size.iter().filter(|&&n| n > 0) {
        report.components += 1;
        report.largest_component = report.largest_component.max(n as usize);
        smallest = smallest.min(n as usize);
    }
    report.smallest_component = if report.components > 0 { smallest } else { 0 };
    parts.release(budget);

    // Boundary components: vertices joined by boundary edges, sized in edges.
    let mut loops = None;
    if report.boundary_edges > 0 {
        let mut sets = Sets::new(unique, budget, work)?;
        work.add(times(corners, 2)?)?;
        work.add(times(report.boundary_edges, times(vertex_find, 3)?)?)?;
        work.add(times(unique, 2)?)?;
        let boundary = || {
            edges
                .chunk_by(HalfEdge::same_edge)
                .filter_map(|group| match group {
                    [edge] => Some(*edge),
                    _ => None,
                })
        };
        for edge in boundary() {
            sets.union(edge.lo, edge.hi);
        }
        sets.size.fill(0);
        for edge in boundary() {
            let root = sets.find(edge.lo);
            sets.size[root as usize] += 1;
        }
        report.boundary_components = sets.size.iter().filter(|&&n| n > 0).count();
        loops = Some(sets);
    }

    let mut lists = AnalysisLists::default();
    if limit > 0 {
        lists = list(
            &edges,
            names,
            &fan_counts,
            loops.as_mut(),
            limit,
            report,
            budget,
            work,
        )?;
    }
    if let Some(sets) = loops {
        sets.release(budget);
    }
    budget.release(fan_counts);
    budget.release(edges);
    Ok(lists)
}

/// The first `limit` defects of each kind, in ascending vertex order.
#[allow(clippy::too_many_arguments)]
fn list(
    edges: &[HalfEdge],
    names: &[u32],
    fan_counts: &[u32],
    loops: Option<&mut Sets>,
    limit: usize,
    report: &MeshAnalysis,
    budget: &mut Budget<'_>,
    work: &mut Work,
) -> Result<AnalysisLists, Error> {
    let name = |v: u32| names[v as usize];
    let caps = [
        report.boundary_edges,
        report.non_manifold_edges,
        report.orientation_conflicts,
        report.non_manifold_vertices,
        report.boundary_components,
    ]
    .map(|n| n.min(limit));
    let mut lists = AnalysisLists {
        boundary_edges: budget.reserved(caps[0])?,
        non_manifold_edges: budget.reserved(caps[1])?,
        orientation_conflicts: budget.reserved(caps[2])?,
        non_manifold_vertices: budget.reserved(caps[3])?,
        boundary_components: budget.reserved(caps[4])?,
    };
    if caps[..3].iter().any(|&n| n > 0) {
        work.add(edges.len())?;
        for group in edges.chunk_by(HalfEdge::same_edge) {
            let Some(&edge) = group.first() else {
                continue;
            };
            let pair = [name(edge.lo), name(edge.hi)];
            if group.len() == 1 && lists.boundary_edges.len() < caps[0] {
                let [lo, hi] = pair;
                lists
                    .boundary_edges
                    .push(if edge.forward { [lo, hi] } else { [hi, lo] });
            }
            if group.len() > 2 && lists.non_manifold_edges.len() < caps[1] {
                lists.non_manifold_edges.push(pair);
            }
            if lists.orientation_conflicts.len() < caps[2] && HalfEdge::conflict(group) {
                lists.orientation_conflicts.push(pair);
            }
        }
    }
    if caps[3] > 0 {
        work.add(fan_counts.len())?;
        for (v, &n) in fan_counts.iter().enumerate() {
            if n > 1 && lists.non_manifold_vertices.len() < caps[3] {
                lists.non_manifold_vertices.push(name(v as u32));
            }
        }
    }
    if let (Some(sets), true) = (loops, caps[4] > 0) {
        // The first vertex met of each component is its smallest; zeroing the
        // component's count skips its other vertices.
        work.add(times(names.len(), steps(names.len()))?)?;
        for v in 0..names.len() as u32 {
            if lists.boundary_components.len() == caps[4] {
                break;
            }
            let root = sets.find(v);
            let edges = sets.size[root as usize];
            if edges > 0 {
                lists.boundary_components.push(BoundaryComponent {
                    vertex: name(v),
                    edges: edges as usize,
                });
                sets.size[root as usize] = 0;
            }
        }
    }
    Ok(lists)
}

/// The source-compatible u32 entry point; see [`analyze_mesh_generic`] for selectable index widths.
#[inline]
pub fn analyze_mesh(
    indices: &[u32],
    positions: Positions<'_>,
    options: AnalyzeOptions,
    workspace: &mut Workspace,
) -> Result<MeshAnalysis, Error> {
    analyze_mesh_generic::<u32>(indices, positions, options, workspace)
}
