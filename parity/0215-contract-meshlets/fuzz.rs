//! Deterministic mutation stream for the p03 meshlet fuzz entry points.
use meshopt_p03_fuzz::exercise;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut state: u64 = args[1].parse().unwrap();
    let count: usize = args[2].parse().unwrap();
    for case in 0..count {
        let mut data = [0u8; 2048];
        for byte in &mut data {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *byte = state as u8;
        }
        if case % 2 == 0 {
            data[2] |= 128;
        } else {
            data[2] &= 127;
        }
        exercise(if case % 4 < 2 { 1 } else { 3 }, &data);
    }
}
