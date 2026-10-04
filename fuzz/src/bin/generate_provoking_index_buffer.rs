#[path = "../p01x_main.rs"]
mod preprocessing;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    preprocessing::run("generate_provoking_index_buffer")
}
