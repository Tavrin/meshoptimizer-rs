#![no_main]

use meshoptimizer_rs::{
    compute_cluster_bounds, compute_cluster_bounds_with_positions, compute_meshlet_bounds,
    compute_meshlet_bounds_with_positions, Bounds, BoundsPositions, ByteOrder, Error, Limits,
    Positions, Workspace,
};

struct Input<'a>(&'a [u8], usize);
impl Input<'_> {
    fn byte(&mut self) -> u8 {
        let value = self.0.get(self.1).copied().unwrap_or(0);
        self.1 += 1;
        value
    }
    fn word(&mut self) -> u32 {
        u32::from_le_bytes([self.byte(), self.byte(), self.byte(), self.byte()])
    }
    fn count(&mut self, max: usize) -> usize {
        u16::from_le_bytes([self.byte(), self.byte()]) as usize % (max + 1)
    }
}

// Compare every meaningful field, including signed zero and quantized cones.
fn bits(result: Result<Bounds, Error>) -> Result<(Vec<u32>, [i8; 4]), Error> {
    result.map(|b| {
        (
            b.center
                .into_iter()
                .chain([b.radius])
                .chain(b.cone_apex)
                .chain(b.cone_axis)
                .chain([b.cone_cutoff])
                .map(f32::to_bits)
                .collect(),
            [
                b.cone_axis_s8[0],
                b.cone_axis_s8[1],
                b.cone_axis_s8[2],
                b.cone_cutoff_s8,
            ],
        )
    })
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let mut input = Input(data, 0);
    let layout = input.byte();
    let n = input.count(320);
    let stride = 3 + input.byte() as usize % 66;
    let offset = input.byte() as usize % 17;
    let finite = input.byte() & 1 == 0;
    let packed: Vec<[f32; 3]> = (0..n)
        .map(|_| {
            core::array::from_fn(|_| {
                if finite {
                    input.byte() as f32 - 128.0
                } else {
                    f32::from_bits(input.word())
                }
            })
        })
        .collect();
    let order = if layout & 4 == 0 {
        ByteOrder::LittleEndian
    } else {
        ByteOrder::BigEndian
    };
    let mut floats = vec![0.0; offset + n * stride + 3];
    for (i, p) in packed.iter().enumerate() {
        floats[offset + i * stride..offset + i * stride + 3].copy_from_slice(p);
    }
    let mut bytes: Vec<u8> = floats
        .iter()
        .flat_map(|v| match order {
            ByteOrder::LittleEndian => v.to_le_bytes(),
            ByteOrder::BigEndian => v.to_be_bytes(),
        })
        .collect();
    // Also exercise unaligned byte offsets and truncated backing storage.
    let byte_offset = input.byte() as usize % 4;
    bytes.splice(..0, core::iter::repeat_n(0, byte_offset));
    if layout & 8 != 0 {
        let len = input.count(floats.len());
        floats.truncate(len);
        bytes.truncate(len);
    }
    let positions = match layout % 3 {
        0 => Ok(Positions::from_packed(&packed)),
        1 => Positions::from_interleaved(&floats, n, stride, offset),
        _ => Positions::from_bytes(&bytes, n, stride * 4, offset * 4 + byte_offset, order),
    };
    let Ok(positions) = positions else { return };
    let cached = BoundsPositions::new(positions);
    // The same handle retains successful and failed validation across queries.
    for _ in 0..4 {
        let mode = input.byte();
        let mut count = input.count(1540);
        if mode & 1 == 0 {
            count -= count % 3;
        }
        let indices: Vec<u32> = (0..count)
            .map(|_| {
                let v = input.word();
                if mode & 2 == 0 && n != 0 {
                    v % n as u32
                } else {
                    v
                }
            })
            .collect();
        let vertex_count = input.count(258);
        let vertices: Vec<u32> = (0..vertex_count)
            .map(|_| {
                let v = input.word();
                if mode & 4 == 0 && n != 0 {
                    v % n as u32
                } else {
                    v
                }
            })
            .collect();
        let triangles: Vec<u8> = (0..count)
            .map(|_| {
                let v = input.byte();
                if mode & 8 == 0 && vertex_count != 0 {
                    (v as usize % vertex_count) as u8
                } else {
                    v
                }
            })
            .collect();
        let limits = Limits {
            max_bytes: if mode & 16 == 0 {
                1 << 24
            } else {
                input.count(16384)
            },
            max_work: if mode & 32 == 0 {
                1 << 24
            } else {
                input.count(16384) as u64
            },
        };
        let mut old = Workspace::new(limits);
        let mut new = Workspace::new(limits);
        assert_eq!(
            bits(compute_cluster_bounds(&indices, positions, &mut old)),
            bits(compute_cluster_bounds_with_positions(
                &indices, cached, &mut new
            )),
        );
        assert_eq!(old.usage(), new.usage());
        assert_eq!(
            bits(compute_meshlet_bounds(
                &vertices, &triangles, positions, &mut old
            )),
            bits(compute_meshlet_bounds_with_positions(
                &vertices, &triangles, cached, &mut new
            )),
        );
        assert_eq!(old.usage(), new.usage());
    }
});
