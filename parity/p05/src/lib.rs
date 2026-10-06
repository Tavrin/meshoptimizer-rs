// The native and WASM drivers execute the same safe request interpreter.
#![allow(dead_code)]
include!("main.rs");
use std::sync::Mutex;
static INPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn request(length: usize) -> u32 {
    if length > 128 {
        return 1;
    }
    let Ok(mut input) = INPUT.lock() else {
        return 1;
    };
    input.clear();
    input.resize(length, 0);
    0
}
#[no_mangle]
pub extern "C" fn put_byte(index: usize, value: u8) -> u32 {
    let Ok(mut input) = INPUT.lock() else {
        return 1;
    };
    let Some(slot) = input.get_mut(index) else {
        return 1;
    };
    *slot = value;
    0
}
#[export_name = "run"]
pub extern "C" fn run_wasm() -> u32 {
    let Ok(input) = INPUT.lock() else { return 1 };
    let Ok(line) = std::str::from_utf8(&input) else {
        return 1;
    };
    let mut parts = line.split_ascii_whitespace();
    let Some(family) = parts.next() else { return 1 };
    let Some(seed) = parts.next().and_then(|s| s.parse::<u32>().ok()) else {
        return 1;
    };
    let case = case_with(seed, None);
    let Ok(result) = run(family, seed, &case) else {
        return 1;
    };
    let Ok(mut output) = OUTPUT.lock() else {
        return 1;
    };
    output.clear();
    output.extend_from_slice(&input_hash(&case).to_le_bytes());
    output.extend_from_slice(&result);
    0
}
#[no_mangle]
pub extern "C" fn output_len() -> usize {
    OUTPUT.lock().map_or(0, |v| v.len())
}
#[no_mangle]
pub extern "C" fn get_byte(index: usize) -> u32 {
    OUTPUT
        .lock()
        .ok()
        .and_then(|v| v.get(index).copied())
        .map_or(256, u32::from)
}
