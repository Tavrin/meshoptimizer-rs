//! Property fuzzing of the actual production quotient over full-width integers.
#[path = "../../src/meshlet_division.rs"]
mod division;
use division::Divisor;
use std::hint::black_box;
fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let seed: u64 = args[1].parse().unwrap();
    let count: u64 = args[2].parse().unwrap();
    let mut rng = seed;
    let mut checksum = 0usize;
    for i in 0..count {
        let d = (next(&mut rng) % 512 + 1) as usize;
        let bits = next(&mut rng) as usize;
        let n = match i % 8 {
            0 => usize::MAX,
            1 => usize::MAX - (bits & 1023),
            2 => bits & 0xffff,
            3 => 0,
            _ => bits,
        };
        let actual = Divisor::new(black_box(d)).quotient(black_box(n));
        assert_eq!(actual, n / d, "seed={seed} case={i} n={n} d={d}");
        checksum ^= actual;
    }
    println!("{{\"seed\":{seed},\"cases\":{count},\"checksum\":{checksum}}}");
}
