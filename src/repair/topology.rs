//! Fan splitting and conservative boundary stitching, not in C++ meshoptimizer.
use super::*;
use crate::{Index, VertexStream};

/// Fan split counts; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SplitVerticesReport {
    /// Original vertices with more than one connected face fan.
    pub split_vertices: usize,
    /// Additional records copied from the original vertices.
    pub duplicated_vertices: usize,
}
/// Stitch decisions; not in C++ meshoptimizer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct StitchReport {
    /// Accepted boundary vertex merges.
    pub stitched_pairs: usize,
    /// Accepted boundary edge subdivisions at existing vertices.
    pub t_junction_splits: usize,
    /// Geometrically eligible candidates refused for topology or winding.
    pub refused: usize,
    /// Sum of Euclidean lengths of remaining one-face edges.
    pub remaining_boundary_length: f64,
}
/// Repaired indices and packed attribute records; not in C++ meshoptimizer.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct TopologyRepairGeneric<R, I: Index = u32> {
    /// Output triangles in original order, with subdivisions appended.
    pub indices: Vec<I>,
    /// Packed records of the input stream's key size, including copied attributes.
    pub vertices: Vec<u8>,
    /// Operation-specific decisions.
    pub report: R,
}

fn root(parent: &mut [usize], mut x: usize, work: &mut Work) -> Result<usize, Error> {
    while parent[x] != x {
        work.add(1)?;
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    Ok(x)
}

/// Split disconnected face fans at each vertex, copying complete attribute records.
/// This extension is not in C++ meshoptimizer. The first fan (lowest input
/// corner) keeps its vertex; subsequent fans receive appended copies. Unused
/// vertices remain. A fan is a set of corners joined across shared edges, the
/// definition `analyze_mesh` uses, so afterwards `non_manifold_vertices` is zero.
/// Edges shared by more than two faces stay shared and are not repaired.
/// Work and storage are bounded by the workspace, with no partial output.
/// # Errors
/// Topology, index, allocation and resource errors as in [`super::orient_faces`].
pub fn split_non_manifold_vertices(
    indices: &[u32],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<TopologyRepairGeneric<SplitVerticesReport>, Error> {
    split_non_manifold_vertices_generic(indices, vertices, workspace)
}
/// Selectable index form of [`split_non_manifold_vertices`]; not in C++ meshoptimizer.
/// # Errors
/// Same errors, plus `IndexOverflow` when appended vertices exceed the index width.
pub fn split_non_manifold_vertices_generic<I: Index>(
    indices: &[I],
    vertices: VertexStream<'_>,
    workspace: &mut Workspace,
) -> Result<TopologyRepairGeneric<SplitVerticesReport, I>, Error> {
    I::with_input(indices, workspace, |indices, ws| {
        let mut work = ws.begin();
        let result = (|| {
            let mut b = Budget::new(ws);
            let es = edges(indices, vertices.len(), &mut b, &mut work)?;
            let mut parent = b.filled(indices.len(), 0usize)?;
            for (i, p) in parent.iter_mut().enumerate() {
                *p = i;
            }
            // Fans are the corner classes joined across every shared edge, the
            // same definition `analyze_mesh` uses, so each output vertex has
            // exactly one fan. Edges shared by more than two faces stay shared.
            for g in groups(&es) {
                work.add(g.len())?;
                for pair in g.windows(2) {
                    let (a, c) = (&pair[0], &pair[1]);
                    for v in a.key {
                        let corner = |e: &Edge| {
                            let base = e.face as usize * 3;
                            base + if indices[base + e.corner] == v {
                                e.corner
                            } else {
                                (e.corner + 1) % 3
                            }
                        };
                        let x = root(&mut parent, corner(a), &mut work)?;
                        let y = root(&mut parent, corner(c), &mut work)?;
                        parent[x.max(y)] = x.min(y);
                    }
                }
            }
            let mut assigned = b.filled(indices.len(), NONE)?;
            let mut seen = b.filled(vertices.len(), 0usize)?;
            let cap = vertices
                .len()
                .checked_add(indices.len())
                .ok_or(Error::SizeOverflow)?;
            let mut sources = b.reserved::<u32>(cap)?;
            for v in 0..vertices.len() {
                sources.push(u32::try_from(v).map_err(|_| Error::SizeOverflow)?);
            }
            let mut out = b.reserved::<I>(indices.len())?;
            let mut report = SplitVerticesReport::default();
            for (c, &v) in indices.iter().enumerate() {
                work.add(1)?;
                let r = root(&mut parent, c, &mut work)?;
                if assigned[r] == NONE {
                    assigned[r] = if seen[v as usize] == 0 {
                        v
                    } else {
                        if seen[v as usize] == 1 {
                            report.split_vertices += 1;
                        }
                        u32::try_from(sources.len().checked_add(1).ok_or(Error::SizeOverflow)?)
                            .map_err(|_| Error::SizeOverflow)?;
                        let id = u32::try_from(sources.len()).map_err(|_| Error::SizeOverflow)?;
                        sources.push(v);
                        report.duplicated_vertices += 1;
                        id
                    };
                    seen[v as usize] += 1;
                }
                out.push(I::from_u32(assigned[r])?);
            }
            let mut data = b.reserved(checked_bytes(sources.len(), vertices.size)?)?;
            for v in sources {
                work.add(vertices.size)?;
                data.extend_from_slice(vertices.key(v));
            }
            Ok(TopologyRepairGeneric {
                indices: out,
                vertices: data,
                report,
            })
        })();
        ws.finish(&work);
        result
    })
}

fn point(p: Positions<'_>, v: u32) -> Result<[f64; 3], Error> {
    Ok(p.get(v as usize)
        .ok_or(Error::IndexOutOfBounds)?
        .map(f64::from))
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [0, 1, 2].map(|i| a[i] - b[i])
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn normal(t: &[u32], p: Positions<'_>) -> Result<[f64; 3], Error> {
    let a = sub(point(p, t[1])?, point(p, t[0])?);
    let b = sub(point(p, t[2])?, point(p, t[0])?);
    Ok([
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ])
}
/// Defect counts used to refuse candidates: edges with more than two faces,
/// two-face edges whose faces are not oppositely directed, and surplus fans
/// (fans minus distinct vertices, as `analyze_mesh` defines them).
type Defects = [usize; 3];
fn defects(
    candidate: &[u32],
    vertex_count: usize,
    b: &mut Budget<'_>,
    w: &mut Work,
) -> Result<Defects, Error> {
    let es = edges(candidate, vertex_count, b, w)?;
    let mut found = [0usize; 3];
    let mut parent = b.filled(candidate.len(), 0usize)?;
    for (i, p) in parent.iter_mut().enumerate() {
        w.add(1)?;
        *p = i;
    }
    for g in groups(&es) {
        w.add(g.len())?;
        if g.len() > 2 {
            found[0] += 1;
        } else if g.len() == 2 && g[0].forward == g[1].forward {
            found[1] += 1;
        }
        for pair in g.windows(2) {
            let (a, c) = (&pair[0], &pair[1]);
            for v in a.key {
                let corner = |e: &Edge| {
                    let base = e.face as usize * 3;
                    base + if candidate[base + e.corner] == v {
                        e.corner
                    } else {
                        (e.corner + 1) % 3
                    }
                };
                let x = root(&mut parent, corner(a), w)?;
                let y = root(&mut parent, corner(c), w)?;
                parent[x.max(y)] = x.min(y);
            }
        }
    }
    let mut used = b.filled(vertex_count, false)?;
    let mut fans = 0usize;
    for (c, &v) in candidate.iter().enumerate() {
        w.add(1)?;
        used[v as usize] = true;
        if root(&mut parent, c, w)? == c {
            fans += 1;
        }
    }
    found[2] = fans
        .checked_sub(used.iter().filter(|&&u| u).count())
        .ok_or(Error::SizeOverflow)?;
    b.release(parent);
    b.release(used);
    b.release(es);
    Ok(found)
}
/// Accept a candidate only when every changed face keeps a positive normal
/// dot product with its predecessor and no defect count exceeds `base`.
/// Defects already present in the input therefore never block unrelated
/// stitches. Returns the candidate's counts when accepted.
fn valid(
    candidate: &[u32],
    old: &[u32],
    p: Positions<'_>,
    base: Defects,
    b: &mut Budget<'_>,
    w: &mut Work,
) -> Result<Option<Defects>, Error> {
    for (t, before) in candidate
        .as_chunks::<3>()
        .0
        .iter()
        .zip(old.as_chunks::<3>().0)
    {
        w.add(1)?;
        if t != before
            && (t[0] == t[1]
                || t[1] == t[2]
                || t[2] == t[0]
                || dot(normal(t, p)?, normal(before, p)?) <= 0.)
        {
            return Ok(None);
        }
    }
    let found = defects(candidate, p.len(), b, w)?;
    Ok(found
        .iter()
        .zip(base)
        .all(|(found, limit)| *found <= limit)
        .then_some(found))
}
fn boundaries(
    i: &[u32],
    p: Positions<'_>,
    b: &mut Budget<'_>,
    w: &mut Work,
) -> Result<Vec<Edge>, Error> {
    let es = edges(i, p.len(), b, w)?;
    let mut out = b.reserved(es.len())?;
    for g in groups(&es) {
        w.add(g.len())?;
        if g.len() == 1 {
            out.push(g[0]);
        }
    }
    b.release(es);
    Ok(out)
}

/// Stitch boundary vertices and subdivide boundary edges at T-junctions.
/// Not in C++ meshoptimizer. `None` selects 1e-6 times the largest bounds extent;
/// `Some` is an absolute finite nonnegative Euclidean tolerance. Only boundary
/// vertices participate. Lowest original vertex wins each merge; attributes of
/// that representative win, so this is an explicit position-only seam policy.
/// Records stay packed and uncompact, and T-junctions reuse existing records.
/// Every candidate must keep a positive normal dot product on each changed face
/// and must not increase the mesh's counts of edges with more than two faces,
/// equally directed two-face edges, or surplus vertex fans. Other candidates are
/// counted as refusals. Defects present in the input do not block other stitches.
/// A deterministic sorted spatial grid finds vertex pairs; edge/vertex tests
/// and whole-mesh validation are metered (worst-case quadratic or worse).
/// # Errors
/// Invalid tolerance/layout, nonfinite positions, invalid topology/references,
/// and workspace allocation, arithmetic or work/byte limit errors. No partial output.
pub fn stitch_cracks(
    indices: &[u32],
    vertices: VertexStream<'_>,
    positions: Positions<'_>,
    tolerance: Option<f32>,
    workspace: &mut Workspace,
) -> Result<TopologyRepairGeneric<StitchReport>, Error> {
    stitch_cracks_generic(indices, vertices, positions, tolerance, workspace)
}
/// Selectable index form of [`stitch_cracks`]; not in C++ meshoptimizer.
/// # Errors
/// Same as [`stitch_cracks`], plus checked index conversion errors.
pub fn stitch_cracks_generic<I: Index>(
    indices: &[I],
    vertices: VertexStream<'_>,
    positions: Positions<'_>,
    tolerance: Option<f32>,
    workspace: &mut Workspace,
) -> Result<TopologyRepairGeneric<StitchReport, I>, Error> {
    I::with_input(indices, workspace, |indices, ws| {
        let mut work = ws.begin();
        let result = (|| {
            if vertices.len() != positions.len() {
                return Err(Error::InvalidLayout);
            }
            let mut lo = [f64::INFINITY; 3];
            let mut hi = [f64::NEG_INFINITY; 3];
            let mut largest = 0f64;
            for v in 0..positions.len() {
                work.add(1)?;
                let p = positions.get(v).ok_or(Error::IndexOutOfBounds)?;
                for a in 0..3 {
                    if !p[a].is_finite() {
                        return Err(Error::NumericalFailure);
                    }
                    lo[a] = lo[a].min(f64::from(p[a]));
                    hi[a] = hi[a].max(f64::from(p[a]));
                    largest = largest.max(f64::from(p[a]).abs());
                }
            }
            let tol = match tolerance {
                Some(t) if t.is_finite() && t >= 0. => f64::from(t),
                Some(_) => return Err(Error::InvalidParameter),
                None => {
                    if positions.is_empty() {
                        0.
                    } else {
                        (0..3).map(|a| hi[a] - lo[a]).fold(0., f64::max) * 1e-6
                    }
                }
            };
            let width = if tol == 0. {
                0.
            } else {
                (2. * tol).max(largest / ((1u64 << 50) as f64))
            };
            let mut b = Budget::new(ws);
            let mut out = b.copied(indices)?;
            let mut limit = defects(&out, positions.len(), &mut b, &mut work)?;
            let mut boundary = boundaries(&out, positions, &mut b, &mut work)?;
            let mut active = b.filled(positions.len(), false)?;
            for e in &boundary {
                work.add(1)?;
                for v in e.key {
                    active[v as usize] = true;
                }
            }
            let mut cells = b.reserved::<Cell>(positions.len())?;
            for (v, &on) in active.iter().enumerate() {
                work.add(1)?;
                if on {
                    cells.push(Cell {
                        key: cell(positions.get(v).ok_or(Error::IndexOutOfBounds)?, width),
                        vertex: u32::try_from(v).map_err(|_| Error::SizeOverflow)?,
                    });
                }
            }
            work.add(sort_work(cells.len())?)?;
            cells.sort_unstable_by_key(|c| (c.key, c.vertex));
            let mut report = StitchReport::default();
            let mut trial = b.copied(&out)?;
            let radius = if tol == 0. { 0 } else { 1 };
            for v in 0..positions.len() {
                work.add(1)?;
                if !active[v] {
                    continue;
                }
                let id = u32::try_from(v).map_err(|_| Error::SizeOverflow)?;
                let p = point(positions, id)?;
                let key = cell(positions.get(v).ok_or(Error::IndexOutOfBounds)?, width);
                for x in -radius..=radius {
                    for y in -radius..=radius {
                        for z in -radius..=radius {
                            let neighbor = [key[0] + x, key[1] + y, key[2] + z];
                            work.add(steps(cells.len()))?;
                            let start = cells.partition_point(|c| c.key < neighbor);
                            for c in &cells[start..] {
                                work.add(1)?;
                                if c.key != neighbor {
                                    break;
                                }
                                if c.vertex >= id || !active[c.vertex as usize] || !active[v] {
                                    continue;
                                }
                                let d = sub(p, point(positions, c.vertex)?);
                                if dot(d, d) > tol * tol {
                                    continue;
                                }
                                work.add(out.len())?;
                                trial.copy_from_slice(&out);
                                for a in &mut trial {
                                    if *a == id {
                                        *a = c.vertex;
                                    }
                                }
                                if let Some(found) =
                                    valid(&trial, &out, positions, limit, &mut b, &mut work)?
                                {
                                    limit = found;
                                    out.copy_from_slice(&trial);
                                    active[v] = false;
                                    report.stitched_pairs += 1;
                                } else {
                                    // A whole matching edge must be joined atomically:
                                    // merging only its first endpoint creates a bow tie.
                                    let mut joined = false;
                                    for e in &boundary {
                                        work.add(1)?;
                                        if !e.key.contains(&id) {
                                            continue;
                                        }
                                        let other =
                                            if e.key[0] == id { e.key[1] } else { e.key[0] };
                                        if !active[other as usize] {
                                            continue;
                                        }
                                        for f in &boundary {
                                            work.add(1)?;
                                            if e.key == f.key || !f.key.contains(&c.vertex) {
                                                continue;
                                            }
                                            let partner = if f.key[0] == c.vertex {
                                                f.key[1]
                                            } else {
                                                f.key[0]
                                            };
                                            if other == partner
                                                || other == c.vertex
                                                || partner == id
                                                || !active[partner as usize]
                                            {
                                                continue;
                                            }
                                            let d = sub(
                                                point(positions, other)?,
                                                point(positions, partner)?,
                                            );
                                            if dot(d, d) > tol * tol {
                                                continue;
                                            }
                                            let loser = other.max(partner);
                                            let winner = other.min(partner);
                                            work.add(out.len())?;
                                            trial.copy_from_slice(&out);
                                            for a in &mut trial {
                                                if *a == id {
                                                    *a = c.vertex;
                                                } else if *a == loser {
                                                    *a = winner;
                                                }
                                            }
                                            if let Some(found) = valid(
                                                &trial, &out, positions, limit, &mut b, &mut work,
                                            )? {
                                                limit = found;
                                                work.add(out.len())?;
                                                out.copy_from_slice(&trial);
                                                active[v] = false;
                                                active[loser as usize] = false;
                                                report.stitched_pairs += 2;
                                                joined = true;
                                                break;
                                            }
                                        }
                                        if joined {
                                            break;
                                        }
                                    }
                                    if !joined {
                                        report.refused += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            b.release(boundary);
            // Rebuild after each accepted subdivision; at most one split per
            // original boundary vertex, bounding output by input + 3*vertices.
            for (v, &on) in active.iter().enumerate() {
                work.add(1)?;
                if !on {
                    continue;
                }
                let id = u32::try_from(v).map_err(|_| Error::SizeOverflow)?;
                boundary = boundaries(&out, positions, &mut b, &mut work)?;
                for e in &boundary {
                    work.add(1)?;
                    if e.key.contains(&id) {
                        continue;
                    }
                    let a = point(positions, e.key[0])?;
                    let ab = sub(point(positions, e.key[1])?, a);
                    let av = sub(point(positions, id)?, a);
                    let len = dot(ab, ab);
                    if len == 0. {
                        continue;
                    }
                    let t = dot(av, ab) / len;
                    if t <= 0. || t >= 1. {
                        continue;
                    }
                    let d = [0, 1, 2].map(|k| av[k] - t * ab[k]);
                    if dot(d, d) > tol * tol {
                        continue;
                    }
                    let n = out.len().checked_add(3).ok_or(Error::SizeOverflow)?;
                    b.reuse(&mut trial, n, 0)?;
                    work.add(n)?;
                    trial[..out.len()].copy_from_slice(&out);
                    let base = e.face as usize * 3;
                    let c = e.corner;
                    let z = out[base + (c + 2) % 3];
                    let end = out[base + (c + 1) % 3];
                    trial[base + (c + 1) % 3] = id;
                    trial[out.len()..].copy_from_slice(&[id, end, z]);
                    let normals_ok = dot(
                        normal(&[id, end, z], positions)?,
                        normal(&out[base..base + 3], positions)?,
                    ) > 0.;
                    let accepted = if normals_ok {
                        valid(&trial, &out, positions, limit, &mut b, &mut work)?
                    } else {
                        None
                    };
                    if let Some(found) = accepted {
                        limit = found;
                        b.reuse(&mut out, n, 0)?;
                        out.copy_from_slice(&trial);
                        report.t_junction_splits += 1;
                        break;
                    } else {
                        report.refused += 1;
                    }
                }
                b.release(boundary);
            }
            boundary = boundaries(&out, positions, &mut b, &mut work)?;
            for e in boundary {
                work.add(1)?;
                let d = sub(point(positions, e.key[0])?, point(positions, e.key[1])?);
                report.remaining_boundary_length += libm::sqrt(dot(d, d));
            }
            let mut converted = b.reserved::<I>(out.len())?;
            for v in out {
                work.add(1)?;
                converted.push(I::from_u32(v)?);
            }
            let mut data = b.reserved(checked_bytes(vertices.len(), vertices.size)?)?;
            for v in 0..vertices.len() {
                work.add(vertices.size)?;
                data.extend_from_slice(
                    vertices.key(u32::try_from(v).map_err(|_| Error::SizeOverflow)?),
                );
            }
            Ok(TopologyRepairGeneric {
                indices: converted,
                vertices: data,
                report,
            })
        })();
        ws.finish(&work);
        result
    })
}
