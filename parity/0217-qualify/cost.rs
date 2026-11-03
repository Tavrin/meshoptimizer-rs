//! Cost disclosure: process_mesh against the manual public-API sequence of tests/pipeline.rs.
//! Setup, input loading and serialization are outside the counted region; allocation and drop are inside.
#[path = "../../tests/pipeline.rs"]
#[allow(dead_code)]
mod support;
use meshoptimizer_rs::*;
use std::{
    hint::black_box,
    io::{self, BufRead, Read, Write},
};

fn control(cmd: &str) {
    if std::env::var_os("PROBE_CTL").is_none() {
        return;
    }
    let mut c = std::fs::OpenOptions::new()
        .write(true)
        .open(format!("/proc/self/fd/{}", std::env::var("PROBE_CTL").unwrap()))
        .unwrap();
    let mut a = std::fs::File::open(format!("/proc/self/fd/{}", std::env::var("PROBE_ACK").unwrap())).unwrap();
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
    let args: Vec<_> = std::env::args().collect();
    let raw = std::fs::read(&args[2]).unwrap();
    let words: Vec<u32> = raw.chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
    let (nv, ni) = (words[0] as usize, words[1] as usize);
    let v: Vec<u8> = words[2..2 + nv * 3].iter().flat_map(|w| w.to_le_bytes()).collect();
    let i = words[2 + nv * 3..2 + nv * 3 + ni].to_vec();
    drop(raw);
    drop(words);
    let mut o = ProcessOptions::default();
    o.quality.samples_per_unit_area = 0.;
    support::equivalent(&i, &v, 12, &o);
    let op = args[1].clone();
    let call = |op: &str| match op {
        "process" => {
            black_box(process_mesh(&i, &v, 12, 0, &o, &mut support::qualification_workspace()).unwrap());
        }
        "manual" => {
            black_box(support::manual(&i, &v, 12, &o));
        }
        _ => panic!("unknown operation"),
    };
    std::fs::write(&args[3], format!("{op}").as_bytes()).unwrap();
    println!("READY");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            break;
        }
        let n: usize = line.parse().unwrap();
        control("enable");
        for _ in 0..n {
            call(black_box(&op));
        }
        control("disable");
        println!("COUNTED 0.0");
        io::stdout().flush().unwrap();
    }
}
