use crate::{workspace::checked_bytes, Error, Limits};

/// The next action for an incremental index-sequence decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Status {
    /// Supply more compressed bytes. Only [`IndexSequenceDecoder::finish`]
    /// declares an exhausted input malformed.
    NeedInput,
    /// Supply output space for at least one more index.
    NeedOutput,
    /// All indices and four padding bytes have arrived. Call `finish` at EOF.
    Done,
}

/// Progress made by one successful [`IndexSequenceDecoder::push`] call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Progress {
    /// Bytes consumed from this call's input; resubmit its unconsumed suffix.
    pub consumed: usize,
    /// Indices written to this call's output; its remaining tail is untouched.
    pub written: usize,
    /// The next action required by the decoder.
    pub status: Status,
}

/// Incremental version 0/1 index-sequence decoding, not in C++ meshoptimizer.
///
/// Opt in by constructing this decoder; no Cargo feature is required. Input
/// can arrive a byte at a time. Only complete indices are emitted, retaining
/// the partial varint and both wrapping predictor baselines between calls.
/// **All emitted indices are provisional until [`Self::finish`] succeeds.**
/// `Done` means the declared count and padding have arrived, not that EOF has
/// been verified: submit every byte of this stream, then call `finish` at EOF.
/// Extra input, including input submitted after `Done`, is rejected.
///
/// As with [`super::decode_index_sequence`], padding is exactly four opaque
/// bytes (not necessarily zero), varints take at most five bytes even if the
/// fifth continuation bit is set, and excess high bits are discarded. This
/// deliberately accepts upstream's noncanonical encodings.
///
/// No heap storage is allocated. Caller output and the fixed decoder state
/// are excluded from `Limits::max_bytes`, so a zero byte limit is supported.
/// The whole stream shares one work budget: construction charges four bytes
/// per declared index, then `push` charges one unit per consumed input byte,
/// independently of chunking. Empty calls cost nothing. Unlike one-shot
/// decoding, resources cannot be preflighted against an unknown source length.
/// Under work limits, a malformed stream can therefore report its own decoding
/// error before `LimitExceeded`, because work is charged per consumed byte
/// rather than for the whole input up front; a valid stream that runs out of
/// work always reports `LimitExceeded`.
///
/// Every error is terminal and is repeated by later calls and `finish`.
/// An error may follow provisional writes in that call; no progress receipt
/// is returned on error. A bad header/version is reported immediately, which
/// can precede the one-shot decoder's minimum-length error on truncated input.
pub struct IndexSequenceDecoder {
    remaining: usize,
    last: [u32; 2],
    value: u32,
    groups: u8,
    header: bool,
    padding: u8,
    work_left: u64,
    error: Option<Error>,
}

impl IndexSequenceDecoder {
    /// Start a decoder for the count supplied by the container format.
    ///
    /// # Errors
    /// `SizeOverflow` if the decoded size exceeds platform bounds, or
    /// `LimitExceeded` if four work units per index exceed `limits.max_work`.
    pub fn new(index_count: usize, limits: Limits) -> Result<Self, Error> {
        let bytes = checked_bytes(index_count, 4)?;
        let work = u64::try_from(bytes).map_err(|_| Error::SizeOverflow)?;
        let work_left = limits
            .max_work
            .checked_sub(work)
            .ok_or(Error::LimitExceeded)?;
        Ok(Self {
            remaining: index_count,
            last: [0; 2],
            value: 0,
            groups: 0,
            header: false,
            padding: 0,
            work_left,
            error: None,
        })
    }

    /// Consume arriving bytes and emit complete `u32` indices.
    ///
    /// Resubmit `input[progress.consumed..]` with fresh output space. A zero
    /// length output is allowed; headers and padding can still be consumed.
    /// While indices remain, lack of output takes precedence over lack of
    /// input, even during a partial varint. `NeedInput` is never an EOF error.
    /// Calling with empty input after `Done` is harmless.
    ///
    /// # Errors
    /// `InvalidStream` for a bad header or extra bytes; `UnsupportedVersion`
    /// for a sequence version above 1; `LimitExceeded` for exhausted work.
    /// Errors are sticky; any output already emitted remains provisional.
    /// Source-compatible u32 operation.
    #[inline]
    pub fn push(&mut self, input: &[u8], output: &mut [u32]) -> Result<Progress, Error> {
        self.push_generic::<u32>(input, output)
    }
    /// Select u16 or u32 output through the sealed Index trait.
    pub fn push_generic<I: crate::Index>(
        &mut self,
        input: &[u8],
        output: &mut [I],
    ) -> Result<Progress, Error> {
        if let Some(error) = self.error {
            return Err(error);
        }
        let result = self.push_inner(input, output);
        if let Err(error) = result {
            self.error = Some(error);
        }
        result
    }

    fn push_inner<I: crate::Index>(
        &mut self,
        input: &[u8],
        output: &mut [I],
    ) -> Result<Progress, Error> {
        let mut consumed = 0;
        let mut written = 0;
        let status = loop {
            if self.header && self.remaining == 0 && self.padding == 4 {
                if consumed != input.len() {
                    return Err(Error::InvalidStream);
                }
                break Status::Done;
            }
            if self.header && self.remaining != 0 && written == output.len() {
                break Status::NeedOutput;
            }
            let Some(&byte) = input.get(consumed) else {
                break Status::NeedInput;
            };
            self.work_left = self.work_left.checked_sub(1).ok_or(Error::LimitExceeded)?;
            consumed += 1;
            if !self.header {
                if byte & 0xf0 != 0xd0 {
                    return Err(Error::InvalidStream);
                }
                if byte & 0x0f > 1 {
                    return Err(Error::UnsupportedVersion);
                }
                self.header = true;
            } else if self.remaining != 0 {
                self.value |= u32::from(byte & 127) << (self.groups * 7);
                self.groups += 1;
                if byte < 128 || self.groups == 5 {
                    let baseline = (self.value & 1) as usize;
                    let value = self.value >> 1;
                    let delta = (value >> 1) ^ 0u32.wrapping_sub(value & 1);
                    let index = self.last[baseline].wrapping_add(delta);
                    self.last[baseline] = index;
                    output[written] = I::from_u32(index)?;
                    written += 1;
                    self.remaining -= 1;
                    self.value = 0;
                    self.groups = 0;
                }
            } else {
                self.padding += 1;
            }
        };
        Ok(Progress {
            consumed,
            written,
            status,
        })
    }

    /// Validate EOF, the declared index count, and exactly four padding bytes.
    /// Only after success may the provisional output be committed.
    ///
    /// # Errors
    /// The previous terminal error, or `InvalidStream` for an incomplete
    /// header, varint, index sequence, or padding. Call only after all input
    /// has been submitted and all requested output space has been supplied.
    pub fn finish(self) -> Result<(), Error> {
        if let Some(error) = self.error {
            Err(error)
        } else if self.header && self.remaining == 0 && self.padding == 4 {
            Ok(())
        } else {
            Err(Error::InvalidStream)
        }
    }
}
