use std::io::{self, Read, Write};
fn main() {
    let mut i = io::stdin().lock();
    let mut o = io::stdout().lock();
    loop {
        let mut n = [0; 4];
        if i.read_exact(&mut n).is_err() {
            break;
        }
        let n = u32::from_le_bytes(n) as usize;
        assert!(n <= 128 * 1024 * 1024);
        let mut b = vec![0; n];
        i.read_exact(&mut b).unwrap();
        let result = meshopt_p03_parity::execute(&b).unwrap();
        o.write_all(&(result.len() as u32).to_le_bytes()).unwrap();
        o.write_all(&result).unwrap();
        o.flush().unwrap();
    }
}
