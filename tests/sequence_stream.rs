use meshoptimizer_rs::{codec::*, Error, Limits, Workspace};

fn encode(indices: &[u32], version: u8) -> Vec<u8> {
    encode_index_sequence(
        indices,
        IndexEncoding::new(version).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap()
}

fn stream(
    count: usize,
    source: &[u8],
    split: usize,
    chunk: usize,
    width: usize,
    limits: Limits,
) -> Result<Vec<u32>, Error> {
    let mut decoder = IndexSequenceDecoder::new(count, limits)?;
    let mut result = Vec::new();
    let mut feed = |input: &[u8]| -> Result<(), Error> {
        let mut at = 0;
        loop {
            // Exercise zero output without getting stuck: retry NeedOutput
            // with one slot. Neither call may touch the caller's tail.
            if width == 0 {
                let p = decoder.push(&input[at..], &mut [])?;
                assert_eq!(p.written, 0);
                at += p.consumed;
                if at == input.len() && p.status != Status::NeedOutput {
                    break;
                }
            }
            let size = width.max(1);
            let mut out = vec![0xdead_beef; size + 2];
            let p = decoder.push(&input[at..], &mut out[..size])?;
            assert!(p.consumed <= input.len() - at);
            assert!(p.written <= size);
            assert!(out[p.written..].iter().all(|&v| v == 0xdead_beef));
            result.extend_from_slice(&out[..p.written]);
            at += p.consumed;
            if at == input.len() {
                break;
            }
            assert!(p.consumed != 0 || p.written != 0);
        }
        Ok(())
    };
    let run = (|| {
        feed(&source[..split])?;
        for input in source[split..].chunks(chunk) {
            feed(&[])?;
            feed(input)?;
        }
        Ok(())
    })();
    if let Err(error) = run {
        assert_eq!(decoder.push(&[], &mut []), Err(error));
        assert_eq!(decoder.finish(), Err(error));
        return Err(error);
    }
    decoder.finish()?;
    Ok(result)
}

#[test]
fn stream_every_split_one_byte_and_short_output() {
    let counts: &[usize] = if cfg!(miri) {
        &[0, 2, 6]
    } else {
        &[0, 1, 2, 6, 31, 64, 129]
    };
    for &count in counts {
        let indices: Vec<_> = (0..count)
            .map(|i| match i % 4 {
                0 => i as u32,
                1 => 1000 + i as u32,
                2 => u32::MAX - i as u32,
                _ => 0x2345_6789 + i as u32,
            })
            .collect();
        for version in [0, 1] {
            let data = encode(&indices, version);
            // Large deltas may truncate in the sequence format, so compare
            // to the existing decoder rather than the original indices.
            let bytes = decode_index_sequence(count, 4, &data, &mut Workspace::default()).unwrap();
            let expected: Vec<_> = bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|b| u32::from_le_bytes(*b))
                .collect();
            for split in 0..=data.len() {
                for width in [0, 1, 3, count + 1] {
                    assert_eq!(
                        stream(count, &data, split, usize::MAX, width, Limits::default()),
                        Ok(expected.clone())
                    );
                }
            }
            assert_eq!(
                stream(count, &data, 0, 1, 1, Limits::default()),
                Ok(expected)
            );
        }
    }
}

#[test]
fn stream_backpressure_partial_varint_and_done_are_explicit() {
    // Progress is non-exhaustive, so compare its fields.
    fn step(
        decoder: &mut IndexSequenceDecoder,
        input: &[u8],
        output: &mut [u32],
    ) -> (usize, usize, Status) {
        let p: Progress = decoder.push(input, output).unwrap();
        (p.consumed, p.written, p.status)
    }
    let mut decoder = IndexSequenceDecoder::new(1, Limits::default()).unwrap();
    assert_eq!(step(&mut decoder, &[], &mut []), (0, 0, Status::NeedInput));
    assert_eq!(
        step(&mut decoder, &[0xd1, 0x80], &mut []),
        (1, 0, Status::NeedOutput)
    );
    let mut output = [42; 2];
    assert_eq!(
        step(&mut decoder, &[0x80], &mut output[..1]),
        (1, 0, Status::NeedInput)
    );
    assert_eq!(output, [42; 2]);
    assert_eq!(step(&mut decoder, &[], &mut []), (0, 0, Status::NeedOutput));
    assert_eq!(
        step(&mut decoder, &[1], &mut output[..1]),
        (1, 1, Status::NeedInput)
    );
    assert_eq!(output, [32, 42]);
    assert_eq!(
        step(&mut decoder, &[9, 8, 7, 6], &mut []),
        (4, 0, Status::Done)
    );
    assert_eq!(step(&mut decoder, &[], &mut []), (0, 0, Status::Done));
    assert_eq!(decoder.finish(), Ok(()));
}

