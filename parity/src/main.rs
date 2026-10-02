use std::io::{Read, Write};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Vec::new();
    std::io::stdin()
        .take(128 * 1024 * 1024 + 1)
        .read_to_end(&mut input)?;
    let output = meshoptimizer_parity::execute(&input).map_err(std::io::Error::other)?;
    std::io::stdout().write_all(&output)?;
    Ok(())
}
