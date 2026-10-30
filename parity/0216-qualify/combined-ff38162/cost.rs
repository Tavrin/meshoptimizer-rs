//! Owned-call costs on the frozen corpus; setup/serialization outside counters.
use meshoptimizer_rs as m;
use std::{hint::black_box, io::{self, BufRead, Read, Write}};

struct Input { p:Vec<[f32;3]>, i:Vec<u32>, sequence:Vec<u8> }
enum Output { Bytes(Vec<u8>), Indices(Vec<u32>), Weld(m::WeldedVertices), Orient(m::OrientedMesh), Fill(m::FilledMesh) }
impl Output {
    fn bytes(self)->Vec<u8> {
        match self {
            Self::Bytes(b)=>b,
            Self::Indices(i)=>i.iter().flat_map(|i|i.to_le_bytes()).collect(),
            Self::Weld(v)=>format!("{v:?}").into_bytes(),
            Self::Orient(v)=>format!("{v:?}").into_bytes(),
            Self::Fill(v)=>format!("{v:?}").into_bytes(),
        }
    }
}
fn call(op:&str,x:&Input,w:&mut m::Workspace)->Output {
    let p=m::Positions::from_packed(&x.p);
    match op {
        "sanitize"=>Output::Indices(m::sanitize(&x.i,p,m::SanitizeOptions::EMPTY,w).unwrap().indices),
        "weld"=>Output::Weld(m::weld_vertices(p,0.,m::WeldPolicy::PositionsOnly,w).unwrap()),
        "orient"=>Output::Orient(m::orient_faces(&x.i,x.p.len(),w).unwrap()),
        "fill"=>Output::Fill(m::fill_holes(&x.i,p,16,w).unwrap()),
        "sequence-one-shot"=>Output::Bytes(m::codec::decode_index_sequence(x.i.len(),4,&x.sequence,w).unwrap()),
        "sequence-stream"=>{
            let mut d=m::codec::IndexSequenceDecoder::new(x.i.len(),m::Limits{max_bytes:0,max_work:u64::MAX}).unwrap();
            let mut out=vec![0;x.i.len()];let mut read=0;let mut written=0;
            while read<x.sequence.len() {
                let end=(read+4096).min(x.sequence.len());
                let result=d.push(&x.sequence[read..end],&mut out[written..]).unwrap();
                assert!(result.consumed!=0 || result.written!=0);
                read+=result.consumed;written+=result.written;
            }
            d.finish().unwrap();assert_eq!(written,out.len());Output::Indices(out)
        }
        _=>panic!("unknown operation"),
    }
}
fn control(cmd:&str) {
    if std::env::var_os("PROBE_CTL").is_none(){return}
    let mut c=std::fs::OpenOptions::new().write(true).open(format!("/proc/self/fd/{}",std::env::var("PROBE_CTL").unwrap())).unwrap();
    let mut a=std::fs::File::open(format!("/proc/self/fd/{}",std::env::var("PROBE_ACK").unwrap())).unwrap();
    c.write_all(format!("{cmd}\n").as_bytes()).unwrap();c.flush().unwrap();
    let mut b=[0];loop{a.read_exact(&mut b).unwrap();if b[0]==b'\n'{break}}
}
fn main() {
    let args:Vec<_>=std::env::args().collect();let raw=std::fs::read(&args[2]).unwrap();
    let words:Vec<u32>=raw.chunks_exact(4).map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
    let (nv,ni)=(words[0] as usize,words[1] as usize);
    let p=words[2..2+nv*3].chunks_exact(3).map(|v|[f32::from_bits(v[0]),f32::from_bits(v[1]),f32::from_bits(v[2])]).collect();
    let i=words[2+nv*3..2+nv*3+ni].to_vec();drop(raw);drop(words);
    let mut w=m::Workspace::new(m::Limits{max_work:u64::MAX,max_bytes:usize::MAX});
    let sequence=m::codec::encode_index_sequence(&i,m::codec::IndexEncoding::DEFAULT,&mut w).unwrap();
    let x=Input{p,i,sequence};let op=&args[1];
    if op.starts_with("sequence") {
        assert_eq!(call("sequence-one-shot",&x,&mut w).bytes(),call("sequence-stream",&x,&mut w).bytes());
    }
    std::fs::write(&args[3],call(op,&x,&mut w).bytes()).unwrap();
    println!("READY");io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let line=line.unwrap();if line=="stop"{break}let n:usize=line.parse().unwrap();
        control("enable");for _ in 0..n{black_box(call(black_box(op),black_box(&x),&mut w));}
        control("disable");println!("COUNTED 0.0");io::stdout().flush().unwrap();
    }
}
