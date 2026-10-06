use std::io::{Read, Write};
fn main() -> std::io::Result<()> {
    #[cfg(feature = "simd")]
    if std::env::args().nth(1).as_deref() == Some("levels") {
        use meshoptimizer_rs::codec::{with_level, Level};
        for (name,level) in [("scalar",Level::Scalar),("sse2",Level::Sse2),("ssse3",Level::Ssse3),("sse41",Level::Sse41),("neon",Level::Neon),("wasm",Level::Wasm)] {
            if with_level(level,||()).is_ok() { println!("{name}"); }
        }
        return Ok(());
    }
    #[cfg(feature = "simd")]
    let ceiling = std::env::args().nth(1).map(|s| match s.as_str() {
        "scalar" => meshoptimizer_rs::codec::Level::Scalar,
        "sse2" => meshoptimizer_rs::codec::Level::Sse2,
        "ssse3" => meshoptimizer_rs::codec::Level::Ssse3,
        "sse41" => meshoptimizer_rs::codec::Level::Sse41,
        "neon" => meshoptimizer_rs::codec::Level::Neon,
        "wasm" => meshoptimizer_rs::codec::Level::Wasm,
        _ => panic!("invalid lowering ceiling"),
    });
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    loop {
        let mut size = [0; 4];
        if input.read_exact(&mut size).is_err() {
            return Ok(());
        }
        let n = u32::from_le_bytes(size) as usize;
        if n > 128 * 1024 * 1024 {
            return Err(std::io::Error::other("request too large"));
        }
        let mut b = vec![0; n];
        input.read_exact(&mut b)?;
        #[cfg(feature = "simd")]
        let result = if let Some(level) = ceiling {
            meshoptimizer_rs::codec::with_level(level, || meshopt_codec_parity::execute(&b))
                .map_err(|e| std::io::Error::other(e.to_string()))?
        } else { meshopt_codec_parity::execute(&b) };
        #[cfg(not(feature = "simd"))]
        let result = meshopt_codec_parity::execute(&b);
        let out = result.map_err(|e| std::io::Error::other(e.to_string()))?;
        output.write_all(&(out.len() as u32).to_le_bytes())?;
        output.write_all(&out)?;
        output.flush()?;
    }
}
