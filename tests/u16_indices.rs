use meshoptimizer_rs::*;

fn narrow(values: &[u32]) -> Vec<u16> {
    values
        .iter()
        .map(|&value| u16::try_from(value).expect("small corpus"))
        .collect()
}

#[test]
fn corpus_matches_u32() -> Result<(), Error> {
    for side in [2u16, 3, 8, 17, 32] {
        let positions: Vec<_> = (0..side)
            .flat_map(|y| {
                (0..side).map(move |x| [f32::from(x), f32::from(y), f32::from(x % 3) * 0.1])
            })
            .collect();
        let mut indices = Vec::<u16>::new();
        for y in 0..side - 1 {
            for x in 0..side - 1 {
                let a = y * side + x;
                indices.extend_from_slice(&[a, a + 1, a + side, a + side, a + 1, a + side + 1]);
            }
        }
        let wide: Vec<_> = indices.iter().copied().map(u32::from).collect();
        let p = Positions::from_packed(&positions);
        let mut ws = Workspace::default();
        macro_rules! same {
            ($f:ident $(, $arg:expr)*) => {{
                let a = $f(&indices, $($arg,)* &mut ws)?;
                let b = $f(&wide, $($arg,)* &mut ws)?;
                assert_eq!(a, narrow(&b), stringify!($f));
            }};
        }
        same!(optimize_vertex_cache_generic, positions.len());
        same!(optimize_vertex_cache_strip_generic, positions.len());
        same!(optimize_vertex_cache_fifo_generic, positions.len(), 16);
        same!(optimize_overdraw_generic, p, 1.05);
        same!(spatial_sort_triangles_generic, p);
        same!(
            generate_shadow_index_buffer_generic,
            VertexStream::packed(&vec![0; positions.len() * 4], 4)?
        );
        let settings = SimplifySettings {
            target_index_count: indices.len() / 6 * 3,
            target_error: 1.0,
            options: SimplifyOptions::EMPTY,
        };
        let a = simplify_generic(&indices, p, settings, &mut ws)?;
        let b = simplify_generic(&wide, p, settings, &mut ws)?;
        assert_eq!(a.indices, narrow(&b.indices));
        assert_eq!(a.error.to_bits(), b.error.to_bits());
        let a = build_meshlets_scan_generic(
            &indices,
            positions.len(),
            MeshletSettings::default(),
            &mut ws,
        )?;
        let b = build_meshlets_scan_generic(
            &wide,
            positions.len(),
            MeshletSettings::default(),
            &mut ws,
        )?;
        assert_eq!(a.vertices, narrow(&b.vertices));
        assert_eq!(a.meshlets, b.meshlets);
        assert_eq!(a.triangles, b.triangles);
        let a = sanitize_generic(&indices, p, SanitizeOptions::default(), &mut ws)?;
        let b = sanitize_generic(&wide, p, SanitizeOptions::default(), &mut ws)?;
        assert_eq!(a.indices, narrow(&b.indices));
        let encoding = codec::IndexEncoding::DEFAULT;
        assert_eq!(
            codec::encode_index_buffer_generic(&indices, encoding, &mut ws)?,
            codec::encode_index_buffer_generic(&wide, encoding, &mut ws)?
        );
        assert_eq!(
            codec::encode_index_sequence_generic(&indices, encoding, &mut ws)?,
            codec::encode_index_sequence_generic(&wide, encoding, &mut ws)?
        );
        let mut out = vec![u16::MAX; indices.len() + 3];
        optimize_vertex_cache_into_generic(&mut out, &indices, positions.len(), &mut ws)?;
        assert_eq!(
            &out[..indices.len()],
            optimize_vertex_cache_generic(&indices, positions.len(), &mut ws)?
        );
        assert_eq!(&out[indices.len()..], &[u16::MAX; 3]);
        let strip = stripify_generic(&indices, positions.len(), u16::MAX, &mut ws)?;
        let wide_strip = stripify_generic(&wide, positions.len(), u32::from(u16::MAX), &mut ws)?;
        assert_eq!(strip, narrow(&wide_strip));
        assert_eq!(
            unstripify_generic(&strip, u16::MAX, &mut ws)?,
            narrow(&unstripify_generic(
                &wide_strip,
                u32::from(u16::MAX),
                &mut ws
            )?)
        );
    }
    Ok(())
}

