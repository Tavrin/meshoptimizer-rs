#[path = "../../tests/pipeline.rs"]
mod support;
extern crate candidate as meshoptimizer_rs;
use meshoptimizer_rs::*;
use std::{hint::black_box, time::Instant};
fn main() {
    let mut cases = Vec::new();
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
        support::equivalent(&i, &v, 12, &o);
        println!(
            "equivalent {name} indices={} vertices={}",
            i.len(),
            v.len() / 12
        );
        // Identical independent workspace per invocation; both produce quality and outputs.
        let mut pipeline = Vec::new();
        let mut manual = Vec::new();
        let repetitions = if i.len() > 100_000 {
            if supplement {
                4
            } else {
                3
            }
        } else {
            7
        };
        let batch = if !supplement {
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
        for rep in 0..repetitions {
            for arm in if rep % 2 == 0 { [0, 1] } else { [1, 0] } {
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
