#![no_main]
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let m = mesh(&mut c, 128);
    let settings = SimplifySettings {
        target_index_count: m.idx.len() * usize::from(c.u8()) / 255,
        target_error: c.unit() * c.unit(),
        options: SimplifyOptions::from_bits(u32::from(c.u8()) | u32::from(c.u8() & 1) << 8)
            .unwrap_or(SimplifyOptions::EMPTY),
    };
    let sloppy_target = m.idx.len() * usize::from(c.u8()) / 255;
    let big = Limits {
        max_bytes: 256 << 20,
        max_work: 1 << 30,
    };
    let p = || Positions::from_packed(&m.pos);
    type Run<'a> = Box<dyn Fn(&mut Workspace) -> Result<String, Error> + 'a>;
    let runs: Vec<Run<'_>> = vec![
        Box::new(|w| simplify(&m.idx, p(), settings, w).map(|r| format!("{r:?}"))),
        Box::new(|w| {
            simplify_sloppy(&m.idx, p(), None, sloppy_target, settings.target_error, w)
                .map(|r| format!("{r:?}"))
        }),
        Box::new(|w| simplify_prune(&m.idx, p(), settings.target_error, w).map(|r| format!("{r:?}"))),
    ];
    for run in runs {
        // Unbudgeted reference.
        let mut ws = Workspace::new(big);
        let reference = run(&mut ws);
        let usage = ws.usage();
        // Budgeted at exactly the counted work: identical result and usage.
        let tight = Limits {
            max_bytes: big.max_bytes,
            max_work: usage.work,
        };
        let mut tw = Workspace::new(tight);
        let tight_result = run(&mut tw);
        assert_eq!(reference, tight_result, "work limit equal to usage");
        assert_eq!(usage.work, tw.usage().work);
        // Reused workspace, same input: same result.
        assert_eq!(reference, run(&mut ws));
        // One unit less must fail with the limit error (or the same earlier error).
        if reference.is_ok() && usage.work > 0 {
            let less = Limits {
                max_work: usage.work - 1,
                ..tight
            };
            let r = run(&mut Workspace::new(less));
            assert_eq!(r.err(), Some(Error::LimitExceeded), "work limit one below usage");
        }
    }
});
