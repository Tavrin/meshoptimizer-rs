#![no_main]
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::{codec::*, Error, Limits, Workspace};

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }
    let count = usize::from(data[0] % 129);
    let chunk = usize::from(data[1] % 17) + 1;
    let width = usize::from(data[2] % 8) + 1;
    let source = &data[3..data.len().min(1027)];
    let expected = if source
        .first()
        .is_some_and(|&b| b & 0xf0 == 0xd0 && b & 15 > 1)
    {
        Err(Error::UnsupportedVersion)
    } else {
        decode_index_sequence(count, 4, source, &mut Workspace::default())
    };
    let mut decoder = IndexSequenceDecoder::new(
        count,
        Limits {
            max_bytes: 0,
            max_work: 2048,
        },
    )
    .unwrap();
    let mut decoded = Vec::new();
    let result = (|| {
        for input in source.chunks(chunk) {
            let mut at = 0;
            loop {
                let zero = decoder.push(&input[at..], &mut [])?;
                assert_eq!(zero.written, 0);
                at += zero.consumed;
                if at == input.len() && zero.status != Status::NeedOutput {
                    break;
                }
                let mut out = [0xdead_beef; 10];
                let p = decoder.push(&input[at..], &mut out[..width])?;
                assert!(p.written <= width && p.consumed <= input.len() - at);
                assert!(out[p.written..].iter().all(|&v| v == 0xdead_beef));
                decoded.extend(out[..p.written].iter().flat_map(|v| v.to_le_bytes()));
                at += p.consumed;
                if at == input.len() {
                    break;
                }
                assert!(p.consumed != 0 || p.written != 0);
            }
        }
        Ok(())
    })();
    let actual = match result {
        Ok(()) => decoder.finish().map(|_| decoded),
        Err(error) => {
            assert_eq!(decoder.push(&[], &mut []), Err(error));
            assert_eq!(decoder.finish(), Err(error));
            Err(error)
        }
    };
    assert_eq!(actual, expected);
});