#[test]
fn stream_noncanonical_varints_and_opaque_padding_match_one_shot() {
    for groups in 1..=5 {
        for last in [0, 127, 128, 255] {
            let mut data = vec![0xd0];
            data.extend_from_slice(&vec![0xff; groups - 1]);
            data.push(last);
            data.extend_from_slice(&[0x80, 255, 7, 1]);
            let expected = decode_index_sequence(1, 4, &data, &mut Workspace::default());
            let actual = stream(1, &data, 0, 1, 1, Limits::default())
                .map(|v| v.iter().flat_map(|v| v.to_le_bytes()).collect());
            assert_eq!(actual, expected, "{data:?}");
        }
    }
}

#[test]
fn stream_malformed_headers_truncation_and_padding_are_terminal() {
    let valid = encode(&[0, 1, 51, 2, 49, 1000], 1);
    let mut cases: Vec<Vec<u8>> = (0..valid.len()).map(|end| valid[..end].to_vec()).collect();
    for header in [0, 0xe0, 0xd2, 0xdf] {
        let mut bad = valid.clone();
        bad[0] = header;
        cases.push(bad);
    }
    let mut extra = valid.clone();
    extra.push(0);
    cases.push(extra);
    // Partial varint and missing padding; a continuation on byte five is
    // accepted as a complete value by upstream, then rejected for missing tail.
    cases.push(vec![0xd1, 0x80, 0x80, 0x80, 0x80, 0x80]);
    for data in cases {
        let expected = decode_index_sequence(6, 4, &data, &mut Workspace::default()).unwrap_err();
        for split in 0..=data.len() {
            assert_eq!(
                stream(6, &data, split, 1, 1, Limits::default()),
                Err(expected)
            );
        }
    }
    // Documented earlier error: unsupported version before minimum length.
    assert_eq!(
        stream(6, &[0xd2], 0, 1, 1, Limits::default()),
        Err(Error::UnsupportedVersion)
    );
    let mut decoder = IndexSequenceDecoder::new(0, Limits::default()).unwrap();
    decoder.push(&[0xd0, 0, 0, 0, 0], &mut []).unwrap();
    assert_eq!(decoder.push(&[0], &mut []), Err(Error::InvalidStream));
    assert_eq!(decoder.finish(), Err(Error::InvalidStream));
}

#[test]
fn stream_work_is_whole_stream_and_independent_of_chunks() {
    let data = encode(&[1, 1000, 2, 999], 1);
    let work = 16 + data.len() as u64;
    for budget in 0..=work + 1 {
        for bytes in [0, 1, usize::MAX] {
            let limits = Limits {
                max_bytes: bytes,
                max_work: budget,
            };
            let expected = stream(4, &data, 0, usize::MAX, 5, limits);
            assert_eq!(expected.is_ok(), budget >= work);
            for split in 0..=data.len() {
                assert_eq!(stream(4, &data, split, 1, 0, limits), expected);
            }
        }
    }
    for count in [usize::MAX, usize::MAX / 4 + 1, isize::MAX as usize / 4 + 1] {
        assert_eq!(
            IndexSequenceDecoder::new(count, Limits::default()).err(),
            Some(Error::SizeOverflow)
        );
    }
}

#[test]
fn stream_finish_checks_count_and_each_padding_boundary() {
    let data = encode(&[3, 5], 1);
    for count in [0, 1, 2, 3] {
        for end in 0..=data.len() {
            let expected = decode_index_sequence(count, 4, &data[..end], &mut Workspace::default());
            let actual = stream(count, &data[..end], 0, 1, 1, Limits::default())
                .map(|v| v.iter().flat_map(|v| v.to_le_bytes()).collect());
            assert_eq!(actual, expected);
        }
    }
    assert_eq!(
        IndexSequenceDecoder::new(0, Limits::default())
            .unwrap()
            .finish(),
        Err(Error::InvalidStream)
    );
}

