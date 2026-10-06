use meshopt::ffi::*;
use std::io::{Read, Write};
fn w(b: &[u8], at: usize) -> usize {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap()) as usize
}
fn main() -> std::io::Result<()> {
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    loop {
        let mut size = [0; 4];
        if input.read_exact(&mut size).is_err() {
            return Ok(());
        }
        let mut b = vec![0; u32::from_le_bytes(size) as usize];
        input.read_exact(&mut b)?;
        let (op, n, s, mode, v, l, samples, it) = (
            w(&b, 4),
            w(&b, 8),
            w(&b, 12),
            w(&b, 16),
            w(&b, 24),
            w(&b, 28),
            w(&b, 32),
            w(&b, 36),
        );
        let src = &b[44..];
        let words: Vec<u32> = if op == 12 || op == 13 {
            src.chunks_exact(4)
                .map(|x| u32::from_le_bytes(x.try_into().unwrap()))
                .collect()
        } else {
            Vec::new()
        };
        let floats: Vec<f32> = if (14..=16).contains(&op) {
            src.chunks_exact(4)
                .map(|x| f32::from_le_bytes(x.try_into().unwrap()))
                .collect()
        } else {
            Vec::new()
        };
        let bound = unsafe {
            match op {
                11 => meshopt_encodeVertexBufferBound(n, s),
                12 => meshopt_encodeIndexBufferBound(n, 0x100000000),
                13 => meshopt_encodeIndexSequenceBound(n, 0x100000000),
                _ => n * s,
            }
        };
        let mut dest = vec![0u8; bound];
        let mut once = || {
            if mode & 128 == 0 {
                dest = vec![0; bound];
            }
            unsafe {
                match op {
                    1 => {
                        assert_eq!(
                            meshopt_decodeVertexBuffer(
                                dest.as_mut_ptr().cast(),
                                n,
                                s,
                                src.as_ptr(),
                                src.len()
                            ),
                            0
                        );
                        n * s
                    }
                    11 => meshopt_encodeVertexBufferLevel(
                        dest.as_mut_ptr(),
                        bound,
                        src.as_ptr().cast(),
                        n,
                        s,
                        l as i32,
                        v as i32,
                    ),
                    12 => {
                        meshopt_encodeIndexVersion(v as i32);
                        meshopt_encodeIndexBuffer(dest.as_mut_ptr(), bound, words.as_ptr(), n)
                    }
                    13 => {
                        meshopt_encodeIndexVersion(v as i32);
                        meshopt_encodeIndexSequence(dest.as_mut_ptr(), bound, words.as_ptr(), n)
                    }
                    14 => {
                        meshopt_encodeFilterOct(
                            dest.as_mut_ptr().cast(),
                            n,
                            s,
                            l as i32,
                            floats.as_ptr(),
                        );
                        n * s
                    }
                    15 => {
                        meshopt_encodeFilterQuat(
                            dest.as_mut_ptr().cast(),
                            n,
                            s,
                            l as i32,
                            floats.as_ptr(),
                        );
                        n * s
                    }
                    16 => {
                        meshopt_encodeFilterExp(
                            dest.as_mut_ptr().cast(),
                            n,
                            s,
                            l as i32,
                            floats.as_ptr(),
                            (mode & 127) as u32,
                        );
                        n * s
                    }
                    _ => panic!("unsupported operation"),
                }
            }
        };
        let mut used = once();
        let mut times = Vec::new();
        for _ in 0..samples {
            let start = std::time::Instant::now();
            for _ in 0..it {
                used = std::hint::black_box(once());
            }
            times.push(start.elapsed().as_secs_f64() / it as f64);
        }
        let mut out = b"MD02".to_vec();
        out.extend(0i32.to_le_bytes());
        out.extend((used as u32).to_le_bytes());
        out.extend((samples as u32).to_le_bytes());
        out.extend(&dest[..used]);
        for t in times {
            out.extend(t.to_le_bytes());
        }
        output.write_all(&(out.len() as u32).to_le_bytes())?;
        output.write_all(&out)?;
        output.flush()?;
    }
}
