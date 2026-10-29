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
    let mut decoder = IndexSequenceDecoder::new(1, Limits::default()).unwrap();
    assert_eq!(
        decoder.push(&[], &mut []),
        Ok(Progress {
            consumed: 0,
            written: 0,
            status: Status::NeedInput
        })
    );
    assert_eq!(
        decoder.push(&[0xd1, 0x80], &mut []),
        Ok(Progress {
            consumed: 1,
            written: 0,
            status: Status::NeedOutput
        })
    );
    let mut output = [42; 2];
    assert_eq!(
        decoder.push(&[0x80], &mut output[..1]),
        Ok(Progress {
            consumed: 1,
            written: 0,
            status: Status::NeedInput
        })
    );
    assert_eq!(output, [42; 2]);
    assert_eq!(
        decoder.push(&[], &mut []),
        Ok(Progress {
            consumed: 0,
            written: 0,
            status: Status::NeedOutput
        })
    );
    assert_eq!(
        decoder.push(&[1], &mut output[..1]),
        Ok(Progress {
            consumed: 1,
            written: 1,
            status: Status::NeedInput
        })
    );
    assert_eq!(output, [32, 42]);
    assert_eq!(
        decoder.push(&[9, 8, 7, 6], &mut []),
        Ok(Progress {
            consumed: 4,
            written: 0,
            status: Status::Done
        })
    );
    assert_eq!(
        decoder.push(&[], &mut []),
        Ok(Progress {
            consumed: 0,
            written: 0,
            status: Status::Done
        })
    );
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
