//! Valid-input differential against the pinned meshopt 0.7.0 C++ 1.3 oracle.
use candidate as m;

#[repr(transparent)]
#[derive(Clone)]
struct Record<const N: usize>([u8; N]);
impl<const N: usize> Default for Record<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

fn check<const N: usize>() -> usize {
    let mut comparisons = 0;
    for count in [0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 63, 64, 65, 257, 4097] {
        let records: Vec<Record<N>> = (0..count)
            .map(|i| Record(core::array::from_fn(|j| (i * 37 + j * 11) as u8)))
            .collect();
        for variant in 0..4 {
            let remap: Vec<u32> = (0..count).map(|i| match variant {
                0 => i as u32,
                1 if i % 3 == 0 => u32::MAX,
                1 => (count - 1) as u32,
                2 => (i / 3 * 3) as u32,
                _ => 0,
            }).collect();
            let required = remap.iter().filter(|&&r| r != u32::MAX)
                .map(|&r| r as usize + 1).max().unwrap_or(0);
            let expected = meshopt::remap_vertex_buffer(&records, required, &remap);
            let expected: Vec<u8> = expected.iter().flat_map(|r| r.0).collect();
            for padding in [0, 1, 3] {
                let stride = N + padding;
                let offset = padding;
                let mut data = vec![0x5a; offset + count * stride + 5];
                for (i, r) in records.iter().enumerate() {
                    data[offset + i * stride..offset + i * stride + N].copy_from_slice(&r.0);
                }
                let stream = m::VertexStream::new(&data, count, N, stride, offset).unwrap();
                let actual = m::remap_vertex_buffer(stream, &remap, &mut m::Workspace::default()).unwrap();
                assert_eq!(actual, expected, "width {N} count {count} variant {variant} padding {padding}");
                comparisons += 1;
            }
            let indices: Vec<u32> = (0..count).rev().filter(|&i| remap[i] != u32::MAX)
                .flat_map(|i| [i as u32, i as u32]).collect();
            let expected = meshopt::remap_index_buffer(Some(&indices), count, &remap);
            let actual = m::remap_index_buffer(Some(&indices), &remap, &mut m::Workspace::default()).unwrap();
            assert_eq!(actual, expected);
            comparisons += 1;
            if remap.iter().all(|&r| r != u32::MAX) {
                assert_eq!(m::remap_index_buffer(None, &remap, &mut m::Workspace::default()).unwrap(),
                    meshopt::remap_index_buffer(None, count, &remap));
                comparisons += 1;
            }
        }
    }
    comparisons
}

fn main() {
    let comparisons = check::<1>() + check::<3>() + check::<4>() + check::<8>()
        + check::<12>() + check::<16>() + check::<20>() + check::<32>();
    println!("C++ 1.3 comparisons {comparisons}, differences 0");
}
