#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| {
    meshopt_codec_fuzz::exercise(13, data);
});
