// Scratch benchmark entry point. moss.rs is an exact pinned copy, with only
// private decoder/filter functions made visible to this sibling module.
mod moss;
use std::io::{Read, Write};
fn word(b: &[u8], i: usize) -> u32 { u32::from_le_bytes(b[i..i+4].try_into().unwrap()) }
fn main() {
    let mut input=std::io::stdin().lock(); let mut output=std::io::stdout().lock();
    loop {
        let mut size=[0;4]; if input.read_exact(&mut size).is_err() {break;}
        let mut b=vec![0;u32::from_le_bytes(size) as usize]; input.read_exact(&mut b).unwrap();
        let op=word(&b,4);let count=word(&b,8) as usize;let stride=word(&b,12) as usize;
        let mode=word(&b,16);let filter=word(&b,20);let samples=word(&b,32);let iterations=word(&b,36);
        let src=&b[44..];
        let once=|| -> Result<Vec<u8>,String> {
            match op {
                1=>moss::decode_vertex_buffer(count,stride,src),
                2=>moss::decode_index_buffer(count,stride,src),
                3=>moss::decode_index_sequence(count,stride,src),
                4..=6=>{let mut v=src.to_vec();match op {4=>moss::filter_octahedral(&mut v,stride)?,5=>moss::filter_quaternion(&mut v,stride)?,_=>moss::filter_exponential(&mut v,stride)?};Ok(v)},
                7=>moss::decode([moss::Mode::Attributes,moss::Mode::Triangles,moss::Mode::Indices][mode as usize],
                    [moss::Filter::None,moss::Filter::Octahedral,moss::Filter::Quaternion,moss::Filter::Exponential][filter as usize],count,stride,src),
                _=>Err("operation".into())
            }
        };
        let mut result=once();let mut times=Vec::new();
        for _ in 0..samples {let start=std::time::Instant::now();for _ in 0..iterations {result=std::hint::black_box(once());}times.push(start.elapsed().as_secs_f64()/f64::from(iterations));}
        let (status,bytes)=match result {Ok(v)=>(0i32,v),Err(_)=>(-1i32,Vec::new())};
        let mut out=b"MD02".to_vec();out.extend_from_slice(&status.to_le_bytes());out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());out.extend_from_slice(&(times.len() as u32).to_le_bytes());out.extend_from_slice(&bytes);
        for t in times {out.extend_from_slice(&t.to_le_bytes());}
        output.write_all(&(out.len() as u32).to_le_bytes()).unwrap();output.write_all(&out).unwrap();output.flush().unwrap();
    }
}