/// How one chunked decode ended. A `push` error carries the absolute source
/// range that the failing call held, so the failing byte can be located.
#[derive(Debug, PartialEq)]
enum Outcome {
    Ok(Vec<u32>),
    New(Error),
    Push(Error, std::ops::Range<usize>),
    Finish(Error),
}

const SENTINEL: u32 = 0xdead_beef;

/// Feed `source` cut at `cuts`, with `capacity` output slots per call. A
/// zero-capacity `NeedOutput` is retried once with zero (it must be a no-op)
/// and then with one slot. Checks every per-call invariant on the way.
fn drive(count: usize, source: &[u8], cuts: &[usize], capacity: usize, limits: Limits) -> Outcome {
    let mut decoder = match IndexSequenceDecoder::new(count, limits) {
        Ok(decoder) => decoder,
        Err(error) => return Outcome::New(error),
    };
    let mut result = Vec::new();
    let mut calls = 0;
    let mut start = 0;
    for end in cuts.iter().copied().chain([source.len()]) {
        let mut at = start;
        let mut cap = capacity;
        loop {
            calls += 1;
            assert!(calls <= 3 * (source.len() + count) + 8, "no progress");
            let input = &source[at..end];
            let mut out = [SENTINEL; 16];
            let p = match decoder.push(input, &mut out[..cap]) {
                Ok(p) => p,
                Err(error) => {
                    assert_eq!(decoder.push(&[], &mut [0]), Err(error));
                    assert_eq!(decoder.finish(), Err(error));
                    return Outcome::Push(error, at..end);
                }
            };
            assert!(p.consumed <= input.len() && p.written <= cap);
            assert!(out[p.written..].iter().all(|&v| v == SENTINEL));
            // Each index needs its final varint byte from this call.
            assert!(p.written <= p.consumed);
            match p.status {
                Status::NeedInput | Status::Done => assert_eq!(p.consumed, input.len()),
                Status::NeedOutput => assert_eq!(p.written, cap),
                _ => panic!("unknown status"),
            }
            if cap != 0 && !input.is_empty() {
                assert!(p.consumed != 0, "a call that can act must consume");
            }
            result.extend_from_slice(&out[..p.written]);
            at += p.consumed;
            cap = capacity;
            if p.status == Status::NeedOutput && p.written == 0 {
                // Zero output stops before any varint byte: only the header
                // may have been taken, and a retry changes nothing.
                assert!(p.consumed <= usize::from(at == 1 && start == 0));
                let again = decoder.push(&source[at..end], &mut []).unwrap();
                assert_eq!((again.consumed, again.written), (0, 0));
                assert_eq!(again.status, Status::NeedOutput);
                cap = 1;
            }
            if at == end && p.status != Status::NeedOutput {
                break;
            }
        }
        start = end;
    }
    match decoder.finish() {
        Ok(()) => Outcome::Ok(result),
        Err(error) => Outcome::Finish(error),
    }
}

/// One-shot `decode_index_sequence_into` with the same limits, plus its work.
fn one_shot(count: usize, source: &[u8], limits: Limits) -> (Result<Vec<u32>, Error>, u64) {
    let mut workspace = Workspace::new(limits);
    let mut bytes = vec![0u8; count * 4];
    let result = decode_index_sequence_into(&mut bytes, count, 4, source, &mut workspace);
    let usage = workspace.usage();
    assert_eq!(usage.bytes, 0);
    let values = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| u32::from_le_bytes(*b))
        .collect();
    (result.map(|()| values), usage.work)
}

fn sequence_cases() -> Vec<(usize, Vec<u8>)> {
    let mut bases: Vec<(usize, Vec<u8>)> = Vec::new();
    for version in [0, 1] {
        for indices in [
            &[][..],
            &[5],
            &[0, 1, 51, 2, 49, 1000],
            &[0, u32::MAX, 0x8000_0000, 1, 7],
        ] {
            bases.push((indices.len(), encode(indices, version)));
        }
    }
    // Five-byte varints at u32 overflow: high bits discarded, and a
    // continuation bit on byte five still ends the varint.
    for fifth in [0x7f, 0x8f, 0xff] {
        bases.push((1, vec![0xd1, 0xff, 0xff, 0xff, 0xff, fifth, 9, 9, 9, 9]));
        bases.push((2, vec![0xd0, 0x81, 0xff, 0xff, 0xff, fifth, 2, 0, 0, 0, 0]));
    }
    let mut cases = Vec::new();
    for (count, data) in bases {
        // Every truncation, including a missing padding byte.
        for end in 0..data.len() {
            cases.push((count, data[..end].to_vec()));
        }
        let mut extra = data.clone();
        extra.push(0);
        cases.push((count, extra));
        for header in [0xc1, 0xd2, 0xdf] {
            let mut bad = data.clone();
            bad[0] = header;
            cases.push((count, bad));
        }
        if count != 0 {
            // The last varint runs on into the padding.
            let mut swallow = data.clone();
            swallow[data.len() - 5] |= 0x80;
            cases.push((count, swallow));
            cases.push((count - 1, data.clone()));
        }
        cases.push((count + 1, data.clone()));
        cases.push((count, data));
    }
    cases
}

