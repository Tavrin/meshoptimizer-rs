//! Native driver: prints `<case> <hash>` for every determinism case, each run twice (repeat equality is asserted).
use determinism::{run_case, CASES};

fn main() {
    for (i, name) in CASES.iter().enumerate() {
        let first = run_case(i as u32);
        let second = run_case(i as u32);
        assert_eq!(first, second, "{name} differs between identical calls");
        println!("{name} {first:016x}");
    }
}
