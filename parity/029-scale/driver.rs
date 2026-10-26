//! Resident, nonallocating scale-stage instruction driver. Preparation is untimed.
#[cfg(feature = "base-arm")]
use baseline as m;
#[cfg(not(feature = "base-arm"))]
use candidate as m;
use std::{
    hint::black_box,
    io::{self, BufRead, Write},
};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&a[1]).unwrap();
    let stride: usize = a[2].parse().unwrap();
    let n = data.len() / stride;
    assert_eq!(n * stride, data.len());
    let packed: Vec<[f32; 3]> = if stride == 12 {
        data.chunks_exact(12)
            .map(|r| {
                std::array::from_fn(|j| f32::from_le_bytes(r[j * 4..j * 4 + 4].try_into().unwrap()))
            })
            .collect()
    } else {
        vec![]
    };
    let p = if stride == 12 {
        m::Positions::from_packed(&packed)
    } else {
        m::Positions::from_bytes(&data, n, stride, 0, m::ByteOrder::LittleEndian).unwrap()
    };
    let bits = m::simplify_scale(black_box(p)).unwrap().to_bits();
    println!("READY {bits}");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            break;
        }
        let count: usize = line.parse().unwrap();
        for _ in 0..count {
            black_box(m::simplify_scale(black_box(p)).unwrap().to_bits());
        }
        println!("COUNTED");
        io::stdout().flush().unwrap();
    }
}
