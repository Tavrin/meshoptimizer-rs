// Existing protocol Result bytes, framed for repeated exact replay.
use std::io::{self, Read, Write};
fn main() {
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    loop {
        let mut size = [0; 4];
        if input.read_exact(&mut size).is_err() {
            break;
        }
        let mut bytes = vec![0; u32::from_le_bytes(size) as usize];
        input.read_exact(&mut bytes).unwrap();
        let result = match meshoptimizer_parity::execute(&bytes) {
            Ok(mut b) => {
                b.insert(0, 0);
                b
            }
            Err(e) => {
                let mut b = vec![1];
                b.extend(e.as_bytes());
                b
            }
        };
        output
            .write_all(&(result.len() as u32).to_le_bytes())
            .unwrap();
        output.write_all(&result).unwrap();
        output.flush().unwrap();
    }
}
