// One pinned backend for std and no_std geometric operations.
pub(crate) fn sqrt(value: f32) -> f32 {
    libm::sqrtf(value)
}
