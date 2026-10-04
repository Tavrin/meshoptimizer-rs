#![no_main]
libfuzzer_sys::fuzz_target!(|data: &[u8]| meshopt_p03_fuzz::exercise(7, data));
