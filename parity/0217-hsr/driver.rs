//! Isolated cache regression/performance driver and hidden-surface disclosure.
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
#[cfg(feature = "ours")]
type Work = m::Workspace;
#[cfg(feature = "cpp")]
type Work = ();
fn run(op: &str, p: &[[f32; 3]], i: &[u32], w: &mut Work) -> (Vec<u32>, String) {
    #[cfg(feature = "ours")]
    if op == "cache" {
        return (
            m::optimize_vertex_cache(i, p.len(), w).unwrap(),
            String::new(),
        );
    }
    #[cfg(feature = "cpp")]
    {
        let _ = (op, w);
        (meshopt::optimize_vertex_cache(i, p.len()), String::new())
    }
    #[cfg(feature = "hidden")]
    {
        let mut options = m::HiddenOptions::default();
        if let Ok(s) = std::env::var("HSR_SUBDIVISIONS") {
            options.subdivisions = s.parse().unwrap();
        }
        let r = m::remove_hidden_triangles(i, m::Positions::from_packed(p), options, w).unwrap();
        let report=format!("{{\"input\":{},\"output\":{},\"removed\":{},\"duplicate\":{},\"back_to_back\":{},\"occluded\":{},\"work\":{},\"bytes\":{}}}",r.report.input_triangles,r.report.output_triangles,r.report.removed(),r.report.duplicate,r.report.enclosed_back_to_back,r.report.occluded,w.usage().work,w.usage().bytes);
        (r.indices, report)
    }
    #[cfg(all(feature = "ours", not(feature = "hidden")))]
    panic!("unknown operation {op}")
}
fn control(command: &str) {
    let Ok(path) = std::env::var("HSR_CTL") else {
        return;
    };
    let mut ctl = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    writeln!(ctl, "{command}").unwrap();
    ctl.flush().unwrap();
    let ack = std::fs::File::open(std::env::var("HSR_ACK").unwrap()).unwrap();
    let mut ack = io::BufReader::new(ack);
    let mut line = String::new();
    ack.read_line(&mut line).unwrap();
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (p, i) = read(&a[2]);
    let mut w = Work::default();
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
