use meshoptimizer_rs::{codec::*, Error, Limits};

pub fn vertex(
    count: usize,
    stride: usize,
    source: &[u8],
    chunk: usize,
    width: usize,
) -> Result<Vec<u8>, Error> {
    let version = decode_vertex_version(source)?;
    let tail = (stride + if version == 0 { 0 } else { stride / 4 }).max(if version == 0 {
        32
    } else {
        24
    });
    if source.len() < 1 + tail {
        return Err(Error::InvalidStream);
    }
    let end = source.len() - tail;
    let mut decoder = VertexBufferDecoder::new(
        count,
        stride,
        source[0],
        &source[end..],
        end - 1,
        Limits::default(),
    )?;
    let mut result = Vec::new();
    let mut at = 1usize;
    loop {
        let size = 1 + at.wrapping_mul(1664525).wrapping_add(1013904223) % chunk;
        let stop = at.saturating_add(size).min(end);
        let mut output = vec![0xcd; width + 7];
        let p = decoder.push(&source[at..stop], &mut output[..width])?;
        assert!(output[p.written..].iter().all(|&v| v == 0xcd));
        result.extend_from_slice(&output[..p.written]);
        at += p.consumed;
        if p.status == Status::Done || (at == end && p.status == Status::NeedInput) {
            break;
        }
        assert!(p.consumed > 0 || p.written > 0 || p.status == Status::NeedInput);
    }
    decoder.finish()?;
    Ok(result)
}
pub fn triangles(
    count: usize,
    source: &[u8],
    chunk: usize,
    width: usize,
) -> Result<Vec<u8>, Error> {
    if source.len() < 17 + count / 3 {
        return Err(Error::InvalidStream);
    }
    let end = source.len() - 16;
    let mut decoder = IndexBufferDecoder::new(
        count,
        &source[..1 + count / 3],
        &source[end..],
        end - 1 - count / 3,
        Limits::default(),
    )?;
    let mut result = Vec::new();
    let mut at = 1usize + count / 3;
    loop {
        let size = 1 + at.wrapping_mul(1664525).wrapping_add(1013904223) % chunk;
        let stop = at.saturating_add(size).min(end);
        let mut output = vec![0xdeadbeef; width + 7];
        let p = decoder.push(&source[at..stop], &mut output[..width])?;
        assert!(output[p.written..].iter().all(|&v| v == 0xdeadbeef));
        result.extend(output[..p.written].iter().flat_map(|v| v.to_le_bytes()));
        at += p.consumed;
        if p.status == Status::Done || (at == end && p.status == Status::NeedInput) {
            break;
        }
        assert!(p.consumed > 0 || p.written > 0 || p.status == Status::NeedInput);
    }
    decoder.finish()?;
    Ok(result)
}
