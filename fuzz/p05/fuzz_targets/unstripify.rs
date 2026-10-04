#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| meshopt_p05_fuzz::run("unstripify", data));
