//! Native driver: prints `<case> <hash>` for every determinism case, each run twice (repeat equality is asserted).
use determinism::{run_case, CASES};

fn main() {
    for (i, name) in CASES.iter().enumerate() {
        let first = run_case(i as u32);
        let second = run_case(i as u32);
        #[cfg(feature = "parallel")]
        for workers in [1, 2, 4, 8, 16] {
            let got = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(|| run_case(i as u32));
            assert_eq!(first, got, "threads={workers} case={name}");
        }
        assert_eq!(first, second, "{name} differs between identical calls");
        println!("{name} {first:016x}");
    }
}
