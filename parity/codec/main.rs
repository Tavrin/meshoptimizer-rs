use std::io::{Read, Write};
fn main() -> std::io::Result<()> {
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
        let out =
            meshopt_codec_parity::execute(&b).map_err(|e| std::io::Error::other(e.to_string()))?;
        output.write_all(&(out.len() as u32).to_le_bytes())?;
        output.write_all(&out)?;
        output.flush()?;
    }
}
