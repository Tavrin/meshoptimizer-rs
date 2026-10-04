// 0.4 codec completion: upstream golden vectors, configuration, capacity and
// malformed-input behavior. Exhaustive differential parity lives in parity/.
// Float literals are upstream's test inputs verbatim, not approximations.
#![allow(clippy::approx_constant)]
use meshoptimizer_rs::{codec::*, Error, Limits, Workspace};

const INDEX_BUFFER: [u32; 15] = [0, 1, 2, 2, 1, 3, 0, 1, 2, 2, 1, 5, 2, 1, 4];
const INDEX_DATA_V1: [u8; 24] = [
    0xe1, 0xf0, 0x10, 0xfe, 0x1f, 0x3d, 0x00, 0x0a, 0x00, 0x76, 0x87, 0x56, 0x67, 0x78, 0xa9, 0x86,
    0x65, 0x89, 0x68, 0x98, 0x01, 0x69, 0x00, 0x00,
];

// kVertexBuffer of upstream demo/tests.cpp: four 12-byte PV records.
fn vertex_buffer() -> Vec<u8> {
    let rows: [[u16; 6]; 4] = [
        [0, 0, 0, 0, 0, 0],
        [300, 0, 0, 0, 500, 0],
        [0, 300, 0, 0, 0, 500],
        [300, 300, 0, 0, 500, 500],
    ];
    rows.iter()
        .flat_map(|r| r.iter().flat_map(|v| v.to_le_bytes()))
        .collect()
}

