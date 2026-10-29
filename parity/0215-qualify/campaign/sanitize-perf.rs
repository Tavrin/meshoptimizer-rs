//! Sanitize / analyze_mesh cost disclosure against filter_index_buffer on the
//! frozen inputs, with the peers counting protocol (READY, then per stdin line
//! N: enable counters, N calls, disable, COUNTED; `stop` ends).
//! Usage: sanitize-perf OP INPUT OUTPUT with OP in filter, sanitize,
//! sanitize-weld, sanitize-full, analyze, analyze-weld.
use meshoptimizer_rs as m;
use std::hint::black_box;
use std::io::{self, BufRead, Read, Write};

fn words(b: &[u8]) -> Vec<u32> {
    b.chunks_exact(4).map(|v| u32::from_le_bytes(v.try_into().unwrap())).collect()
}

fn run(op: &str, p: &[[f32; 3]], raw: &[u8], i: &[u32], w: &mut m::Workspace) -> Vec<u8> {
    let pos = m::Positions::from_packed(p);
    let sanitize = |options: m::SanitizeOptions, w: &mut m::Workspace| {
        let r = m::sanitize(i, pos, options, w).unwrap();
        format!("{:?}", r.report).into_bytes().into_iter().chain(r.indices.iter().flat_map(|v| v.to_le_bytes())).collect()
    };
    match op {
        "filter" => {
            let s = m::VertexStream::new(raw, p.len(), 12, 12, 0).unwrap();
            m::filter_index_buffer(i, s, w).unwrap().iter().flat_map(|v| v.to_le_bytes()).collect()
        }
        "sanitize" => sanitize(m::SanitizeOptions::EMPTY, w),
        "sanitize-weld" => sanitize(m::SanitizeOptions::WELD_POSITIONS, w),
        "sanitize-full" => sanitize(
            m::SanitizeOptions::from_bits(
                m::SanitizeOptions::COINCIDENT_CORNERS.bits() | m::SanitizeOptions::WELD_POSITIONS.bits(),
            )
            .unwrap(),
            w,
        ),
        "analyze" => format!("{:?}", m::analyze_mesh(i, pos, m::AnalyzeOptions::EMPTY, w).unwrap()).into_bytes(),
        "analyze-weld" => format!(
            "{:?}",
            m::analyze_mesh(i, pos, m::AnalyzeOptions::EMPTY.with_weld_positions(true), w).unwrap()
        )
        .into_bytes(),
        _ => panic!("unknown op {op}"),
    }
}

fn control(cmd: &str) {
    if std::env::var_os("PROBE_CTL").is_none() {
        return;
    }
    // The parent passes both pipe ends open; reopen them through /proc so
    // this driver needs no unsafe code.
    let ctl = std::env::var("PROBE_CTL").unwrap();
    let ack = std::env::var("PROBE_ACK").unwrap();
    let mut c = std::fs::OpenOptions::new()
        .write(true)
        .open(format!("/proc/self/fd/{ctl}"))
        .unwrap();
    let mut a = std::fs::File::open(format!("/proc/self/fd/{ack}")).unwrap();
    c.write_all(format!("{cmd}\n").as_bytes()).unwrap();
    c.flush().unwrap();
    let mut b = [0];
    loop {
        a.read_exact(&mut b).unwrap();
        if b[0] == b'\n' {
            break;
        }
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let raw = std::fs::read(&a[2]).unwrap();
    let n = u32::from_le_bytes(raw[0..4].try_into().unwrap()) as usize;
    let ni = u32::from_le_bytes(raw[4..8].try_into().unwrap()) as usize;
    let p: Vec<[f32; 3]> = words(&raw[8..8 + n * 12])
        .chunks_exact(3)
        .map(|v| [f32::from_bits(v[0]), f32::from_bits(v[1]), f32::from_bits(v[2])])
        .collect();
    let i = words(&raw[8 + n * 12..8 + n * 12 + ni * 4]);
    let mut w = m::Workspace::new(m::Limits {
        max_bytes: 3usize << 30,
        ..m::Limits::default()
    });
    let bytes = raw[8..8 + n * 12].to_vec();
    std::fs::write(&a[3], run(&a[1], &p, &bytes, &i, &mut w)).unwrap();
    println!("READY");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            break;
        }
        let count: usize = line.parse().unwrap();
        control("enable");
        for _ in 0..count {
            black_box(run(black_box(&a[1]), black_box(&p), black_box(&bytes), black_box(&i), &mut w));
        }
        control("disable");
        println!("COUNTED");
        io::stdout().flush().unwrap();
    }
}
