//! Unpublished versioned binary driver and safe WASM byte transport.
use meshoptimizer_rs::{optimize_overdraw, optimize_vertex_cache, Positions, Workspace};
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

const MAX: usize = 128 * 1024 * 1024;
fn word(input: &[u8], offset: usize) -> Result<u32, String> {
    let bytes = input.get(offset..offset + 4).ok_or("short message")?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}
fn push(output: &mut Vec<u8>, word: u32) {
    output.extend_from_slice(&word.to_le_bytes());
}

/// Validate and execute one protocol message, including paired benchmark sampling.
pub fn execute(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.get(..4) != Some(b"MO01") {
        return Err("bad protocol version".into());
    }
    let op = word(input, 4)?;
    let vertices = word(input, 8)? as usize;
    let count = word(input, 12)? as usize;
    let threshold = f32::from_bits(word(input, 16)?);
    let mode = word(input, 20)?;
    let samples = word(input, 24)? as usize;
    if op == 0
        || op > 6
        || mode > 1
        || samples > 100
        || (mode == 1 && samples < 1)
        || (mode == 0 && samples != 0)
    {
        return Err("unsupported operation or mode".into());
    }
    let expected = vertices
        .checked_mul(12)
        .and_then(|n| count.checked_mul(4).and_then(|c| n.checked_add(c)))
        .and_then(|n| n.checked_add(28))
        .ok_or("count overflow")?;
    if expected > MAX || (op <= 3 && input.len() != expected) || input.len() < expected {
        return Err("invalid message length".into());
    }
    let mut positions = Vec::new();
    positions
        .try_reserve_exact(vertices)
        .map_err(|e| e.to_string())?;
    for i in 0..vertices {
        positions.push([
            f32::from_bits(word(input, 28 + i * 12)?),
            f32::from_bits(word(input, 32 + i * 12)?),
            f32::from_bits(word(input, 36 + i * 12)?),
        ]);
    }
    let mut indices = Vec::new();
    indices
        .try_reserve_exact(count)
        .map_err(|e| e.to_string())?;
    for i in 0..count {
        indices.push(word(input, 28 + vertices * 12 + i * 4)?);
    }
    let mut target = 0;
    let mut error = 0.0;
    let mut options = meshoptimizer_rs::SimplifyOptions::EMPTY;
    let mut ac = 0;
    let mut weights = Vec::new();
    let mut attributes = Vec::new();
    let mut flags = Vec::new();
    if op >= 4 {
        target = word(input, expected)? as usize;
        error = f32::from_bits(word(input, expected + 4)?);
        options = meshoptimizer_rs::SimplifyOptions::from_bits(word(input, expected + 8)?)
            .map_err(|e| e.to_string())?;
        ac = word(input, expected + 12)? as usize;
        if ac > 32 || input.len() != expected + 16 + ac * 4 + vertices * ac * 4 + vertices * 4 {
            return Err("invalid simplifier message".into());
        }
        for k in 0..ac {
            weights.push(f32::from_bits(word(input, expected + 16 + k * 4)?));
        }
        for k in 0..vertices * ac {
            attributes.push(f32::from_bits(word(input, expected + 16 + ac * 4 + k * 4)?));
        }
        for k in 0..vertices {
            let raw = word(input, expected + 16 + ac * 4 + vertices * ac * 4 + k * 4)?;
            let raw = u8::try_from(raw).map_err(|_| "invalid flag width")?;
            flags.push(meshoptimizer_rs::VertexFlags::from_bits(raw).map_err(|e| e.to_string())?);
        }
    }
    let operation = || -> Result<Vec<u32>, String> {
        match op {
            1 => optimize_vertex_cache(&indices, vertices, &mut Workspace::default())
                .map_err(|e| e.to_string()),
            2 => optimize_overdraw(
                &indices,
                Positions::from_packed(&positions),
                threshold,
                &mut Workspace::default(),
            )
            .map_err(|e| e.to_string()),
            3 => {
                if mode != 0 || vertices != 0 {
                    return Err("invalid math probe".into());
                }
                let mut out = Vec::new();
                out.try_reserve_exact(count).map_err(|e| e.to_string())?;
                for &bits in &indices {
                    out.push(libm::sqrtf(f32::from_bits(bits)).to_bits());
                }
                Ok(out)
            }
            4 | 5 => {
                let settings = meshoptimizer_rs::SimplifySettings {
                    target_index_count: target,
                    target_error: error,
                    options,
                };
                let p = Positions::from_packed(&positions);
                let mut workspace = Workspace::default();
                let result = if op == 4 {
                    meshoptimizer_rs::simplify(&indices, p, settings, &mut workspace)
                } else {
                    let a = meshoptimizer_rs::Attributes::from_interleaved(
                        &attributes,
                        vertices,
                        ac,
                        ac,
                        0,
                    )
                    .map_err(|e| e.to_string())?;
                    meshoptimizer_rs::simplify_with_attributes(
                        &indices,
                        p,
                        a,
                        &weights,
                        Some(&flags),
                        settings,
                        &mut workspace,
                    )
                }
                .map_err(|e| e.to_string())?;
                let mut out = Vec::with_capacity(result.indices.len() + 1);
                out.push(result.error.to_bits());
                out.extend(result.indices);
                Ok(out)
            }
            6 => Ok(vec![meshoptimizer_rs::simplify_scale(
                Positions::from_packed(&positions),
            )
            .map_err(|e| e.to_string())?
            .to_bits()]),
            _ => Err("unknown operation".into()),
        }
    };
    let mut times = Vec::<f64>::new();
    times
        .try_reserve_exact(samples)
        .map_err(|e| e.to_string())?;
    let result = operation()?; // untimed parity result and benchmark warm-up
    if mode == 1 {
        #[cfg(target_arch = "wasm32")]
        return Err("WASM timing is not a native benchmark".into());
        #[cfg(not(target_arch = "wasm32"))]
        for _ in 0..samples {
            let start = Instant::now();
            let out = operation()?;
            std::hint::black_box(&out);
            times.push(start.elapsed().as_secs_f64());
        }
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(16 + result.len() * 4 + times.len() * 8)
        .map_err(|e| e.to_string())?;
    output.extend_from_slice(b"MR01");
    push(&mut output, 0);
    push(&mut output, result.len() as u32);
    push(&mut output, times.len() as u32);
    for index in result {
        push(&mut output, index);
    }
    for seconds in times {
        output.extend_from_slice(&seconds.to_le_bytes());
    }
    Ok(output)
}

static INPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// Reserve one bounded WASM request; returns zero on success.
#[no_mangle]
pub extern "C" fn request(size: u32) -> u32 {
    if size as usize > MAX {
        return 1;
    }
    match INPUT.lock() {
        Ok(mut input) => {
            input.clear();
            if input.try_reserve_exact(size as usize).is_err() {
                return 1;
            }
            input.resize(size as usize, 0);
            0
        }
        Err(_) => 1,
    }
}
/// Set a request byte; returns zero on success.
#[no_mangle]
pub extern "C" fn put_byte(index: u32, value: u32) -> u32 {
    match INPUT.lock() {
        Ok(mut input) => match input.get_mut(index as usize) {
            Some(b) => {
                *b = value as u8;
                0
            }
            None => 1,
        },
        Err(_) => 1,
    }
}
/// Execute a request; failures clear previous output and return one.
#[no_mangle]
pub extern "C" fn run() -> u32 {
    let Ok(mut output) = OUTPUT.lock() else {
        return 1;
    };
    output.clear();
    let Ok(input) = INPUT.lock() else {
        return 1;
    };
    match execute(&input) {
        Ok(bytes) => {
            *output = bytes;
            0
        }
        Err(_) => 1,
    }
}
/// Return current response length.
#[no_mangle]
pub extern "C" fn output_len() -> u32 {
    match OUTPUT.lock() {
        Ok(output) => output.len() as u32,
        Err(_) => 0,
    }
}
/// Read a response byte; out-of-range access returns 256.
#[no_mangle]
pub extern "C" fn get_byte(index: u32) -> u32 {
    match OUTPUT.lock() {
        Ok(output) => match output.get(index as usize) {
            Some(b) => *b as u32,
            None => 256,
        },
        Err(_) => 256,
    }
}
