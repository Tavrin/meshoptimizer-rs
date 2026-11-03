use meshoptimizer_rs as m;
use std::{
    hint::black_box,
    io::{self, BufRead, Read, Write},
    time::Instant,
};
fn read(path: &str) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut f = std::fs::File::open(path).unwrap();
    fn word(f: &mut impl Read) -> u32 {
        let mut b = [0; 4];
        f.read_exact(&mut b).unwrap();
        u32::from_le_bytes(b)
    }
    let n = usize::try_from(word(&mut f)).unwrap();
    let ni = usize::try_from(word(&mut f)).unwrap();
    let p = (0..n)
        .map(|_| std::array::from_fn(|_| f32::from_bits(word(&mut f))))
        .collect();
    let i = (0..ni).map(|_| word(&mut f)).collect();
    (p, i)
}
fn run(op: &str, p: &[[f32; 3]], i: &[u32], w: &mut m::Workspace) -> (Vec<u32>, String) {
    if op == "simplify" {
        let r = m::simplify(
            i,
            m::Positions::from_packed(p),
            m::SimplifySettings {
                target_index_count: i.len() / 6 * 3,
                target_error: 0.5,
                options: m::SimplifyOptions::EMPTY,
            },
            w,
        )
        .unwrap();
        return (r.indices, String::from("{}"));
    }
    let mut o = m::HiddenOptions::default();
    if let Ok(s) = std::env::var("HSR_SUBDIVISIONS") {
        o.subdivisions = s.parse().unwrap();
    }
    if let Ok(s) = std::env::var("HSR_PIXEL_SIZE") {
        o.pixel_size = s.parse().unwrap();
    }
    let r = m::remove_hidden_triangles(i, m::Positions::from_packed(p), o, w).unwrap();
    let report=format!("{{\"input\":{},\"output\":{},\"removed\":{},\"duplicate\":{},\"back_to_back\":{},\"occluded\":{},\"work\":{},\"bytes\":{}}}",r.report.input_triangles,r.report.output_triangles,r.report.removed(),r.report.duplicate,r.report.enclosed_back_to_back,r.report.occluded,w.usage().work,w.usage().bytes);
    (r.indices, report)
}
fn control(command: &str) {
    let Ok(path) = std::env::var("HSR_CTL") else {
        return;
    };
    let mut ctl = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    writeln!(ctl, "{command}").unwrap();
    ctl.flush().unwrap();
    let mut ack =
        io::BufReader::new(std::fs::File::open(std::env::var("HSR_ACK").unwrap()).unwrap());
    let mut line = String::new();
    ack.read_line(&mut line).unwrap();
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (p, i) = read(&a[2]);
    let mut w = m::Workspace::new(m::Limits {
        max_bytes: 3 << 30,
        ..m::Limits::default()
    });
    let start = Instant::now();
    let (out, report) = run(&a[1], &p, &i, &mut w);
    let elapsed = start.elapsed().as_secs_f64();
    let raw: Vec<u8> = out.iter().flat_map(|v| v.to_le_bytes()).collect();
    std::fs::write(&a[3], raw).unwrap();
    println!("RESULT {elapsed:.9} {report}");
    println!("READY");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            break;
        }
        let n: usize = line.parse().unwrap();
        control("enable");
        let start = Instant::now();
        for _ in 0..n {
            black_box(run(black_box(&a[1]), black_box(&p), black_box(&i), &mut w));
        }
        let elapsed = start.elapsed().as_secs_f64();
        control("disable");
        println!("COUNTED {elapsed:.9}");
        io::stdout().flush().unwrap();
    }
}