#[test]
fn encoders_reproduce_upstream_goldens() {
    let mut w = Workspace::default();
    let v1 = IndexEncoding::new(1).unwrap();
    assert_eq!(
        encode_index_buffer(&INDEX_BUFFER, v1, &mut w).unwrap(),
        INDEX_DATA_V1
    );
    let sequence = encode_index_sequence(&[0, 1, 51, 2, 49, 1000], v1, &mut w).unwrap();
    assert_eq!(
        sequence,
        [0xd1, 0x00, 0x04, 0xcd, 0x01, 0x04, 0x07, 0x98, 0x1f, 0x00, 0x00, 0x00, 0x00]
    );
    // kVertexDataV0 of upstream demo/tests.cpp is the version 0 encoder output.
    let expected_v0: [u8; 85] = [
        0xa0, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x58, 0x57, 0x58, 0x01, 0x26, 0x00, 0x00, 0x00, 0x01,
        0x0c, 0x00, 0x00, 0x00, 0x58, 0x01, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x3f, 0x00, 0x00, 0x00, 0x17, 0x18, 0x17, 0x01, 0x26, 0x00, 0x00, 0x00, 0x01, 0x0c, 0x00,
        0x00, 0x00, 0x17, 0x01, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let vertices = vertex_buffer();
    let v0 = VertexEncoding::new(0, 2).unwrap();
    let encoded = encode_vertex_buffer(&vertices, 4, 12, v0, &mut w).unwrap();
    assert_eq!(encoded, expected_v0);
    // Version 1 at level 2, as recorded from the scalar C++ 1.3 oracle (kVertexDataV1
    // upstream is a decoder vector, not current encoder output).
    let expected_v1: [u8; 60] = [
        0xa1, 0xff, 0xaa, 0xff, 0x00, 0x58, 0x57, 0x58, 0x00, 0x02, 0x01, 0x02, 0x00, 0x00, 0x58,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x17, 0x18, 0x17, 0x00, 0x02, 0x01, 0x02, 0x00, 0x00,
        0x17, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let encoded = encode_vertex_buffer(&vertices, 4, 12, VertexEncoding::DEFAULT, &mut w).unwrap();
    assert_eq!(encoded, expected_v1);
    assert_eq!(
        decode_vertex_buffer(4, 12, &encoded, &mut w).unwrap(),
        vertices
    );
    let oct = [
        1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.7071068, 0.0, 0.707168, 1.0, -0.7071068, 0.0,
        -0.707168, 1.0,
    ];
    assert_eq!(
        encode_filter_oct(4, 4, 8, &oct, &mut w).unwrap(),
        [0x7f, 0, 0x7f, 0, 0, 0x81, 0x7f, 0, 0x3f, 0, 0x7f, 0x7f, 0x81, 0x40, 0x7f, 0x7f]
    );
    let color = [
        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.5, 0.0, 0.0, 1.0, 0.25, 0.4, 0.4, 0.4, 0.75,
    ];
    let mut encoded = encode_filter_color(4, 4, 8, &color, &mut w).unwrap();
    assert_eq!(
        encoded,
        [
            0x40, 0x7f, 0xc1, 0xff, 0x7f, 0x00, 0x7f, 0xc0, 0x40, 0x81, 0xc0, 0xa0, 0x66, 0x00,
            0x00, 0xdf
        ]
    );
    decode_filter_color(&mut encoded, 4, 4, &mut w).unwrap();
    // Grayscale is preserved exactly.
    assert!(encoded[12] == encoded[13] && encoded[12] == encoded[14]);
}

#[test]
fn explicit_configuration_replaces_global_versions() {
    assert_eq!(VertexEncoding::new(2, 0), Err(Error::UnsupportedVersion));
    assert_eq!(VertexEncoding::new(1, 10), Err(Error::InvalidParameter));
    assert_eq!(IndexEncoding::new(2), Err(Error::UnsupportedVersion));
    let mut w = Workspace::default();
    let vertices = vertex_buffer();
    // Concurrent-style alternation: each call carries its own version.
    for (version, header) in [(0u8, 0xa0u8), (1, 0xa1), (0, 0xa0)] {
        for level in 0..=9 {
            let e = VertexEncoding::new(version, level).unwrap();
            let encoded = encode_vertex_buffer(&vertices, 4, 12, e, &mut w).unwrap();
            assert_eq!(encoded[0], header);
            assert_eq!(
                decode_vertex_buffer(4, 12, &encoded, &mut w).unwrap(),
                vertices
            );
        }
        let e = IndexEncoding::new(version).unwrap();
        assert_eq!(
            encode_index_buffer(&INDEX_BUFFER, e, &mut w).unwrap()[0],
            0xe0 | version
        );
        assert_eq!(
            encode_index_sequence(&[1], e, &mut w).unwrap()[0],
            0xd0 | version
        );
    }
    assert_eq!(
        encode_vertex_buffer(&vertices, 4, 6, VertexEncoding::DEFAULT, &mut w),
        Err(Error::InvalidLayout)
    );
    assert_eq!(
        encode_index_buffer(&[0, 1], IndexEncoding::DEFAULT, &mut w),
        Err(Error::InvalidTopology)
    );
}

#[test]
fn encoder_capacity_matches_reference_zero_returns() {
    let mut w = Workspace::default();
    let vertices = vertex_buffer();
    let e = VertexEncoding::DEFAULT;
    let bound = encode_vertex_buffer_bound(4, 12).unwrap();
    let size = encode_vertex_buffer(&vertices, 4, 12, e, &mut w)
        .unwrap()
        .len();
    assert!(size <= bound);
    let mut out = vec![0; bound];
    assert_eq!(
        encode_vertex_buffer_into(&mut out, &vertices, 4, 12, e, &mut w),
        Ok(size)
    );
    // Too small by one byte fails; the encoder never writes past the slice.
    assert_eq!(
        encode_vertex_buffer_into(&mut out[..size - 1], &vertices, 4, 12, e, &mut w),
        Err(Error::BufferTooSmall)
    );
    let ie = IndexEncoding::DEFAULT;
    let mut out = [0u8; 64];
    assert_eq!(
        encode_index_buffer_into(&mut out, &INDEX_BUFFER, ie, &mut w),
        Ok(24)
    );
    // The reference rejects any buffer without sixteen bytes of slack per triangle.
    assert_eq!(
        encode_index_buffer_into(&mut out[..23], &INDEX_BUFFER, ie, &mut w),
        Err(Error::BufferTooSmall)
    );
    assert_eq!(encode_index_buffer_bound(12, 10), Ok(1 + 4 * 5 + 16));
    assert_eq!(encode_index_sequence_bound(6, 1001), Ok(1 + 6 * 2 + 4));
    // Empty inputs encode to the minimal valid streams.
    assert_eq!(
        encode_vertex_buffer(&[], 0, 16, e, &mut w).unwrap().len(),
        1 + 16 + 16 / 4 + 4
    );
    assert_eq!(encode_index_buffer(&[], ie, &mut w).unwrap().len(), 17);
    assert_eq!(
        encode_index_sequence(&[], ie, &mut w).unwrap(),
        [0xd1, 0, 0, 0, 0]
    );
    // Allocating encoders account the bound before reserving it.
    let mut tight = Workspace::new(Limits {
        max_bytes: bound - 1,
        max_work: 1 << 20,
    });
    assert_eq!(
        encode_vertex_buffer(&vertices, 4, 12, e, &mut tight),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn sequence_baseline_negation_keeps_the_observed_reference_wrap() {
    // A delta of exactly 2^31 is undefined in C++; pinned GCC builds do not
    // switch baselines (low bit 0), and neither does this port.
    let mut w = Workspace::default();
    let indices = [0x8000_0000, 0, 0x8000_0000];
    let encoded = encode_index_sequence(&indices, IndexEncoding::DEFAULT, &mut w).unwrap();
    assert_eq!(
        encoded,
        [
            0xd1, 0xfe, 0xff, 0xff, 0xff, 0x0f, 0xfe, 0xff, 0xff, 0xff, 0x0f, 0xfe, 0xff, 0xff,
            0xff, 0x0f, 0, 0, 0, 0
        ]
    );
    // Bit 31 of such zigzag deltas is not representable, upstream included:
    // the stream decodes, but large-delta sequences are not lossless.
    assert!(decode_index_sequence(3, 4, &encoded, &mut w).is_ok());
}

#[test]
fn filter_encoders_validate_and_reject_undefined_conversions() {
    let mut w = Workspace::default();
    let four = [0.5f32; 4];
    assert_eq!(
        encode_filter_oct(1, 4, 9, &four, &mut w),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        encode_filter_quat(1, 8, 3, &four, &mut w),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        encode_filter_color(1, 12, 8, &four, &mut w),
        Err(Error::InvalidLayout)
    );
    assert_eq!(
        encode_filter_oct(2, 8, 8, &four, &mut w),
        Err(Error::InvalidLayout)
    );
    let modes = [
        ExpMode::Separate,
        ExpMode::SharedVector,
        ExpMode::SharedComponent,
        ExpMode::Clamped,
    ];
    for mode in modes {
        assert_eq!(
            encode_filter_exp(1, 8, 8, &[1.0, f32::NAN], mode, &mut w),
            Err(Error::NumericalFailure)
        );
        // One-bit mantissas with exponent 128 have no defined reference result.
        assert_eq!(
            encode_filter_exp(1, 4, 1, &[2f32.powi(127)], mode, &mut w),
            Err(Error::NumericalFailure)
        );
        assert!(encode_filter_exp(1, 4, 2, &[2f32.powi(127)], mode, &mut w).is_ok());
    }
    // Exp zero values inherit the previous exponent in Separate mode.
    let encoded = encode_filter_exp(2, 4, 15, &[1.0, 0.0], ExpMode::Separate, &mut w).unwrap();
    assert_eq!(encoded[3], encoded[7]);
    // A zero alpha word makes the reference scale infinite.
    assert_eq!(
        decode_filter_color(&mut [10, 0, 0, 0], 1, 4, &mut w),
        Err(Error::NumericalFailure)
    );
    let mut wide = [0xff, 0xff, 0xff, 0x7f, 0, 0x80, 1, 0];
    assert_eq!(
        decode_filter_color(&mut wide, 1, 8, &mut w),
        Err(Error::NumericalFailure)
    );
    let mut tail = [0u8; 20];
    encode_filter_quat_into(
        &mut tail[..16],
        2,
        8,
        12,
        &[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0],
        &mut w,
    )
    .unwrap();
    assert_eq!(&tail[16..], &[0; 4]);
}

#[test]
fn meshlet_codec_round_trips_and_rejects_malformed_streams() {
    let mut w = Workspace::default();
    let triangles = [0u8, 1, 2, 2, 1, 3, 3, 5, 4, 2, 0, 6, 6, 6, 6];
    let vertices = [5u32, 12, 140, 0, 12389, 123456789, 7];
    let bound = encode_meshlet_bound(7, 5).unwrap();
    let encoded = encode_meshlet(&vertices, &triangles, &mut w).unwrap();
    assert!(!encoded.is_empty() && encoded.len() <= bound);
    let mut short = vec![0xcc; encoded.len() - 1];
    assert_eq!(
        encode_meshlet_into(&mut short, &vertices, &triangles, &mut w),
        Err(Error::BufferTooSmall)
    );
    assert!(short.iter().all(|&b| b == 0xcc));
    let raw = decode_meshlet_raw(7, 5, &encoded, &mut w).unwrap();
    assert_eq!(raw.vertices, vertices);
    for (t, packed) in triangles.as_chunks::<3>().0.iter().zip(&raw.triangles) {
        let d = [*packed as u8, (packed >> 8) as u8, (packed >> 16) as u8];
        let rotations = [[t[0], t[1], t[2]], [t[1], t[2], t[0]], [t[2], t[0], t[1]]];
        assert!(rotations.contains(&d));
    }
    for (vs, ts) in [(2, 3), (2, 4), (4, 3), (4, 4)] {
        let d = decode_meshlet(7, vs, 5, ts, &encoded, &mut w).unwrap();
        assert_eq!(d.vertices.len(), 7 * vs);
        assert_eq!(d.triangles.len(), 5 * ts);
        assert_eq!(d.vertices[vs..vs + 1], [12]);
    }
    for i in 1..encoded.len() {
        assert!(decode_meshlet_raw(7, 5, &encoded[..i], &mut w).is_err());
        assert!(decode_meshlet_raw(7, 5, &encoded[i..], &mut w).is_err());
    }
    assert_eq!(
        decode_meshlet(7, 3, 5, 3, &encoded, &mut w),
        Err(Error::InvalidLayout)
    );
    assert_eq!(
        encode_meshlet(&[0; 257], &[], &mut w),
        Err(Error::InvalidParameter)
    );
    // An empty meshlet is its sixteen-byte gap.
    assert_eq!(encode_meshlet(&[], &[], &mut w).unwrap(), [0; 16]);
    decode_meshlet_raw_into(&mut [], 0, &mut [], 0, &[0; 16], &mut w).unwrap();
}
