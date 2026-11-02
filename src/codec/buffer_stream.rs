use super::{index::TriangleState, Progress, Status};
use crate::{Error, Limits};

// A compressed vertex block is at most 8192 literals + 512 group headers +
// 64 controls. The remaining space covers the largest (320 byte) trailer.
const CAPACITY: usize = 9216;
struct Input {
    bytes: [u8; CAPACITY],
    // Pending window is bytes[start..used]; consuming advances `start`
    // instead of moving data, and `fill` compacts only when space runs out.
    start: usize,
    used: usize,
    left: usize,
    tail: [u8; 320],
    tail_len: usize,
    appended: bool,
}
impl Input {
    fn new(left: usize, tail: &[u8]) -> Self {
        let mut result = Self {
            bytes: [0; CAPACITY],
            start: 0,
            used: 0,
            left,
            tail: [0; 320],
            tail_len: tail.len(),
            appended: false,
        };
        result.tail[..tail.len()].copy_from_slice(tail);
        result
    }
    fn pending(&self) -> &[u8] {
        &self.bytes[self.start..self.used]
    }
    fn len(&self) -> usize {
        self.used - self.start
    }
    fn compact(&mut self) {
        self.bytes.copy_within(self.start..self.used, 0);
        self.used -= self.start;
        self.start = 0;
    }
    /// True if the pending window was full only because consumed bytes were
    /// not yet moved out; it then compacts, and the caller should retry.
    fn reclaim(&mut self) -> bool {
        if self.start > 0 && self.used + self.tail_len >= CAPACITY {
            self.compact();
            true
        } else {
            false
        }
    }
    fn fill(&mut self, input: &[u8]) -> usize {
        let wanted = input.len().min(self.left);
        // Once the trailer is appended `used` may reach CAPACITY; nothing more arrives.
        let free = CAPACITY.saturating_sub(self.used + self.tail_len);
        // Amortized: moving at most ~3 bytes per consumed byte.
        if free < wanted && self.start >= CAPACITY / 4 {
            self.compact();
        }
        let n = wanted.min(CAPACITY.saturating_sub(self.used + self.tail_len));
        self.bytes[self.used..self.used + n].copy_from_slice(&input[..n]);
        self.used += n;
        self.left -= n;
        if self.left == 0 && !self.appended {
            self.bytes[self.used..self.used + self.tail_len]
                .copy_from_slice(&self.tail[..self.tail_len]);
            self.used += self.tail_len;
            self.appended = true;
        }
        n
    }
    fn remove(&mut self, n: usize) {
        self.start += n;
    }
    fn complete(&self) -> bool {
        self.appended && self.len() == self.tail_len
    }
}
fn budget(bytes: usize, source: usize, limits: Limits) -> Result<(), Error> {
    let work = bytes.checked_add(source).ok_or(Error::SizeOverflow)?;
    if u64::try_from(work).map_err(|_| Error::SizeOverflow)? > limits.max_work {
        return Err(Error::LimitExceeded);
    }
    Ok(())
}

