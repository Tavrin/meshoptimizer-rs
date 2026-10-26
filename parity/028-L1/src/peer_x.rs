//! Published optimesh public API adapters; allocation/copies/destruction stay timed.
use super::{Input, Output};
use optimesh::{
    clusterizer as c, indexcodec as i, simplifier as s, vertexcodec as v, vertexfilter as f,
};
pub(super) fn run(op: &str, x: &Input, _: &mut ()) -> Output {
    let n = x.p.len();
    let positions: &[f32] = x
        .replay
        .as_ref()
        .map_or(x.p.as_flattened(), |r| r.data.as_slice());
    let stride = x.replay.as_ref().map_or(12, |r| r.stride);
    match op {
        "vertex-decode" => {
            let mut b = vec![0; n * 12];
            v::decode_vertex_buffer(&mut b, n, 12, &x.vb).unwrap();
            Output::Bytes(b)
        }
        "vertex-encode" | "vertex-encode-v0" | "vertex-encode-default" => {
            let mut b = vec![0; v::encode_vertex_buffer_bound(n, 12)];
            let k = if op == "vertex-encode-default" {
                v::encode_vertex_buffer(&mut b, &x.raw, n, 12)
            } else {
                v::encode_vertex_buffer_level(
                    &mut b,
                    &x.raw,
                    n,
                    12,
                    2,
                    if op.ends_with("v0") { 0 } else { 1 },
                )
            };
            assert!(k > 0);
            b.truncate(k);
            Output::Bytes(b)
        }
        "index-decode" | "sequence-decode" => {
            let mut dst = vec![0; x.i.len()];
            if op == "index-decode" {
                i::decode_index_buffer(&mut dst, &x.ib).unwrap()
            } else {
                i::decode_index_sequence(&mut dst, &x.seq).unwrap()
            };
            Output::Indices(dst)
        }
        "index-encode" | "sequence-encode" => {
            let mut b = vec![
                0;
                if op == "index-encode" {
                    i::encode_index_buffer_bound(x.i.len(), n)
                } else {
                    i::encode_index_sequence_bound(x.i.len(), n)
                }
            ];
            let k = if op == "index-encode" {
                i::encode_index_buffer(&mut b, &x.i)
            } else {
                i::encode_index_sequence(&mut b, &x.i)
            };
            assert!(k > 0);
            b.truncate(k);
            Output::Bytes(b)
        }
        "cache" => {
            let mut dst = vec![0; x.i.len()];
            optimesh::vcacheoptimizer::optimize_vertex_cache(&mut dst, &x.i, n);
            Output::Indices(dst)
        }
        "cache-strip" => {
            let mut dst = vec![0; x.i.len()];
            optimesh::vcacheoptimizer::optimize_vertex_cache_strip(&mut dst, &x.i, n);
            Output::Indices(dst)
        }
        "cache-fifo" => {
            let mut dst = vec![0; x.i.len()];
            optimesh::vcacheoptimizer::optimize_vertex_cache_fifo(&mut dst, &x.i, n, 16);
            Output::Indices(dst)
        }
        "overdraw" => {
            let mut dst = vec![0; x.i.len()];
            optimesh::overdrawoptimizer::optimize_overdraw(
                &mut dst,
                &x.i,
                positions,
                n,
                stride,
                x.replay.as_ref().map_or(1.05, |r| r.threshold),
            );
            Output::Indices(dst)
        }
        "fetch-remap" => {
            let mut b = vec![0; n];
            optimesh::vfetchoptimizer::optimize_vertex_fetch_remap(&mut b, &x.i);
            Output::Indices(b)
        }
        "fetch" => {
            let mut ix = x.i.clone();
            let mut b = vec![0; x.raw.len()];
            let k = optimesh::vfetchoptimizer::optimize_vertex_fetch(&mut b, &mut ix, &x.raw, 12);
            b.truncate(k * 12);
            Output::Fetch(ix, b)
        }
        "simplify" | "simplify-attributes" | "simplify-sloppy" => {
            let mut dst = vec![0; x.i.len()];
            let p = s::VertexData {
                positions,
                count: n,
                stride,
            };
            let target = s::SimplifyTarget {
                target_index_count: x
                    .replay
                    .as_ref()
                    .map_or(x.i.len() / 2 / 3 * 3, |r| r.target),
                target_error: x.replay.as_ref().map_or(0.5, |r| r.error),
                options: x.replay.as_ref().map_or(0, |r| r.options),
            };
            let (k, e) = match op {
                "simplify" => s::simplify(&mut dst, &x.i, &p, &target),
                "simplify-sloppy" => {
                    s::simplify_sloppy(&mut dst, &x.i, &p, None, target.target_index_count, 0.5)
                }
                _ => s::simplify_with_attributes(
                    &mut dst,
                    &x.i,
                    &p,
                    &s::Attributes {
                        data: &x.floats,
                        stride: 16,
                        weights: &[1.; 4],
                        count: 4,
                    },
                    None,
                    &target,
                ),
            };
            dst.truncate(k);
            Output::Simplify(dst, e)
        }
        "meshlets" | "meshlets-flex" | "meshlets-scan" | "meshlets-spatial" => {
            let bound = c::build_meshlets_bound(
                x.i.len(),
                64,
                if op == "meshlets-flex" || op == "meshlets-spatial" {
                    64
                } else {
                    124
                },
            );
            let mut ms = vec![c::Meshlet::default(); bound];
            let mut verts = vec![0; x.i.len().max(bound * 64)];
            let mut tris = vec![0; x.i.len().max(bound * 124 * 3)];
            let mut out = c::MeshletBuffers {
                meshlets: &mut ms,
                vertices: &mut verts,
                triangles: &mut tris,
            };
            let p = c::Positions {
                data: positions,
                count: n,
                stride: 12,
            };
            let k = match op {
                "meshlets" => c::build_meshlets(&mut out, &x.i, &p, 64, 124, 0.),
                "meshlets-scan" => c::build_meshlets_scan(&mut out, &x.i, n, 64, 124),
                "meshlets-flex" => c::build_meshlets_flex(
                    &mut out,
                    &x.i,
                    &p,
                    &c::MeshletOptions {
                        max_vertices: 64,
                        min_triangles: 64,
                        max_triangles: 124,
                        cone_weight: 0.,
                        split_factor: 0.,
                    },
                ),
                _ => c::build_meshlets_spatial(&mut out, &x.i, &p, 64, 64, 124, 0.5),
            };
            ms.truncate(k);
            Output::MeshX(ms, verts, tris)
        }
        "oct-decode" | "quat-decode" | "oct-encode" | "quat-encode" => {
            let oct = op.starts_with("oct");
            let mut b = if op.ends_with("decode") {
                x.filters[if oct { 0 } else { 1 }]
                    .chunks_exact(2)
                    .map(|b| i16::from_le_bytes(b.try_into().unwrap()))
                    .collect::<Vec<_>>()
            } else {
                vec![0; n * 4]
            };
            match op {
                "oct-decode" => f::decode_filter_oct16(&mut b),
                "quat-decode" => f::decode_filter_quat(&mut b),
                "oct-encode" => f::encode_filter_oct16(&mut b, 12, &x.floats),
                _ => f::encode_filter_quat(&mut b, 12, &x.quat),
            };
            Output::Bytes(b.iter().flat_map(|v| v.to_le_bytes()).collect())
        }
        "exp-decode" | "exp-encode" => {
            let mut b = if op.ends_with("decode") {
                x.filters[2]
                    .chunks_exact(4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .collect::<Vec<_>>()
            } else {
                vec![0; n * 4]
            };
            if op.ends_with("decode") {
                f::decode_filter_exp(&mut b)
            } else {
                f::encode_filter_exp(&mut b, 16, 20, &x.floats, f::ExpMode::Separate)
            };
            Output::Bytes(b.iter().flat_map(|v| v.to_le_bytes()).collect())
        }
        "color-decode" | "color-encode" => {
            let mut b = if op.ends_with("decode") {
                x.filters[3]
                    .chunks_exact(2)
                    .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                    .collect::<Vec<_>>()
            } else {
                vec![0; n * 4]
            };
            if op.ends_with("decode") {
                f::decode_filter_color16(&mut b)
            } else {
                let a: Vec<_> = x.floats.iter().map(|v| v.abs()).collect();
                f::encode_filter_color16(&mut b, 12, &a)
            };
            Output::Bytes(b.iter().flat_map(|v| v.to_le_bytes()).collect())
        }

        "stripify" => {
            let mut b = vec![0; optimesh::stripifier::stripify_bound(x.i.len())];
            let k = optimesh::stripifier::stripify(&mut b, &x.i, n, u32::MAX);
            b.truncate(k);
            Output::Indices(b)
        }
        "unstripify" => {
            let strip = super::decode_words(&x.seq);
            let mut b = vec![0; optimesh::stripifier::unstripify_bound(strip.len())];
            let k = optimesh::stripifier::unstripify(&mut b, &strip, u32::MAX);
            b.truncate(k);
            Output::Indices(b)
        }
        "vertex-version" => Output::Bytes(vec![v::decode_vertex_version(&x.vb).unwrap() as u8]),
        "index-version" => Output::Bytes(vec![i::decode_index_version(&x.ib).unwrap() as u8]),
        "vertex-bound" => size(v::encode_vertex_buffer_bound(n, 12)),
        "index-bound" => size(i::encode_index_buffer_bound(x.i.len(), n)),
        "sequence-bound" => size(i::encode_index_sequence_bound(x.i.len(), n)),
        "strip-bound" => size(optimesh::stripifier::stripify_bound(x.i.len())),
        "unstrip-bound" => size(optimesh::stripifier::unstripify_bound(x.i.len())),
        "meshlet-bound" => size(c::build_meshlets_bound(x.i.len(), 64, 124)),
        "simplify-scale" => super::floats(&[s::simplify_scale(positions, n, 12)]),
        "simplify-prune" => {
            let mut b = vec![0; x.i.len()];
            let k = s::simplify_prune(
                &mut b,
                &x.i,
                &s::VertexData {
                    positions,
                    count: n,
                    stride: 12,
                },
                0.5,
            );
            b.truncate(k);
            Output::Indices(b)
        }
        "simplify-points" => {
            let mut b = vec![0; n / 2];
            let k = s::simplify_points(
                &mut b,
                &s::VertexData {
                    positions,
                    count: n,
                    stride: 12,
                },
                None,
                0.,
                n / 2,
            );
            b.truncate(k);
            Output::Indices(b)
        }
        "spatial-remap" | "spatial-points" => {
            let mut b = vec![0; n];
            if op == "spatial-remap" {
                optimesh::spatialorder::spatial_sort_remap(&mut b, positions, n, 12)
            } else {
                optimesh::spatialorder::spatial_cluster_points(&mut b, positions, n, 12, 16)
            };
            Output::Indices(b)
        }
        "spatial-triangles" => {
            let mut b = vec![0; x.i.len()];
            optimesh::spatialorder::spatial_sort_triangles(&mut b, &x.i, positions, n, 12);
            Output::Indices(b)
        }
        "analyze-cache" => {
            let r = optimesh::indexanalyzer::analyze_vertex_cache(&x.i, n, 16, 32, 128);
            Output::Indices(vec![
                r.vertices_transformed,
                r.warps_executed,
                r.acmr.to_bits(),
                r.atvr.to_bits(),
            ])
        }
        "analyze-fetch" => {
            let r = optimesh::indexanalyzer::analyze_vertex_fetch(&x.i, n, 12);
            Output::Indices(vec![r.bytes_fetched, r.overfetch.to_bits()])
        }
        "quantize-half" => Output::Indices(
            positions
                .iter()
                .map(|v| optimesh::quantization::quantize_half(*v) as u32)
                .collect(),
        ),
        "quantize-float" => super::floats(
            &positions
                .iter()
                .map(|v| optimesh::quantization::quantize_float(*v, 12))
                .collect::<Vec<_>>(),
        ),
        "dequantize-half" => super::floats(
            &(0..256)
                .map(|h| optimesh::quantization::dequantize_half(h * 251))
                .collect::<Vec<_>>(),
        ),
        "quantize-unorm" => Output::Indices(
            x.floats
                .iter()
                .map(|v| optimesh::quantization::quantize_unorm(*v, 12) as u32)
                .collect(),
        ),
        "quantize-snorm" => Output::Indices(
            x.floats
                .iter()
                .map(|v| optimesh::quantization::quantize_snorm(*v, 12) as u32)
                .collect(),
        ),

        "remap" | "remap-multi" | "remap-custom" | "remap-vertices" | "remap-indices" => {
            use optimesh::indexgenerator as g;
            let mut r = vec![0; n];
            let k = match op {
                "remap-multi" => g::generate_vertex_remap_multi(
                    &mut r,
                    Some(&x.i),
                    &[g::Stream {
                        data: &x.raw,
                        size: 12,
                        stride: 12,
                    }],
                ),
                "remap-custom" => g::generate_vertex_remap_custom(
                    &mut r,
                    Some(&x.i),
                    positions,
                    12,
                    Some(&|_, _| true),
                ),
                _ => g::generate_vertex_remap(&mut r, Some(&x.i), &x.raw, 12),
            };
            match op {
                "remap-vertices" => {
                    let mut b = vec![0; n * 12];
                    g::remap_vertex_buffer(&mut b, &x.raw, 12, &r);
                    b.truncate(k * 12);
                    Output::Bytes(b)
                }
                "remap-indices" => {
                    let mut b = vec![0; x.i.len()];
                    g::remap_index_buffer(&mut b, Some(&x.i), &r);
                    Output::Indices(b)
                }
                _ => Output::Indices(r),
            }
        }
        "shadow" | "shadow-multi" => {
            use optimesh::indexgenerator as g;
            let mut b = vec![0; x.i.len()];
            if op == "shadow" {
                g::generate_shadow_index_buffer(&mut b, &x.i, &x.raw, n, 12, 12)
            } else {
                g::generate_shadow_index_buffer_multi(
                    &mut b,
                    &x.i,
                    n,
                    &[g::Stream {
                        data: &x.raw,
                        size: 12,
                        stride: 12,
                    }],
                )
            };
            Output::Indices(b)
        }
        "position-remap" => {
            let mut b = vec![0; n];
            optimesh::indexgenerator::generate_position_remap(&mut b, positions, n, 12);
            Output::Indices(b)
        }
        "adjacency" | "tessellation" => {
            let mut b = vec![0; x.i.len() * if op == "adjacency" { 2 } else { 4 }];
            if op == "adjacency" {
                optimesh::indexgenerator::generate_adjacency_index_buffer(
                    &mut b, &x.i, positions, n, 12,
                )
            } else {
                optimesh::indexgenerator::generate_tessellation_index_buffer(
                    &mut b, &x.i, positions, n, 12,
                )
            };
            Output::Indices(b)
        }
        "provoking" => {
            let mut b = vec![0; x.i.len()];
            let mut r = vec![0; n + x.i.len() / 3];
            let k =
                optimesh::indexgenerator::generate_provoking_index_buffer(&mut b, &mut r, &x.i, n);
            let mut out = vec![k as u32];
            out.extend(b);
            out.extend_from_slice(&r[..k]);
            Output::Indices(out)
        }
        "cluster-bounds" | "sphere-bounds" | "meshlet-bounds" => {
            use optimesh::meshletutils as u;
            let r = match op {
                "cluster-bounds" => {
                    u::compute_cluster_bounds(&x.i[..x.i.len().min(124 * 3)], positions, n, 12)
                }
                "sphere-bounds" => u::compute_sphere_bounds(positions, n, 12, None, 0),
                _ => u::compute_meshlet_bounds(
                    &(0..n.min(64) as u32).collect::<Vec<_>>(),
                    &[0, 1, 2],
                    1,
                    positions,
                    n,
                    12,
                ),
            };
            super::bounds(
                r.center,
                r.radius,
                r.cone_apex,
                r.cone_axis,
                r.cone_cutoff,
                r.cone_axis_s8,
                r.cone_cutoff_s8,
            )
        }
        "optimize-meshlet" => {
            let mut v: Vec<_> = (0..n.min(64) as u32).collect();
            let mut t = vec![0, 1, 2];
            let nv = v.len();
            optimesh::meshletutils::optimize_meshlet(&mut v, &mut t, 1, nv);
            let mut b = (nv as u32).to_le_bytes().to_vec();
            for j in v {
                b.extend(j.to_le_bytes())
            }
            b.extend(t);
            Output::Bytes(b)
        }
        "partition" => {
            let counts: Vec<_> = x.i.chunks(96).map(|c| c.len() as u32).collect();
            let mut b = vec![0; counts.len()];
            let k = optimesh::partition::partition_clusters(
                &mut b,
                &x.i,
                &counts,
                Some(positions),
                n,
                12,
                4,
            );
            let mut out = vec![k as u32];
            out.extend(b);
            Output::Indices(out)
        }
        "simplify-update" => {
            let mut ix = x.i.clone();
            let mut pp = positions.to_vec();
            let (k, e) = s::simplify_with_update(
                &mut ix,
                &mut s::VertexDataMut {
                    positions: &mut pp,
                    count: n,
                    stride: 12,
                },
                &mut s::AttributesMut {
                    data: &mut [],
                    stride: 0,
                    weights: &[],
                    count: 0,
                },
                None,
                &s::SimplifyTarget {
                    target_index_count: x.i.len() / 2 / 3 * 3,
                    target_error: 0.5,
                    options: 0,
                },
            );
            ix.truncate(k);
            let mut b = vec![k as u32];
            b.extend(ix);
            b.push(e.to_bits());
            b.extend(pp.iter().map(|v| v.to_bits()));
            Output::Indices(b)
        }
        "analyze-overdraw" => {
            let r = optimesh::rasterizer::analyze_overdraw(&x.i, positions, n, 12);
            Output::Indices(vec![
                r.pixels_covered,
                r.pixels_shaded,
                r.overdraw.to_bits(),
            ])
        }
        "analyze-coverage" => {
            let r = optimesh::rasterizer::analyze_coverage(&x.i, positions, n, 12);
            super::floats(&[r.coverage[0], r.coverage[1], r.coverage[2], r.extent])
        }
        _ => panic!("unavailable optimesh adapter {op}"),
    }
}

fn size(n: usize) -> Output {
    Output::Bytes((n as u64).to_le_bytes().to_vec())
}
