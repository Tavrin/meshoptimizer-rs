//! Detection is cached; ISA tokens can only be constructed here.
use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
/// Available diagnostic dispatch ceilings (not a stable public API).
pub enum Level {
    /// Safe reference.
    Scalar = 0,
    /// x86-64 baseline filters.
    Sse2 = 1,
    /// SSSE3 plus POPCNT vertex groups.
    Ssse3 = 2,
    /// SSE4.1 meshlet groups, also SSSE3 plus POPCNT.
    Sse41 = 3,
    /// AArch64 NEON.
    Neon = 4,
    /// wasm simd128.
    Wasm = 5,
}
static DETECTED: AtomicU8 = AtomicU8::new(255);

/// Hardware-supported level, detected once.
pub fn detected_level() -> Level {
    let cached = DETECTED.load(Ordering::Relaxed);
    if cached != 255 {
        return from_byte(cached);
    }
    let level = detect();
    DETECTED.store(level as u8, Ordering::Relaxed);
    level
}
fn from_byte(n: u8) -> Level {
    match n {
        1 => Level::Sse2,
        2 => Level::Ssse3,
        3 => Level::Sse41,
        4 => Level::Neon,
        5 => Level::Wasm,
        _ => Level::Scalar,
    }
}
fn detect() -> Level {
    #[cfg(target_arch = "x86_64")]
    {
        #[cfg(feature = "std")]
        let (shuffle, meshlet) = (
            std::is_x86_feature_detected!("ssse3") && std::is_x86_feature_detected!("popcnt"),
            std::is_x86_feature_detected!("sse4.1"),
        );
        #[cfg(not(feature = "std"))]
        let (shuffle, meshlet) = (
            cfg!(all(target_feature = "ssse3", target_feature = "popcnt")),
            cfg!(target_feature = "sse4.1"),
        );
        return if shuffle && meshlet {
            Level::Sse41
        } else if shuffle {
            Level::Ssse3
        } else {
            Level::Sse2
        };
    }
    #[cfg(target_arch = "aarch64")]
    {
        return Level::Neon;
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        return Level::Wasm;
    }
    #[allow(unreachable_code)]
    Level::Scalar
}

#[cfg(all(feature = "std", any(test, feature = "parity-internals")))]
std::thread_local! { static LIMIT: core::cell::Cell<u8> = const { core::cell::Cell::new(255) }; }
#[cfg(all(not(feature = "std"), any(test, feature = "parity-internals")))]
static LIMIT: AtomicU8 = AtomicU8::new(255);

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
pub(super) fn selected() -> Level {
    let detected = detected_level();
    #[cfg(any(test, feature = "parity-internals"))]
    {
        #[cfg(feature = "std")]
        let limit = LIMIT.with(|l| l.get());
        #[cfg(not(feature = "std"))]
        let limit = LIMIT.load(Ordering::Relaxed);
        if limit == 0 {
            return Level::Scalar;
        }
        if detected as u8 <= 3 && limit <= 3 {
            return from_byte((detected as u8).min(limit));
        }
    }
    detected
}

/// Run under a lower dispatch ceiling. Unsupported or cross-architecture levels
/// return InvalidParameter. With std the ceiling is thread-local and unwind-safe;
/// without std it is process-wide (concurrent diagnostics may lower each other).
#[cfg(any(test, feature = "parity-internals"))]
pub fn with_level<R>(level: Level, f: impl FnOnce() -> R) -> Result<R, crate::Error> {
    let detected = detected_level();
    if level != Level::Scalar && !(level == detected || (detected as u8 <= 3 && level <= detected))
    {
        return Err(crate::Error::InvalidParameter);
    }
    struct Restore(u8);
    impl Drop for Restore {
        fn drop(&mut self) {
            #[cfg(feature = "std")]
            LIMIT.with(|l| l.set(self.0));
            #[cfg(not(feature = "std"))]
            LIMIT.store(self.0, Ordering::Relaxed);
        }
    }
    #[cfg(feature = "std")]
    let old = LIMIT.with(|l| l.replace(level as u8));
    #[cfg(not(feature = "std"))]
    let old = LIMIT.swap(level as u8, Ordering::Relaxed);
    let _restore = Restore(old);
    Ok(f())
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
pub(super) struct Baseline(());
#[cfg(target_arch = "x86_64")]
pub(super) struct Ssse3(());
#[cfg(target_arch = "x86_64")]
pub(super) struct Sse41(());
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
pub(super) fn baseline() -> Option<Baseline> {
    (selected() != Level::Scalar).then_some(Baseline(()))
}
#[cfg(target_arch = "x86_64")]
pub(super) fn ssse3() -> Option<Ssse3> {
    matches!(selected(), Level::Ssse3 | Level::Sse41).then_some(Ssse3(()))
}
#[cfg(target_arch = "x86_64")]
pub(super) fn sse41() -> Option<Sse41> {
    (selected() == Level::Sse41).then_some(Sse41(()))
}