/// Incremental vertex-buffer decoder, not in C++ meshoptimizer.
///
/// Supply the header byte and the **entire padded trailer** separately to
/// [`Self::new`], then submit exactly `body_len` bytes between them to `push`.
/// The container must provide that length. Version 0 trailers have length
/// `max(stride, 32)`; version 1 uses `max(stride + stride / 4, 24)`.
/// A block becomes available once its bytes and the parser's 24-byte lookahead
/// have arrived. Output may be split at any byte boundary.
///
/// Fixed storage is independent of stream size: 9216 compressed bytes, 8192
/// decoded bytes, and at most 640 trailer/predictor bytes, plus scalar state.
/// No heap allocation occurs; fixed state and caller buffers are excluded from
/// `max_bytes`. Work is preflighted once as encoded plus decoded bytes, as in
/// the one-shot caller-buffer decoder. Parsing incomplete blocks retries at
/// most one bounded block per arriving chunk. A buffered block runs the same
/// SIMD block kernels as the one-shot decoder.
/// All output is provisional until `finish` succeeds. Errors are sticky.
#[non_exhaustive]
pub struct VertexBufferDecoder {
    input: Input,
    count: usize,
    stride: usize,
    version: u8,
    last: [u8; 256],
    channels: [u8; 64],
    output: [u8; 8192],
    ready: usize,
    at: usize,
    error: Option<Error>,
}
impl VertexBufferDecoder {
    /// Construct from container metadata and the separately fetched trailer.
    ///
    /// # Errors
    /// The one-shot layout, header, size and work errors; `InvalidStream` for
    /// a trailer of the wrong length or an impossible declared source length.
    pub fn new(
        count: usize,
        stride: usize,
        header: u8,
        trailer: &[u8],
        body_len: usize,
        limits: Limits,
    ) -> Result<Self, Error> {
        let bytes = super::layout(super::Mode::Attributes, count, stride)?;
        let source = body_len
            .checked_add(trailer.len())
            .and_then(|n| n.checked_add(1))
            .ok_or(Error::SizeOverflow)?;
        budget(bytes, source, limits)?;
        let version = super::decode_vertex_version(&[header])?;
        let tail = stride + if version == 0 { 0 } else { stride / 4 };
        let padded = tail.max(if version == 0 { 32 } else { 24 });
        if trailer.len() != padded {
            return Err(Error::InvalidStream);
        }
        let mut last = [0; 256];
        last[..stride].copy_from_slice(&trailer[padded - tail..padded - tail + stride]);
        let mut channels = [0; 64];
        if version != 0 {
            channels[..stride / 4].copy_from_slice(&trailer[padded - stride / 4..]);
        }
        Ok(Self {
            input: Input::new(body_len, trailer),
            count,
            stride,
            version,
            last,
            channels,
            output: [0; 8192],
            ready: 0,
            at: 0,
            error: None,
        })
    }
    /// Feed body bytes and drain decoded bytes; resubmit the unconsumed suffix.
    /// `Progress::written` counts bytes. Empty output is supported.
    ///
    /// # Errors
    /// `InvalidStream` for malformed/trailing data. Earlier writes are provisional.
    pub fn push(&mut self, input: &[u8], output: &mut [u8]) -> Result<Progress, Error> {
        if let Some(e) = self.error {
            return Err(e);
        }
        let result = self.push_inner(input, output);
        if let Err(e) = result {
            self.error = Some(e);
        }
        result
    }
    fn push_inner(&mut self, input: &[u8], output: &mut [u8]) -> Result<Progress, Error> {
        let (mut consumed, mut written) = (0, 0);
        let status = loop {
            let n = (self.ready - self.at).min(output.len() - written);
            output[written..written + n].copy_from_slice(&self.output[self.at..self.at + n]);
            self.at += n;
            written += n;
            if self.at != self.ready {
                break Status::NeedOutput;
            }
            consumed += self.input.fill(&input[consumed..]);
            if self.count == 0 {
                if self.input.left != 0 || !self.input.complete() || consumed != input.len() {
                    return Err(Error::InvalidStream);
                }
                break Status::Done;
            }
            let block = self.count.min(((8192 / self.stride) & !15).min(256));
            match super::vertex::stream_block(
                self.input.pending(),
                &mut self.output[..block * self.stride],
                block,
                self.stride,
                self.version,
                &self.last,
                &self.channels,
            ) {
                Ok(used) => {
                    if used
                        > self.input.len().saturating_sub(if self.input.appended {
                            self.input.tail_len
                        } else {
                            0
                        })
                    {
                        return Err(Error::InvalidStream);
                    }
                    self.input.remove(used);
                    self.ready = block * self.stride;
                    self.at = 0;
                    self.count -= block;
                    self.last[..self.stride]
                        .copy_from_slice(&self.output[self.ready - self.stride..self.ready]);
                }
                Err(_)
                    if self.input.left != 0
                        && self.input.len() < CAPACITY - self.input.tail_len =>
                {
                    if self.input.reclaim() {
                        continue;
                    }
                    break Status::NeedInput;
                }
                Err(e) => return Err(e),
            }
        };
        Ok(Progress {
            consumed,
            written,
            status,
        })
    }
    /// Validate EOF after draining output and submitting every body byte.
    ///
    /// # Errors
    /// The sticky error or `InvalidStream` for incomplete input/output.
    pub fn finish(self) -> Result<(), Error> {
        if let Some(e) = self.error {
            Err(e)
        } else if self.count == 0 && self.at == self.ready && self.input.complete() {
            Ok(())
        } else {
            Err(Error::InvalidStream)
        }
    }
}

