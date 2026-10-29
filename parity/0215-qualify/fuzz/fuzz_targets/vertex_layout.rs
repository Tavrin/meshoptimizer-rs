#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| meshopt_qualify_fuzz::vertex_layout(data));