#[test]
fn narrowing_is_checked_and_preserves_destinations() -> Result<(), Error> {
    let mut ws = Workspace::default();
    let limits = ws.limits();
    assert_eq!(
        remap_index_buffer_generic::<u16>(None, &[65536], &mut ws),
        Err(Error::IndexOverflow)
    );
    assert_eq!(
        remap_index_buffer_generic::<u16>(None, &[65535], &mut ws)?,
        [u16::MAX]
    );
    let mut out = [91u16, 92];
    assert_eq!(
        remap_index_buffer_into_generic(&mut out, None, &[65536], &mut ws),
        Err(Error::IndexOverflow)
    );
    assert_eq!(out, [91, 92]);
    assert_eq!(ws.limits().max_bytes, limits.max_bytes);
    assert_eq!(ws.limits().max_work, limits.max_work);
    let encoded =
        codec::encode_index_sequence_generic(&[65536u32], codec::IndexEncoding::DEFAULT, &mut ws)?;
    let mut decoder = codec::IndexSequenceDecoder::new(1, Limits::default())?;
    assert_eq!(
        decoder.push_generic(&encoded, &mut [0u16]),
        Err(Error::IndexOverflow)
    );
    // Existing byte-layout decoders retain the upstream conversion rules.
    assert_eq!(
        codec::decode_index_sequence(1, 2, &encoded, &mut ws)?,
        [0, 0]
    );
    Ok(())
}

#[test]
fn adapters_obey_storage_limits() {
    let limits = Limits {
        max_bytes: 3,
        max_work: 100,
    };
    let mut ws = Workspace::new(limits);
    assert_eq!(
        optimize_vertex_cache_generic(&[0u16, 0, 0], 1, &mut ws),
        Err(Error::LimitExceeded)
    );
    assert_eq!(ws.limits().max_bytes, limits.max_bytes);
    assert_eq!(ws.limits().max_work, limits.max_work);
}

#[test]
fn streaming_and_meshlet_decoding_reject_overflow() -> Result<(), Error> {
    let mut ws = Workspace::default();
    let encoded = codec::encode_index_sequence_generic(
        &[65535u32, 65536],
        codec::IndexEncoding::DEFAULT,
        &mut ws,
    )?;
    let mut decoder = codec::IndexSequenceDecoder::new(2, Limits::default())?;
    let mut out = [0u16; 2];
    assert_eq!(
        decoder.push_generic(&encoded, &mut out),
        Err(Error::IndexOverflow)
    );
    assert_eq!(decoder.finish(), Err(Error::IndexOverflow));
    let encoded = codec::encode_meshlet_generic(&[65536u32], &[0, 0, 0], &mut ws)?;
    assert!(matches!(
        codec::decode_meshlet_raw_generic::<u16>(1, 1, &encoded, &mut ws),
        Err(Error::IndexOverflow)
    ));
    assert_eq!(
        codec::decode_meshlet(1, 2, 1, 3, &encoded, &mut ws)?.vertices,
        [0, 0]
    );
    let encoded = codec::encode_meshlet_generic(&[65535u16], &[0, 0, 0], &mut ws)?;
    assert_eq!(
        codec::decode_meshlet_raw_generic::<u16>(1, 1, &encoded, &mut ws)?.vertices,
        [u16::MAX]
    );
    Ok(())
}

#[test]
fn u32_adapters_preserve_storage_identity() -> Result<(), Error> {
    let mut ws = Workspace::default();
    let input = [0u32, 1, 2];
    <u32 as Index>::with_input(&input, &mut ws, |wide, _| {
        assert_eq!(wide.as_ptr(), input.as_ptr());
        Ok(())
    })?;
    let owned = input.to_vec();
    let pointer = owned.as_ptr();
    let output = <u32 as Index>::from_vec(owned, &mut ws)?;
    assert_eq!(output.as_ptr(), pointer);
    Ok(())
}

