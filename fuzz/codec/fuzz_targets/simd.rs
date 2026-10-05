#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| {
    if let Some((&selector, data)) = data.split_first() {
        const ENTRIES: [u8; 18] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 23, 26, 27, 28, 29];
        meshopt_codec_fuzz::exercise(ENTRIES[usize::from(selector) % ENTRIES.len()], data);
    }
});
