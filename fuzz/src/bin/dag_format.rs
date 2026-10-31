//! Deterministic reader mutation campaign. Runtime is process CPU time on Linux.
use meshoptimizer_rs::{clusterlod::dag_format::Reader, Workspace};
use std::{fs, time::Instant};
fn cpu() -> f64 {
    let s = fs::read_to_string("/proc/self/stat").unwrap();
    let fields: Vec<_> = s[s.rfind(')').unwrap() + 2..].split_whitespace().collect();
    (fields[11].parse::<f64>().unwrap() + fields[12].parse::<f64>().unwrap()) / 100.
}
fn main() {
    let seconds: f64 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "600".into())
        .parse()
        .unwrap();
    let mut paths: Vec<_> = fs::read_dir("fuzz/seeds/dag_format")
        .unwrap()
        .map(|p| p.unwrap().path())
        .collect();
    paths.sort();
    let seeds: Vec<_> = paths.into_iter().map(|p| fs::read(p).unwrap()).collect();
    let start = cpu();
    let wall = Instant::now();
    let mut rng = 217u64;
    let mut count = 0u64;
    let mut accepted = 0u64;
    let mut ws = Workspace::default();
    while cpu() - start < seconds {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let mut b = seeds[(rng as usize) % seeds.len()].clone();
        for _ in 0..1 + rng % 8 {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            let at = rng as usize % b.len();
            b[at] ^= (rng >> 32) as u8;
            if rng % 8 == 0 {
                b.truncate(at + 1);
            }
        }
        if let Ok(r) = Reader::read(&b, &mut ws) {
            accepted += 1;
            for p in 0..r.page_count() {
                r.decode_page(p, &b[r.page_range(p).unwrap()], &mut ws)
                    .unwrap();
            }
        }
        count += 1;
    }
    println!(
        "seed=217 executions={count} accepted={accepted} cpu_seconds={} wall_seconds={} crashes=0",
        cpu() - start,
        wall.elapsed().as_secs_f64()
    );
}
