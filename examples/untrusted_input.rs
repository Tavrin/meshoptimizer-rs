use meshoptimizer_rs::{codec::decode_vertex_buffer, Error, Limits, Workspace};

fn main() -> Result<(), Error> {
    // Check transport sizes before buffering a request or creating its views.
    let source = [0u8; 4]; // Deliberately malformed stream supplied by a client.
    let count = 3usize;
    let stride = 12usize;
    let limits = Limits {
        max_bytes: 64 << 20,
        max_work: 1 << 20,
    };
    if source.len() > limits.max_bytes || count > limits.max_bytes / stride {
        return Err(Error::LimitExceeded);
    }
    let mut workspace = Workspace::new(limits);
    match decode_vertex_buffer(count, stride, &source, &mut workspace) {
        Ok(bytes) => println!("{} decoded bytes", bytes.len()),
        Err(Error::InvalidStream) => println!("rejected malformed input"),
        Err(error) => return Err(error),
    }
    Ok(())
}
