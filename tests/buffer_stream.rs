#[path = "support/buffer_stream.rs"]
mod support;
use meshoptimizer_rs::{codec::*, Workspace};

#[test]
fn random_roundtrips_and_malformed() {
    let mut rng = 17u32;
    for version in 0..=1 {
        for count in [0, 1, 15, 16, 17, 32, 255, 256, 257, 1027] {
            for stride in [4, 12, 32, 128, 256] {
                let vertices: Vec<u8> = (0..count * stride)
                    .map(|_| {
                        rng ^= rng << 13;
                        rng ^= rng >> 17;
                        rng ^= rng << 5;
                        rng.to_le_bytes()[0]
                    })
                    .collect();
                let encoded = encode_vertex_buffer(
                    &vertices,
                    count,
                    stride,
                    VertexEncoding::new(version, 2).unwrap(),
                    &mut Workspace::default(),
                )
                .unwrap();
                if count <= 32 && stride <= 32 {
                    if let Ok(dir) = std::env::var("STREAM_SEEDS") {
                        let mut seed = vec![
                            u8::try_from(count).unwrap(),
                            u8::try_from(stride / 4 - 1).unwrap(),
                            7,
                            19,
                        ];
                        seed.extend_from_slice(&encoded);
                        std::fs::write(format!("{dir}/v-{version}-{count}-{stride}"), seed)
                            .unwrap();
                    }
                }
                for chunk in [1, 7, 4096, 65536] {
                    // Tiny chunks re-parse the pending block per push; keep them to small streams.
                    if chunk < 4096 && encoded.len() > 1500 {
                        continue;
                    }
                    assert_eq!(
                        support::vertex(count, stride, &encoded, chunk, 113).unwrap(),
                        vertices
                    );
                }
                for len in [0, 1, encoded.len() / 2, encoded.len() - 1] {
                    let source = &encoded[..len];
                    assert_eq!(
                        support::vertex(
                            count,
                            stride,
                            source,
                            if encoded.len() > 1500 { 509 } else { 7 },
                            31
                        ),
                        decode_vertex_buffer(count, stride, source, &mut Workspace::default())
                    );
                }
            }
            let indices: Vec<u32> = (0..count * 3)
                .map(|i| u32::try_from(i % 65536).unwrap())
                .collect();
            let encoded = encode_index_buffer(
                &indices,
                IndexEncoding::new(version).unwrap(),
                &mut Workspace::default(),
            )
            .unwrap();
            if count <= 32 {
                if let Ok(dir) = std::env::var("STREAM_SEEDS") {
                    let mut seed = vec![u8::try_from(count).unwrap(), 0, 7, 19];
                    seed.extend_from_slice(&encoded);
                    std::fs::write(format!("{dir}/t-{version}-{count}"), seed).unwrap();
                }
            }
            for chunk in [1, 7, 4096, 65536] {
                assert_eq!(
                    support::triangles(indices.len(), &encoded, chunk, 7).unwrap(),
                    decode_index_buffer(indices.len(), 4, &encoded, &mut Workspace::default())
                        .unwrap()
                );
            }
        }
    }
}
#[test]
fn arbitrary_errors() {
    let mut rng = 31u32;
    for _ in 0..2000 {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let count = usize::try_from(rng % 33).unwrap();
        let len = usize::try_from(rng % 256).unwrap();
        let mut source: Vec<u8> = (0..len)
            .map(|_| {
                rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                rng.to_le_bytes()[2]
            })
            .collect();
        if !source.is_empty() {
            source[0] = 0xa0 | (source[0] % 3);
        }
        assert_eq!(
            support::vertex(count, 4, &source, 1, 9),
            decode_vertex_buffer(count, 4, &source, &mut Workspace::default())
        );
        if !source.is_empty() {
            source[0] = 0xe0 | (source[0] % 3);
        }
        assert_eq!(
            support::triangles(count * 3, &source, 1, 2),
            decode_index_buffer(count * 3, 4, &source, &mut Workspace::default())
        );
    }
}

#[cfg(all(feature = "simd", feature = "parity-internals"))]
#[test]
fn every_available_simd_path() {
    for level in [
        Level::Scalar,
        Level::Sse2,
        Level::Ssse3,
        Level::Sse41,
        Level::Avx2,
        Level::Avx512,
        Level::Neon,
        Level::Wasm,
    ] {
        if with_level(level, || ()).is_ok() {
            with_level(level, random_roundtrips_and_malformed).unwrap();
        }
    }
}

