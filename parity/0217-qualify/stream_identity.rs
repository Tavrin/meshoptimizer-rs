//! Streaming versus one-shot identity on the eight frozen inputs: vertex buffers (stride 12 and 28,
//! versions 0 and 1) and triangle index buffers (versions 0 and 1), chunk sizes 7, 4096 and 65536
//! (1 byte additionally on inputs under 30,000 vertices). Decoded bytes must be equal.
use candidate::{codec::*, Limits, Workspace};

fn read(path: &std::path::Path) -> (Vec<[f32; 3]>, Vec<u32>) {
    let data = std::fs::read(path).unwrap();
    let word = |a: usize| u32::from_le_bytes(data[a..a + 4].try_into().unwrap());
    let (n, ni) = (word(0) as usize, word(4) as usize);
    let p = (0..n).map(|v| core::array::from_fn(|k| f32::from_bits(word(8 + (v * 3 + k) * 4)))).collect();
    let i = (0..ni).map(|k| word(8 + n * 12 + k * 4)).collect();
    (p, i)
}

fn stream_vertex(v: &[u8], n: usize, stride: usize, version: u32, chunk: usize) -> Vec<u8> {
    let tail = (stride + if version == 0 { 0 } else { stride / 4 }).max(if version == 0 { 32 } else { 24 });
    let end = v.len() - tail;
    let mut out = vec![0u8; n * stride];
    let mut d = VertexBufferDecoder::new(n, stride, v[0], &v[end..], end - 1, Limits::default()).unwrap();
    let (mut at, mut o) = (1, 0);
    loop {
        let p = d.push(&v[at..(at + chunk).min(end)], &mut out[o..]).unwrap();
        at += p.consumed;
        o += p.written;
        if p.status == Status::Done {
            break;
        }
        assert!(p.consumed != 0 || p.written != 0, "stalled");
    }
    d.finish().unwrap();
    assert_eq!(o, out.len());
    out
}

fn stream_index(t: &[u8], count: usize, chunk: usize) -> Vec<u8> {
    let start = 1 + count / 3;
    let end = t.len() - 16;
    let mut out = vec![0u32; count];
    let mut d = IndexBufferDecoder::new(count, &t[..start], &t[end..], end - start, Limits::default()).unwrap();
    let (mut at, mut o) = (start, 0);
    loop {
        let p = d.push(&t[at..(at + chunk).min(end)], &mut out[o..]).unwrap();
        at += p.consumed;
        o += p.written;
        if p.status == Status::Done {
            break;
        }
        assert!(p.consumed != 0 || p.written != 0, "stalled");
    }
    d.finish().unwrap();
    assert_eq!(o, count);
    out.iter().flat_map(|x| x.to_le_bytes()).collect()
}

fn main() {
    let dir = std::env::args().nth(1).unwrap();
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).filter(|p| p.extension().is_some_and(|e| e == "bin")).collect();
    files.sort();
    let mut checks = 0u64;
    for path in &files {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let (p, ix) = read(path);
        let n = p.len();
        let positions: Vec<u8> = p.iter().flatten().flat_map(|x| x.to_le_bytes()).collect();
        let attrs: Vec<u8> = (0..n).flat_map(|v| {
            let mut r = positions[v * 12..v * 12 + 12].to_vec();
            r.extend_from_slice(&((v as u32).wrapping_mul(2654435761) >> 8).to_le_bytes());
            r.extend_from_slice(&[(v % 251) as u8; 12]);
            r
        }).collect();
        let chunks: Vec<usize> = if n < 30_000 { vec![1, 7, 4096, 65536] } else { vec![7, 4096, 65536] };
        for version in 0..=1u32 {
            for (stride, data) in [(12usize, &positions), (28usize, &attrs)] {
                let v = encode_vertex_buffer(data, n, stride, VertexEncoding::new(version as _, 2).unwrap(), &mut Workspace::default()).unwrap();
                let one = decode_vertex_buffer(n, stride, &v, &mut Workspace::default()).unwrap();
                assert_eq!(&one, data, "one-shot vertex {name}");
                for &chunk in &chunks {
                    assert_eq!(stream_vertex(&v, n, stride, version, chunk), one, "stream vertex {name} stride {stride} v{version} chunk {chunk}");
                    checks += 1;
                }
            }
            let t = encode_index_buffer(&ix, IndexEncoding::new(version as _).unwrap(), &mut Workspace::default()).unwrap();
            let mut one = vec![0u8; ix.len() * 4];
            decode_index_buffer_into(&mut one, ix.len(), 4, &t, &mut Workspace::default()).unwrap();
            for &chunk in &chunks {
                assert_eq!(stream_index(&t, ix.len(), chunk), one, "stream index {name} v{version} chunk {chunk}");
                checks += 1;
            }
        }
        println!("PASS {name} checks so far {checks}");
    }
    println!("TOTAL streaming-vs-one-shot checks {checks} differences 0");
}
