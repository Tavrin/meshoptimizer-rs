//! Review probe E4, alone in its test binary so the process peak memory is
//! this test's: `read_all` scratch is linear in the file length.
#![cfg(feature = "clusterlod")]
#[path = "support/mdag2_craft.rs"]
mod craft;

use meshoptimizer_rs::clusterlod::dag_format::v2;
use meshoptimizer_rs::Workspace;

/// Resident set size and its peak, in KiB, from `/proc/self/status` (Linux).
fn memory_kib() -> Option<(u64, u64)> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let field = |name: &str| -> Option<u64> {
        let line = status.lines().find(|l| l.starts_with(name))?;
        line.split_whitespace().nth(1)?.parse().ok()
    };
    Some((field("VmRSS:")?, field("VmHWM:")?))
}

/// A valid 18 MiB file whose last page holds 15,000 child groups of one
/// parent spanning 100,000 pages. Before the fix `read_all` needed about 6 GB
/// of scratch (and returned `Allocation` under a 4 GB cap); now it needs
/// `open`'s linear scratch plus one page.
#[test]
fn read_all_scratch_is_linear_in_the_file() {
    let file = craft::e4(15_000, 100_000);
    assert!(file.len() < 18 << 20);
    let before = memory_kib();
    let start = std::time::Instant::now();
    let r = v2::Reader::read_all(&file, &mut Workspace::default()).unwrap();
    let elapsed = start.elapsed();
    assert_eq!(r.mesh().page_count, 100_001);
    let Some(((rss, _), (_, peak))) = before.zip(memory_kib()) else {
        return;
    };
    let scratch = peak.saturating_sub(rss);
    println!(
        "E4 read_all: {elapsed:?}, file {} KiB, peak RSS {peak} KiB, at most {scratch} KiB above the RSS before",
        file.len() >> 10
    );
    assert!(scratch < 256 << 10, "{scratch} KiB");
}
