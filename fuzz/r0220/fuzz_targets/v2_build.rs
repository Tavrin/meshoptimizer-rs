#![no_main]
#[path = "../build_support.rs"]
mod build_support;
#[path = "../vg_support.rs"]
mod support;
libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if data.len() <= 4096 {
        build_support::exercise(data);
    }
});
