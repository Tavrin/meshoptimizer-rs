use std::io::{Read, Write};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Vec::new();
    let path = std::env::args_os().nth(1);
    if let Some(path) = &path {
        std::fs::File::open(path)?
            .take(128 * 1024 * 1024 + 1)
            .read_to_end(&mut input)?;
    } else {
        std::io::stdin()
            .take(128 * 1024 * 1024 + 1)
            .read_to_end(&mut input)?;
    }
    let output = if path.is_some() {
        meshoptimizer_parity::execute_paired(&input)
    } else {
        meshoptimizer_parity::execute(&input)
    }
    .map_err(std::io::Error::other)?;
    std::io::stdout().write_all(&output)?;
    Ok(())
}