#[test]
fn callback_unwind_restores_workspace_limits() {
    let mut ws = Workspace::default();
    let limits = ws.limits();
    let positions = [[0.0; 3]; 3];
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = generate_vertex_remap_custom_generic(
            Some(&[0u16, 1, 2]),
            Positions::from_packed(&positions),
            |_, _| panic!("callback"),
            &mut ws,
        );
    }));
    assert!(result.is_err());
    assert_eq!(ws.limits().max_bytes, limits.max_bytes);
    assert_eq!(ws.limits().max_work, limits.max_work);
}

#[test]
fn generated_indices_are_checked() -> Result<(), Error> {
    let mut ws = Workspace::default();
    let positions = vec![[0.0; 3]; 65537];
    assert_eq!(
        spatial_sort_remap_generic::<u16>(Positions::from_packed(&positions), &mut ws),
        Err(Error::IndexOverflow)
    );
    let indices: Vec<u16> = [0, 1, 2].into_iter().cycle().take(65537 * 3).collect();
    assert!(matches!(
        generate_provoking_index_buffer_generic(&indices, 3, &mut ws),
        Err(Error::IndexOverflow)
    ));
    Ok(())
}

#[test]
fn generic_lod_carriers() -> Result<(), Error> {
    let p = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
    ];
    let indices = [0u16, 1, 2, 2, 1, 3];
    let positions = Positions::from_packed(&p);
    let mut ws = Workspace::default();
    let options = LodChainOptions::new(LodTargets::Counts(&[3]), LodErrors::Uniform(1.0));
    let chain: LodChainGeneric<u16> =
        build_lod_chain_generic(&indices, positions, &options, &mut ws)
            .map_err(|error| error.error)?;
    let wide: Vec<u32> = indices.iter().copied().map(u32::from).collect();
    let reference = build_lod_chain_generic(&wide, positions, &options, &mut ws)
        .map_err(|error| error.error)?;
    assert_eq!(
        chain.levels[0].indices,
        narrow(&reference.levels[0].indices)
    );
    let mesh = LodQualityMeshGeneric::new_generic(&indices, positions);
    let quality = measure_lod_quality_generic(mesh, mesh, &LodQualityOptions::default(), &mut ws)?;
    assert!(quality.symmetric.max <= 1e-12);
    let quality = measure_lod_chain_quality_generic(
        mesh,
        &chain,
        &LodQualityOptions::default(),
        None,
        &mut ws,
    )?;
    assert_eq!(quality.len(), 1);
    Ok(())
}

#[cfg(feature = "parallel")]
#[test]
fn parallel_descriptors_accept_u16() -> Result<(), Error> {
    use meshoptimizer_rs::parallel::*;
    let indices = [0u16, 1, 2];
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let input = MeshletInputGeneric {
        indices: &indices,
        positions: Positions::from_packed(&positions),
        settings: MeshletSettings::default(),
        builder: MeshletBuilder::Scan,
    };
    let out = build_meshlets_batch_generic(&[input], Limits::default())?;
    assert_eq!(
        out[0].as_ref().map(|mesh| &mesh.vertices),
        Ok(&vec![0u16, 1, 2])
    );
    let input = EncodeInputGeneric::Triangles {
        indices: &indices,
        encoding: codec::IndexEncoding::DEFAULT,
    };
    let out = encode_buffers_batch_generic(&[input], Limits::default())?;
    assert!(out[0].is_ok());
    Ok(())
}

#[test]
fn standalone_u32_types_keep_literal_inference() {
    let local = LocalMeshlet {
        vertices: vec![0, 1, 2],
        triangles: vec![0, 1, 2],
    };
    assert_eq!(core::mem::size_of_val(&local.vertices[0]), 4);
    let fetched = FetchedMesh {
        indices: vec![0, 1, 2],
        vertices: vec![],
        vertex_count: 3,
    };
    assert_eq!(core::mem::size_of_val(&fetched.indices[0]), 4);
}

#[cfg(feature = "parallel")]
#[test]
fn original_encode_variants_remain_importable() {
    use meshoptimizer_rs::parallel::EncodeInput::*;
    let input = Sequence {
        indices: &[0, 1, 2],
        encoding: codec::IndexEncoding::DEFAULT,
    };
    assert!(input.encode(&mut Workspace::default()).is_ok());
}