/// Incremental triangle decoder, not in C++ meshoptimizer.
///
/// Fetch the header plus `index_count / 3` triangle codes and the final
/// sixteen-byte codebook first. `push` then accepts only the intervening body.
/// The codes are borrowed for the decoder lifetime: this format forces a
/// separate code stream, costing one caller-owned byte per triangle. Internal
/// storage is fixed (the shared 9216-byte input scratch plus FIFOs and a pending
/// triangle); it never retains the body or full decoded output. Work and
/// provisional-output rules match [`VertexBufferDecoder`]; construction budgets
/// four decoded bytes per index even for subsequent u16 output. Triangles become
/// available after their body bytes and up to sixteen lookahead bytes arrive.
#[non_exhaustive]
pub struct IndexBufferDecoder<'a> {
    input: Input,
    codes: &'a [u8],
    version: u8,
    state: TriangleState,
    output: [u32; 3],
    at: usize,
    error: Option<Error>,
}
impl<'a> IndexBufferDecoder<'a> {
    /// Construct from the header/codes, codebook and body length.
    ///
    /// # Errors
    /// One-shot layout/topology, header, overflow and work errors, or
    /// `InvalidStream` for incorrect prefix/trailer lengths.
    pub fn new(
        index_count: usize,
        prefix: &'a [u8],
        trailer: &[u8],
        body_len: usize,
        limits: Limits,
    ) -> Result<Self, Error> {
        let bytes = super::layout(super::Mode::Triangles, index_count, 4)?;
        let source = prefix
            .len()
            .checked_add(body_len)
            .and_then(|n| n.checked_add(trailer.len()))
            .ok_or(Error::SizeOverflow)?;
        budget(bytes, source, limits)?;
        if prefix.len() != 1 + index_count / 3 || trailer.len() != 16 {
            return Err(Error::InvalidStream);
        }
        if prefix[0] & 0xf0 != 0xe0 {
            return Err(Error::InvalidStream);
        }
        let version = super::decode_index_version(prefix)?;
        Ok(Self {
            input: Input::new(body_len, trailer),
            codes: &prefix[1..],
            version,
            state: TriangleState::new(),
            output: [0; 3],
            at: 3,
            error: None,
        })
    }
    /// Feed body bytes and emit u32 indices; written counts indices.
    ///
    /// # Errors
    /// `InvalidStream` for malformed or trailing input. Errors are sticky.
    pub fn push(&mut self, input: &[u8], output: &mut [u32]) -> Result<Progress, Error> {
        self.push_generic(input, output)
    }
    /// Emit u16/u32 indices. As in one-shot triangle decoding, u16 truncates.
    ///
    /// # Errors
    /// Same as [`Self::push`].
    pub fn push_generic<I: crate::Index>(
        &mut self,
        input: &[u8],
        output: &mut [I],
    ) -> Result<Progress, Error> {
        if let Some(e) = self.error {
            return Err(e);
        }
        let result = self.push_inner(input, output);
        if let Err(e) = result {
            self.error = Some(e);
        }
        result
    }
    fn push_inner<I: crate::Index>(
        &mut self,
        input: &[u8],
        output: &mut [I],
    ) -> Result<Progress, Error> {
        let (mut consumed, mut written) = (0, 0);
        let status = loop {
            while self.at < 3 && written < output.len() {
                let v = self.output[self.at];
                output[written] = I::from_u32(if core::mem::size_of::<I>() == 2 {
                    v & 65535
                } else {
                    v
                })?;
                self.at += 1;
                written += 1;
            }
            if self.at < 3 {
                break Status::NeedOutput;
            }
            consumed += self.input.fill(&input[consumed..]);
            if self.codes.is_empty() {
                if !self.input.complete() || consumed != input.len() {
                    return Err(Error::InvalidStream);
                }
                break Status::Done;
            }
            // Once the codebook is appended the body ends 16 bytes before the
            // window end; before that the window holds body bytes only.
            let safe_end = if self.input.appended {
                self.input.len() - 16
            } else {
                usize::MAX
            };
            let mut cursor = 0;
            let (chunks, _) = output[written..].as_chunks_mut::<3>();
            // Decode every triangle the window and the output have room for in
            // one call; with less than a whole triangle of output left, decode
            // one into the pending slot instead.
            let (done, status) = if chunks.is_empty() {
                let mut pending = [[0u32; 3]];
                let (done, status) = self.state.run(
                    &self.codes[..1],
                    &mut pending,
                    self.input.pending(),
                    &mut cursor,
                    safe_end,
                    &self.input.tail[..16],
                    self.version,
                    |slot, triangle| *slot = triangle,
                );
                self.output = pending[0];
                self.at = if done == 1 { 0 } else { 3 };
                (done, status)
            } else {
                let narrow = core::mem::size_of::<I>() == 2;
                let mut narrowing = Ok(());
                let result = self.state.run(
                    self.codes,
                    chunks,
                    self.input.pending(),
                    &mut cursor,
                    safe_end,
                    &self.input.tail[..16],
                    self.version,
                    |slot: &mut [I; 3], triangle| {
                        for (target, value) in slot.iter_mut().zip(triangle) {
                            match I::from_u32(if narrow { value & 65535 } else { value }) {
                                Ok(index) => *target = index,
                                Err(e) => narrowing = Err(e),
                            }
                        }
                    },
                );
                narrowing?;
                written += result.0 * 3;
                result
            };
            if self.input.appended && cursor > safe_end {
                return Err(Error::InvalidStream);
            }
            self.input.remove(cursor);
            self.codes = &self.codes[done..];
            match status {
                Ok(()) => {}
                Err(_) if self.input.left != 0 && self.input.len() < CAPACITY - 16 => {
                    if self.input.reclaim() {
                        continue;
                    }
                    break Status::NeedInput;
                }
                Err(e) => return Err(e),
            }
        };
        Ok(Progress {
            consumed,
            written,
            status,
        })
    }
    /// Validate EOF; all input must have arrived and output must be drained.
    ///
    /// # Errors
    /// The sticky error or `InvalidStream` for an incomplete stream.
    pub fn finish(self) -> Result<(), Error> {
        if let Some(e) = self.error {
            Err(e)
        } else if self.codes.is_empty() && self.at == 3 && self.input.complete() {
            Ok(())
        } else {
            Err(Error::InvalidStream)
        }
    }
}