#[test]
fn empty_output_sticky_and_narrowing() {
    use meshoptimizer_rs::{Error, Limits};
    let indices = [u32::MAX, 70000, 0];
    let encoded = encode_index_buffer(
        &indices,
        IndexEncoding::new(1).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    let end = encoded.len() - 16;
    let mut d = IndexBufferDecoder::new(
        3,
        &encoded[..2],
        &encoded[end..],
        end - 2,
        Limits::default(),
    )
    .unwrap();
    let p = d.push(&encoded[2..end], &mut []).unwrap();
    assert_eq!(p.status, Status::NeedOutput);
    let mut output = [0u16; 3];
    let p2 = d
        .push_generic(&encoded[2 + p.consumed..end], &mut output)
        .unwrap();
    assert_eq!(p2.status, Status::Done);
    let expected = decode_index_buffer(3, 2, &encoded, &mut Workspace::default()).unwrap();
    assert_eq!(
        output
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(d.push(&[0], &mut []), Err(Error::InvalidStream));
    assert_eq!(d.push(&[], &mut []), Err(Error::InvalidStream));
    assert_eq!(d.finish(), Err(Error::InvalidStream));
}

#[test]
fn upstream_corpus() {
    let Ok(directory) = std::env::var("STREAM_FIXTURES") else {
        return;
    };
    let mut cases = 0;
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("input") {
            continue;
        }
        let bytes = std::fs::read(path).unwrap();
        let word = |i: usize| {
            usize::try_from(u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap())).unwrap()
        };
        let (op, n, stride) = (word(4), word(8), word(12));
        let source = &bytes[44..];
        for chunk in [1, 7, 4096, 65536] {
            if op == 1 {
                assert_eq!(
                    support::vertex(n, stride, source, chunk, 19),
                    decode_vertex_buffer(n, stride, source, &mut Workspace::default())
                );
                cases += 1;
            } else if op == 2 {
                assert_eq!(
                    support::triangles(n, source, chunk, 2),
                    decode_index_buffer(n, 4, source, &mut Workspace::default())
                );
                cases += 1;
            }
        }
    }
    assert!(cases > 0);
    println!("upstream corpus comparisons: {cases}");
}

#[test]
fn metadata_limits_and_vertex_stickiness() {
    use meshoptimizer_rs::{Error, Limits};
    let encoded = encode_vertex_buffer(
        &[0; 4],
        1,
        4,
        VertexEncoding::new(0, 2).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    let end = encoded.len() - 32;
    let limits = Limits {
        max_bytes: 0,
        max_work: u64::try_from(encoded.len() + 4).unwrap(),
    };
    let mut d =
        VertexBufferDecoder::new(1, 4, encoded[0], &encoded[end..], end - 1, limits).unwrap();
    let p = d.push(&encoded[1..end], &mut []).unwrap();
    assert_eq!(p.status, Status::NeedOutput);
    assert_eq!(d.finish(), Err(Error::InvalidStream));
    let mut d =
        VertexBufferDecoder::new(1, 4, encoded[0], &encoded[end..], end - 1, limits).unwrap();
    assert_eq!(
        d.push(&encoded[1..end], &mut [0; 4]).unwrap().status,
        Status::Done
    );
    assert_eq!(d.push(&[0], &mut []), Err(Error::InvalidStream));
    assert_eq!(d.push(&[], &mut []), Err(Error::InvalidStream));
    assert_eq!(d.finish(), Err(Error::InvalidStream));
    assert_eq!(
        VertexBufferDecoder::new(
            1,
            4,
            encoded[0],
            &encoded[end..],
            end - 1,
            Limits {
                max_work: limits.max_work - 1,
                ..limits
            }
        )
        .err(),
        Some(Error::LimitExceeded)
    );
    assert_eq!(
        VertexBufferDecoder::new(usize::MAX, 4, 0xa0, &[0; 32], 0, limits).err(),
        Some(Error::SizeOverflow)
    );
}

#[test]
fn long_triangle_bodies_cycle_the_input_window() {
    // Random indices cost several bytes each, so the body is many times the
    // 9216-byte scratch and the pending window is compacted repeatedly.
    let mut rng = 5u32;
    for version in 0..=1 {
        let indices: Vec<u32> = (0..36_000)
            .map(|_| {
                rng ^= rng << 13;
                rng ^= rng >> 17;
                rng ^= rng << 5;
                rng % 100_000
            })
            .collect();
        let encoded = encode_index_buffer(
            &indices,
            IndexEncoding::new(version).unwrap(),
            &mut Workspace::default(),
        )
        .unwrap();
        assert!(encoded.len() > 3 * 9216);
        let expected =
            decode_index_buffer(indices.len(), 4, &encoded, &mut Workspace::default()).unwrap();
        for (chunk, width) in [(509, 1000), (65536, 7), (61, 100_000)] {
            assert_eq!(
                support::triangles(indices.len(), &encoded, chunk, width).unwrap(),
                expected
            );
        }
    }
}