#[test]
fn stream_any_two_cuts_and_any_capacity_match_one_shot() {
    let mut checked = 0usize;
    for (count, data) in sequence_cases() {
        let len = data.len();
        let whole = 4 * count as u64 + len as u64;
        let (unlimited, _) = one_shot(count, &data, Limits::default());
        let mut limit_set = vec![Limits::default()];
        let tight = whole.saturating_sub(1);
        for max_work in [0, 4 * count as u64, tight, whole, whole + 1] {
            for max_bytes in [0, usize::MAX] {
                limit_set.push(Limits {
                    max_bytes,
                    max_work,
                });
            }
        }
        for limits in limit_set {
            // Byte-at-a-time with one slot is the reference chunking.
            let bytewise: Vec<usize> = (1..len).collect();
            let reference = drive(count, &data, &bytewise, 1, limits);
            if let Outcome::Push(_, range) = &reference {
                assert_eq!(range.len(), 1, "{data:?}");
            }
            let same = |actual: Outcome| match (&reference, &actual) {
                (Outcome::Push(e, r), Outcome::Push(a, w)) => {
                    assert_eq!(e, a, "{count} {data:?} {limits:?}");
                    assert!(w.contains(&r.start), "{w:?} {r:?} {data:?}");
                }
                _ => assert_eq!(actual, reference, "{count} {data:?} {limits:?}"),
            };
            // Whole-stream equivalence with one-shot, and its documented gaps.
            let (expected, work) = one_shot(count, &data, limits);
            let streamed = match &reference {
                Outcome::Ok(v) => Ok(v.clone()),
                Outcome::New(e) | Outcome::Push(e, _) | Outcome::Finish(e) => Err(*e),
            };
            match expected {
                Ok(v) => {
                    assert_eq!(streamed, Ok(v));
                    assert_eq!(work, whole);
                }
                // One-shot charges the whole source up front; the stream
                // charges consumed bytes, so a malformed stream may report
                // its own error first. A valid stream must hit the limit.
                Err(Error::LimitExceeded) => {
                    assert!(limits.max_work < whole);
                    let own = unlimited.clone().err();
                    assert!(
                        streamed == Err(Error::LimitExceeded) || streamed.clone().err() == own,
                        "{streamed:?} {own:?} {data:?}"
                    );
                    if own.is_none() {
                        assert_eq!(streamed, Err(Error::LimitExceeded));
                    }
                }
                Err(e) => {
                    // Documented: the version is reported before one-shot's
                    // minimum-length check rejects a short stream.
                    let early = e == Error::InvalidStream
                        && len < count + 5
                        && data
                            .first()
                            .is_some_and(|&h| h & 0xf0 == 0xd0 && h & 0x0f > 1);
                    let want = if early { Error::UnsupportedVersion } else { e };
                    assert_eq!(streamed, Err(want), "{count} {data:?} {limits:?}");
                }
            }
            let mut capacities = vec![0, 1, 2, count];
            capacities.sort_unstable();
            capacities.dedup();
            // Every pair of cuts under no limit and one unit short of it.
            let pairs = !cfg!(miri)
                && (limits == Limits::default()
                    || (limits.max_work == tight && limits.max_bytes == 0));
            for a in 0..=len {
                for &capacity in &capacities {
                    same(drive(count, &data, &[a], capacity, limits));
                    checked += 1;
                    if pairs {
                        for b in a..=len {
                            same(drive(count, &data, &[a, b], capacity, limits));
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(checked > 100_000 || cfg!(miri), "{checked}");
}
