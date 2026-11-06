//! Validates a WGSL file with no optional capabilities, which rejects 64-bit integers, f16,
//! subgroups and other extensions. A separate crate because this dependency tree would otherwise
//! change the feature set of `libm` in the library's own test builds.
fn main() {
    let path = std::env::args().nth(1).expect("usage: wgsl-check <file.wgsl>");
    let source = std::fs::read_to_string(&path).expect("read");
    let module = naga::front::wgsl::parse_str(&source).expect("parse");
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .expect("validate");
    println!("{path}: valid");
}
