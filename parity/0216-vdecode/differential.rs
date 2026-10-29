use std::io::{Read,Write};
use meshoptimizer_rs::{codec::*,Workspace};
fn main() {
    let level=match std::env::args().nth(1).as_deref() {
        Some("scalar")=>Level::Scalar,Some("sse2")=>Level::Sse2,Some("ssse3")=>Level::Ssse3,
        Some("sse41")=>Level::Sse41,Some("neon")=>Level::Neon,Some("wasm")=>Level::Wasm,_=>detected_level()
    };
    if with_level(level,||()).is_err() {std::process::exit(77);}
    let mut input=std::io::stdin().lock();let mut output=std::io::stdout().lock();let mut h=[0;4];
    while input.read_exact(&mut h).is_ok() {
        let mut b=vec![0;u32::from_le_bytes(h) as usize];input.read_exact(&mut b).unwrap();
        let word=|j|u32::from_le_bytes(b[j..j+4].try_into().unwrap()) as usize;
        let (count,stride,len)=(word(0),word(4),word(8));assert_eq!(len,b.len()-12);
        let mut out=vec![0xcc;count*stride+7];
        let result=with_level(level,||decode_vertex_buffer_into(&mut out,count,stride,&b[12..],&mut Workspace::default())).unwrap();
        output.write_all(&((out.len()+4) as u32).to_le_bytes()).unwrap();
        output.write_all(&(u32::from(result.is_err())).to_le_bytes()).unwrap();
        output.write_all(&out).unwrap();output.flush().unwrap();
    }
}
