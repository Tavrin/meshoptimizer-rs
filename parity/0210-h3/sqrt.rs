// Built against the actual partition helper extracted verbatim by proof-build.py.
use std::hint::black_box;
struct Context<'a>(std::marker::PhantomData<&'a ()>);
impl Context<'_> {
    #[allow(dead_code)]
    fn sqrt(&mut self, x: f32) -> f32 {
        libm::sqrtf(x)
    }
}
include!(concat!(env!("H3_SCRATCH"), "/partition-sqrt.rs"));
fn main() {
    let mut context = Context(std::marker::PhantomData);
    let mut mismatches = 0u64;
    let mut bare_mismatches = 0u64;
    let mut first_bare = None;
    for bits in 0..=u32::MAX {
        let x = black_box(f32::from_bits(bits));
        let expected = libm::sqrtf(x).to_bits();
        let actual = sqrt(&mut context, x).to_bits();
        if actual != expected {
            if mismatches < 8 {
                eprintln!("replacement {bits:08x}: {actual:08x} != {expected:08x}");
            }
            mismatches += 1;
        }
        if x.sqrt().to_bits() != expected {
            bare_mismatches += 1;
            first_bare.get_or_insert(bits);
        }
    }
    println!("inputs=4294967296 replacement_mismatches={mismatches} bare_mismatches={bare_mismatches} first_bare={first_bare:?}");
    assert_eq!(mismatches, 0);
}
