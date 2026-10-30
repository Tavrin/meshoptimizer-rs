use candidate as m;
use std::{
    io::Write,
    process::{Command, Stdio},
};
fn main() {
    let mut seed = 21709u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) as u32 % 200) as f32 / 32. - 3.
    };
    let mut input = String::new();
    let mut expected = Vec::new();
    for case in 0..256 {
        let mut a: Vec<[f32; 3]> = (0..(case % 9 + 1) * 3)
            .map(|_| [next(), next(), next()])
            .collect();
        let mut b: Vec<[f32; 3]> = (0..(case % 7 + 1) * 3)
            .map(|_| [next(), next(), next()])
            .collect();
        if case % 5 == 0 {
            for p in a.iter_mut().chain(&mut b) {
                p[2] = 0.;
            }
        }
        if case % 13 == 0 {
            for p in a.iter_mut().chain(&mut b) {
                *p = [p[0], 0., 0.];
            }
        }
        if case % 17 == 0 {
            for p in a.iter_mut().chain(&mut b) {
                *p = [1., 2., 3.];
            }
        }
        let ai: Vec<u32> = (0..u32::try_from(a.len()).unwrap()).collect();
        let bi: Vec<u32> = (0..u32::try_from(b.len()).unwrap()).collect();
        let mut options = m::LodQualityOptions::default();
        options.samples_per_unit_area = if case % 3 == 0 { 0. } else { 2. };
        input += &format!(
            "{} {} {}\n",
            a.len() / 3,
            b.len() / 3,
            options.samples_per_unit_area
        );
        for p in a.iter().chain(&b) {
            for v in p {
                input += &format!("{v} ");
            }
            input += '\n'.to_string().as_str();
        }
        let mut w = m::Workspace::default();
        let q = m::measure_lod_quality(
            m::LodQualityMesh::new(&ai, m::Positions::from_packed(&a)),
            m::LodQualityMesh::new(&bi, m::Positions::from_packed(&b)),
            &options,
            &mut w,
        )
        .unwrap();
        expected.push([q.source_to_level, q.level_to_source]);
    }
    let mut child = Command::new(std::env::args().nth(1).unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()).unwrap());
    let out = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<_> = text.lines().collect();
    assert_eq!(lines.len(), expected.len());
    for (case, (line, q)) in lines.iter().zip(expected).enumerate() {
        let v: Vec<f64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(v.len(), 6);
        for side in 0..2 {
            for (a, b) in [(v[side * 3], q[side].max), (v[side * 3 + 1], q[side].rms)] {
                assert!((a - b).abs() < 1e-7, "case {case} {a} {b}");
            }
            assert_eq!(v[side * 3 + 2], q[side].samples as f64);
        }
    }
    println!("256 exhaustive C++ oracle cases; 1024 distance statistics and 512 sample counts; differences 0 (tolerance 1e-7)");
}
