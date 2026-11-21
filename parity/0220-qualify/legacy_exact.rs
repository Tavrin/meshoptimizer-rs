#[path = "legacy-candidate.rs"]
mod after;
#[path = "legacy-baseline.rs"]
mod before;
#[path = "mdag1-candidate.rs"]
mod md_after;
#[path = "mdag1-baseline.rs"]
mod md_before;
fn main() {
    for (i, n) in before::CASES.iter().enumerate() {
        let a = before::run_case(i as u32);
        let b = after::run_case(i as u32);
        assert_eq!(a, b, "{n}");
        println!("{n} {a:016x} {b:016x} equal");
    }
    let a = md_before::run();
    let b = md_after::run();
    assert_eq!(a, b);
    println!("MDAG1 {} bytes equal", a.len());
}
