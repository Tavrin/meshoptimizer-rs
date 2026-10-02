use core::fmt;

/// Checked input, resource, allocation, or numerical failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A triangle list does not contain a multiple of three indices.
    InvalidTopology,
    /// An index does not identify a supplied vertex.
    IndexOutOfBounds,
    /// A vertex view has an invalid stride, width, offset, or length.
    InvalidLayout,
    /// A parameter is outside the operation's supported domain.
    InvalidParameter,
    /// A vertex flag contains unknown bits.
    UnknownFlags,
    /// A codec version is unsupported.
    UnsupportedVersion,
    /// A caller destination is too small.
    BufferTooSmall,
    /// An encoded stream is malformed.
    InvalidStream,
    /// Size arithmetic or a platform allocation bound overflowed.
    SizeOverflow,
    /// The per-call storage or counted-work limit was exceeded.
    LimitExceeded,
    /// A fallible allocation failed.
    AllocationFailed,
    /// Geometry or an intermediate calculation is non-finite.
    NumericalFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidTopology => "invalid triangle topology",
            Self::IndexOutOfBounds => "vertex index out of bounds",
            Self::InvalidLayout => "invalid vertex layout",
            Self::InvalidParameter => "invalid parameter",
            Self::UnknownFlags => "unknown vertex flags",
            Self::UnsupportedVersion => "unsupported stream version",
            Self::BufferTooSmall => "destination buffer too small",
            Self::InvalidStream => "invalid encoded stream",
            Self::SizeOverflow => "size overflow",
            Self::LimitExceeded => "resource limit exceeded",
            Self::AllocationFailed => "allocation failed",
            Self::NumericalFailure => "non-finite geometry or calculation",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
