use core::mem::{offset_of, size_of};
use meshoptimizer_rs::*;

#[repr(C)]
struct Vertex {
    tag: u8,
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    tail: u8,
}

#[test]
fn padded_layout_matches_independently_written_bytes() {
    let vertices = [
        Vertex {
            tag: 17,
            position: [-0.0, 2.0, 3.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.5, 1.0],
            tail: 42,
        },
        Vertex {
            tag: 18,
            position: [4.0, 5.0, 6.0],
            normal: [1.0, 0.0, 0.0],
            uv: [1.0, 0.5],
            tail: 43,
        },
    ];
    assert_eq!(size_of::<Vertex>(), 40);
    let layout = VertexLayout::new(offset_of!(Vertex, position), |v: &Vertex| &v.position).unwrap();
    assert_eq!((layout.stride(), layout.offset()), (40, 4));
    let data = layout.copy_from(&vertices).unwrap();
    let mut bytes = vec![0; 80];
    for (i, vertex) in vertices.iter().enumerate() {
        for (j, value) in vertex.position.iter().enumerate() {
            bytes[i * 40 + 4 + j * 4..i * 40 + 8 + j * 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    assert_eq!(data.as_bytes(), bytes);
    assert_eq!((data.stride(), data.offset()), (40, 4));
    let manual = Positions::from_bytes(&bytes, 2, 40, 4, ByteOrder::LittleEndian).unwrap();
    for i in 0..2 {
        assert_eq!(
            data.positions().unwrap().get(i).unwrap().map(f32::to_bits),
            manual.get(i).unwrap().map(f32::to_bits)
        );
    }
    let uv = VertexBuffer::from_vertices(&vertices, offset_of!(Vertex, uv), |v| &v.uv).unwrap();
    for (i, vertex) in vertices.iter().enumerate() {
        for j in 0..2 {
            assert_eq!(uv.attributes().unwrap().get(i, j), Some(vertex.uv[j]));
        }
    }
    assert_eq!(
        uv.positions().unwrap_err(),
        VertexLayoutError::InvalidPositionSize
    );
}

#[test]
fn packed_aligned_empty_and_bit_patterns() {
    #[repr(C, align(32))]
    struct Aligned {
        position: [f32; 3],
    }
    let layout = VertexLayout::new(0, |v: &Aligned| &v.position).unwrap();
    assert_eq!(layout.stride(), 32);
    let data = layout
        .copy_from(&[Aligned {
            position: [1., 2., 3.],
        }])
        .unwrap();
    assert_eq!(data.positions().unwrap().get(0), Some([1., 2., 3.]));
    let values = [[-0.0, f32::from_bits(0x7fc12345), f32::INFINITY]];
    let data = VertexBuffer::from_vertices(&values, 0, |v| v).unwrap();
    assert_eq!((data.stride(), data.offset()), (12, 0));
    assert_eq!(
        data.positions().unwrap().get(0).unwrap().map(f32::to_bits),
        values[0].map(f32::to_bits)
    );
    let empty =
        VertexBuffer::from_vertices::<Vertex>(&[], offset_of!(Vertex, position), |v| &v.position)
            .unwrap();
    assert!(empty.positions().unwrap().is_empty());
    assert!(empty.attributes().unwrap().is_empty());
    assert!(empty.as_bytes().is_empty());
}

#[test]
fn rejects_invalid_layouts_and_selector_addresses() {
    use VertexLayoutError::*;
    assert_eq!(
        VertexLayout::new(1, |v: &Vertex| &v.position).err(),
        Some(MisalignedField)
    );
    assert_eq!(
        VertexLayout::new(usize::MAX - 3, |v: &Vertex| &v.position).err(),
        Some(InvalidFieldSize)
    );
    assert_eq!(
        VertexLayout::new(36, |v: &Vertex| &v.position).err(),
        Some(InvalidFieldSize)
    );
    assert_eq!(
        VertexLayout::new(0, |v: &[f32; 33]| v).err(),
        Some(InvalidFieldSize)
    );
    assert_eq!(
        VertexLayout::new(0, |_: &[f32; 3]| &[]).err(),
        Some(InvalidFieldSize)
    );
    assert_eq!(
        VertexLayout::new(0, |_: &[u8; 13]| &ZERO).err(),
        Some(InvalidStride)
    );
    assert_eq!(
        VertexLayout::new(0, |v: &[f32; 0]| v).err(),
        Some(InvalidStride)
    );
    assert_eq!(
        VertexLayout::new(0, |v: &[f32; 65]| v).err(),
        Some(InvalidStride)
    );
    #[repr(C, packed)]
    struct Packed {
        bytes: [u8; 12],
    }
    static ZERO: [f32; 3] = [0.; 3];
    assert_eq!(
        VertexLayout::new(0, |_: &Packed| &ZERO).err(),
        Some(MisalignedField)
    );
    assert_eq!(
        VertexBuffer::from_vertices(&[[1., 2., 3.]], 0, |_| &ZERO).unwrap_err(),
        FieldOffsetMismatch
    );
    let vertices = [[1., 2., 3., 4., 5., 6.]];
    assert_eq!(
        VertexBuffer::from_vertices(&vertices, 0, |v| v[3..].first_chunk::<3>().unwrap())
            .unwrap_err(),
        FieldOffsetMismatch
    );
    let vertices = [[0., 1., 2., 3., 4., 5.], [1., 2., 3., 4., 5., 6.]];
    assert_eq!(
        VertexBuffer::from_vertices(&vertices, 0, |v| {
            if v[0] == 0. {
                v.first_chunk::<3>().unwrap()
            } else {
                v[3..].first_chunk::<3>().unwrap()
            }
        })
        .unwrap_err(),
        FieldOffsetMismatch
    );
}

#[test]
fn simplify_and_meshlets_match_manual_views_bit_for_bit() {
    let vertices: Vec<Vertex> = (0..81)
        .map(|i| Vertex {
            tag: 0,
            position: [(i % 9) as f32, (i / 9) as f32, ((i % 9) as f32 * 0.3).sin()],
            normal: [0., 0., 1.],
            uv: [(i % 9) as f32 / 8., (i / 9) as f32 / 8.],
            tail: 0,
        })
        .collect();
    let packed: Vec<[f32; 3]> = vertices.iter().map(|v| v.position).collect();
    let mut floats = vec![0.; vertices.len() * 10];
    for (record, point) in floats.as_chunks_mut::<10>().0.iter_mut().zip(&packed) {
        record[1..4].copy_from_slice(point);
    }
    let mut indices = Vec::new();
    for y in 0..8 {
        for x in 0..8 {
            let a = y * 9 + x;
            indices.extend([a, a + 1, a + 9, a + 9, a + 1, a + 10]);
        }
    }
    let data = VertexBuffer::from_vertices(&vertices, 4, |v| &v.position).unwrap();
    let helper = data.positions().unwrap();
    let uv = VertexBuffer::from_vertices(&vertices, offset_of!(Vertex, uv), |v| &v.uv).unwrap();
    let uv_floats: Vec<f32> = vertices.iter().flat_map(|v| v.uv).collect();
    let uv_manual = Attributes::from_interleaved(&uv_floats, 81, 2, 2, 0).unwrap();
    for manual in [
        Positions::from_packed(&packed),
        Positions::from_interleaved(&floats, 81, 10, 1).unwrap(),
    ] {
        let settings = SimplifySettings {
            target_index_count: 96,
            target_error: 0.1,
            options: SimplifyOptions::EMPTY,
        };
        let mut a = Workspace::default();
        let mut b = Workspace::default();
        let actual = simplify(&indices, helper, settings, &mut a).unwrap();
        let expected = simplify(&indices, manual, settings, &mut b).unwrap();
        assert_eq!(actual.indices, expected.indices);
        assert_eq!(actual.error.to_bits(), expected.error.to_bits());
        let actual = simplify_with_attributes(
            &indices,
            helper,
            uv.attributes().unwrap(),
            &[0.5; 2],
            None,
            settings,
            &mut a,
        )
        .unwrap();
        let expected = simplify_with_attributes(
            &indices, manual, uv_manual, &[0.5; 2], None, settings, &mut b,
        )
        .unwrap();
        assert_eq!(actual.indices, expected.indices);
        assert_eq!(actual.error.to_bits(), expected.error.to_bits());
        let settings = MeshletSettings {
            max_vertices: 16,
            max_triangles: 12,
        };
        let actual = build_meshlets(&indices, helper, settings, 0.25, &mut a).unwrap();
        let expected = build_meshlets(&indices, manual, settings, 0.25, &mut b).unwrap();
        assert_eq!(actual.meshlets, expected.meshlets);
        assert_eq!(actual.vertices, expected.vertices);
        assert_eq!(actual.triangles, expected.triangles);
    }
}
