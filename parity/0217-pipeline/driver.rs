#[path = "../../tests/pipeline.rs"]
mod support;
extern crate candidate as meshoptimizer_rs;
use meshoptimizer_rs::*;
use std::{hint::black_box, time::Instant};
fn cpu_ticks() -> u64 {
    let stat = std::fs::read_to_string("/proc/self/stat").unwrap();
    let fields: Vec<_> = stat
        .rsplit_once(')')
        .unwrap()
        .1
        .split_whitespace()
        .collect();
    fields[11]
        .parse::<u64>()
        .unwrap()
        .checked_add(fields[12].parse::<u64>().unwrap())
        .unwrap()
}
fn main() {
    let mut cases = Vec::new();
    let quiet = std::env::var_os("PIPELINE_QUIET").is_some();
    let hz: u128 = std::env::var("PIPELINE_HZ")
        .unwrap_or_else(|_| "100".into())
        .parse()
        .unwrap();
    let supplement = std::env::var_os("PIPELINE_SUPPLEMENT").is_some();
    for n in [4, 16, 32] {
        let (i, v) = support::grid(n);
        cases.push((format!("grid-{n}"), i, v));
    }
    for path in std::env::args().skip(1) {
        let data = std::fs::read(&path).unwrap();
        let word = |a| u32::from_le_bytes(data[a..a + 4].try_into().unwrap());
        let n = usize::try_from(word(0)).unwrap();
        let ni = usize::try_from(word(4)).unwrap();
        let v = data[8..8 + n * 12].to_vec();
        let i = (0..ni).map(|k| word(8 + n * 12 + k * 4)).collect();
        cases.push((
            std::path::Path::new(&path)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            i,
            v,
        ));
    }
    for (name, i, v) in cases {
        let mut o = ProcessOptions::default();
        o.quality.samples_per_unit_area = 0.;
        let calibration_start = Instant::now();
        support::equivalent(&i, &v, 12, &o);
        let mut estimated_ns = calibration_start.elapsed().as_nanos() / 2;
        if quiet && estimated_ns < 5_000_000 {
            let start = Instant::now();
            for _ in 0..128 {
                black_box(
                    process_mesh(&i, &v, 12, 0, &o, &mut support::qualification_workspace())
                        .unwrap(),
                );
            }
            estimated_ns = start.elapsed().as_nanos() / 128;
        }
        println!(
            "equivalent {name} indices={} vertices={}",
            i.len(),
            v.len() / 12
        );
        // Identical independent workspace per invocation; both produce quality and outputs.
        let mut pipeline = Vec::new();
        let mut manual = Vec::new();
        let repetitions = if i.len() > 100_000 {
            if quiet {
                7
            } else if supplement {
                4
            } else {
                3
            }
        } else {
            7
        };
        let batch = if quiet {
            assert!(estimated_ns > 0, "calibration clock did not resolve");
            9_000_000_000u128.div_ceil(estimated_ns)
        } else if !supplement {
            1
        } else if i.len() <= 100 {
            10_000
        } else if i.len() <= 2_000 {
            256
        } else if i.len() <= 7_000 {
            64
        } else if i.len() <= 25_000 {
            16
        } else {
            1
        };
        if quiet {
            println!("calibration {name} estimated_ns={estimated_ns} batch={batch}");
        }
        for rep in 0..repetitions {
            for arm in if rep % 2 == 0 { [0, 1] } else { [1, 0] } {
                let ticks_before = if quiet { cpu_ticks() } else { 0 };
                let start = Instant::now();
                for _ in 0..batch {
                    if arm == 0 {
                        black_box(
                            process_mesh(
                                &i,
                                &v,
                                12,
                                0,
                                &o,
                                &mut support::qualification_workspace(),
                            )
                            .unwrap(),
                        );
                    } else {
                        black_box(support::manual(&i, &v, 12, &o));
                    }
                }
                let ns = start.elapsed().as_nanos();
                let ticks = if quiet {
                    cpu_ticks().checked_sub(ticks_before).unwrap()
                } else {
                    0
                };
                if quiet {
                    let lower_cpu_ns =
                        (u128::from(ticks) * 1_000_000_000 / hz).saturating_sub(2_000_000_000 / hz);
                    let qualified = ns >= 5_000_000_000
                        && lower_cpu_ns * 100 >= ns.saturating_add(1_000_000) * 99;
                    println!("cpu {name} repetition={rep} arm={arm} ticks={ticks} hz={hz} wall_ns={ns} qualified={qualified}");
                    if !qualified {
                        eprintln!("REJECT: wall duration or CPU occupancy did not qualify");
                        std::process::exit(2);
                    }
                }
                println!("sample {name} repetition={rep} arm={arm} ns={ns} batch={batch}");
                if arm == 0 {
                    pipeline.push(ns)
                } else {
                    manual.push(ns)
                }
            }
        }
        pipeline.sort();
        manual.sort();
        println!(
            "timing {name} pipeline_ns={} manual_ns={} ratio={:.6} samples={repetitions} batch={batch}",
            pipeline[repetitions / 2] / batch,
            manual[repetitions / 2] / batch,
            pipeline[repetitions / 2] as f64 / manual[repetitions / 2] as f64
        );
    }
}
