//! Baseline/candidate one-shot contracts, streaming chunking, and C++ decoding.
use candidate::{codec::*, Error, Limits, Workspace};
use std::{
    fmt::Write,
    io::{BufRead, BufReader, Write as IoWrite},
    process::{Command, Stdio},
};

#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<Vec<u8>, String>,
    destination: Vec<u8>,
    usage: (usize, u64),
}

macro_rules! observe {
    ($m:ident, $ws:expr, $count:expr, $stride:expr, $data:expr, $api:expr) => {{
        let mut destination = vec![0xa5; $count * $stride + 3];
        let result = if $api == 0 {
            $m::codec::decode_index_sequence($count, $stride, $data, $ws)
        } else {
            if $api == 2 {
                destination.truncate(($count * $stride).saturating_sub(1));
            }
            $m::codec::decode_index_sequence_into(&mut destination, $count, $stride, $data, $ws)
                .map(|_| Vec::new())
        };
        let usage = $ws.usage();
        Observation {
            result: result.map_err(|e| format!("{e:?}")),
            destination,
            usage: (usage.bytes, usage.work),
        }
    }};
}

fn hex(bytes: &[u8]) -> String {
    let mut result = String::new();
    for b in bytes {
        write!(result, "{b:02x}").unwrap();
    }
    result
}
fn unhex(s: &str) -> Vec<u8> {
    if s == "-" {
        return Vec::new();
    }
    s.as_bytes()
        .chunks_exact(2)
        .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
        .collect()
}

fn stream(
    count: usize,
    data: &[u8],
    split: usize,
    chunk: usize,
    width: usize,
    budget: u64,
) -> Result<Vec<u8>, Error> {
    let mut decoder = IndexSequenceDecoder::new(
        count,
        Limits {
            max_bytes: 0,
            max_work: budget,
        },
    )?;
    let mut output = Vec::new();
    let mut feed = |input: &[u8]| -> Result<(), Error> {
        let mut at = 0;
        loop {
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
            assert!(p.consumed <= input.len() - at && p.written <= size);
            assert!(out[p.written..].iter().all(|&v| v == 0xdead_beef));
            output.extend(out[..p.written].iter().flat_map(|v| v.to_le_bytes()));
            at += p.consumed;
            if at == input.len() {
                break;
            }
            assert!(p.consumed != 0 || p.written != 0);
        }
        Ok(())
    };
    let result = (|| {
        feed(&data[..split])?;
        for part in data[split..].chunks(chunk) {
            feed(&[])?;
            feed(part)?;
        }
        Ok(())
    })();
    if let Err(error) = result {
        assert_eq!(decoder.push(&[], &mut []), Err(error));
        assert_eq!(decoder.finish(), Err(error));
        return Err(error);
    }
    decoder.finish()?;
    Ok(output)
}

fn expected(count: usize, data: &[u8]) -> Result<Vec<u8>, Error> {
    // Early version rejection is the sole documented difference from the
    // minimum-length-first one-shot error selection, at unlimited work.
    if data
        .first()
        .is_some_and(|&b| b & 0xf0 == 0xd0 && b & 15 > 1)
    {
        Err(Error::UnsupportedVersion)
    } else {
        decode_index_sequence(count, 4, data, &mut Workspace::default())
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut cases = Vec::new();
    for line in std::fs::read_to_string(&args[1]).unwrap().lines() {
        let cols: Vec<_> = line.split_whitespace().collect();
        cases.push((
            cols[0].to_string(),
            cols[1].parse::<usize>().unwrap(),
            unhex(cols[2]),
        ));
    }
    let corpus = cases.len();
    let mut seed = 21604u64;
    // Both versions, both baselines, wrapping deltas, all varint widths and
    // boundary lengths; kept small enough to sweep every work limit.
    for i in 0..160 {
        let count = [0, 1, 2, 3, 5, 15, 16, 17, 31, 32, 33, 63, 64, 65][i % 14];
        let indices: Vec<_> = (0..count)
            .map(|j| {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                match i % 5 {
                    0 => j as u32,
                    1 => j as u32 / 3,
                    2 => 0xffff_ffff - j as u32,
                    3 => (seed >> 32) as u32,
                    _ => (seed as u32) & 1023,
                }
            })
            .collect();
        let data = baseline::codec::encode_index_sequence(
            &indices,
            baseline::codec::IndexEncoding::new((i % 2) as u8).unwrap(),
            &mut baseline::Workspace::default(),
        )
        .unwrap();
        cases.push((format!("generated-{i}"), count, data));
    }
    for groups in 1..=5 {
        for last in [0, 127, 128, 255] {
            let mut data = vec![0xd1];
            data.extend(vec![0xff; groups - 1]);
            data.push(last);
            data.extend([0x80, 255, 7, 1]);
            cases.push((format!("noncanonical-{groups}-{last}"), 1, data));
        }
    }
    let valid_cases = cases.clone();
    for (name, count, data) in &valid_cases {
        for end in 0..data.len() {
            cases.push((
                format!("{name}-truncated-{end}"),
                *count,
                data[..end].to_vec(),
            ));
        }
        for header in [0, 0xe0, 0xd2, 0xdf] {
            if !data.is_empty() {
                let mut bad = data.clone();
                bad[0] = header;
                cases.push((format!("{name}-header-{header}"), *count, bad));
            }
        }
        let mut extra = data.clone();
        extra.push(0);
        cases.push((format!("{name}-extra"), *count, extra));
    }
    let mut cpp = Command::new(&args[2])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut cpp_in = cpp.stdin.take().unwrap();
    let mut cpp_out = BufReader::new(cpp.stdout.take().unwrap());
    let mut snapshots = String::new();
    let (mut comparisons, mut chunkings, mut budget_checks, mut cpp_checks) =
        (0u64, 0u64, 0u64, 0u64);
    for (name, count, data) in &cases {
        let reference = decode_index_sequence(*count, 4, data, &mut Workspace::default());
        let encoded = if data.is_empty() {
            "-".to_owned()
        } else {
            hex(data)
        };
        writeln!(cpp_in, "{count} {encoded}").unwrap();
        cpp_in.flush().unwrap();
        let mut line = String::new();
        assert!(cpp_out.read_line(&mut line).unwrap() > 0);
        let (status, bytes) = line.trim_end().split_once(' ').unwrap_or((line.trim(), ""));
        assert_eq!(status == "0", reference.is_ok(), "C++ status: {name}");
        if let Ok(bytes_r) = &reference {
            assert_eq!(hex(bytes_r), bytes, "C++ bytes: {name}");
        }
        cpp_checks += 1;
        writeln!(
            snapshots,
            "{name}\t{count}\t{encoded}\t{}",
            match &reference {
                Ok(v) => format!("ok:{}", hex(v)),
                Err(e) => format!("{e:?}"),
            }
        )
        .unwrap();
        let stream_expected = expected(*count, data);
        for split in 0..=data.len() {
            for width in [0, 1, 3, count + 1] {
                assert_eq!(
                    stream(*count, data, split, usize::MAX, width, u64::MAX),
                    stream_expected,
                    "split {split} width {width}: {name}"
                );
                chunkings += 1;
            }
        }
        assert_eq!(
            stream(*count, data, 0, 1, 1, u64::MAX),
            stream_expected,
            "one-byte: {name}"
        );
        chunkings += 1;
        // Every budget including output reservation and one beyond the exact
        // successful charge. Check empty/one-byte feeds and backpressure.
        let mut warm_source = vec![0; count + 5];
        warm_source[0] = 0xd1;
        let total = data.len() as u64 + *count as u64 * 4;
        for work in 0..=total + 1 {
            let a = stream(*count, data, 0, usize::MAX, count + 1, work);
            let b = stream(*count, data, 0, 1, 0, work);
            assert_eq!(a, b, "stream budget {work}: {name}");
            if reference.is_ok() {
                assert_eq!(a.is_ok(), work >= total);
            }
            budget_checks += 1;
            for stride in [2, 4] {
                let footprint = count * stride;
                let mut bytes_limits =
                    vec![0, footprint.saturating_sub(1), footprint, footprint + 1];
                bytes_limits.sort_unstable();
                bytes_limits.dedup();
                for bytes in bytes_limits {
                    for api in 0..3 {
                        let mut base = baseline::Workspace::default();
                        let mut cand = Workspace::default();
                        for state in 0..3 {
                            // Warm after success, or after an injected failure.
                            if state != 0 {
                                base.set_limits(baseline::Limits::default());
                                cand.set_limits(Limits::default());
                                let warm_data = if state == 1 { &warm_source[..] } else { &[] };
                                let _ = baseline::codec::decode_index_sequence(
                                    *count, stride, warm_data, &mut base,
                                );
                                let _ = decode_index_sequence(*count, stride, warm_data, &mut cand);
                            }
                            base.set_limits(baseline::Limits {
                                max_bytes: bytes,
                                max_work: work,
                            });
                            cand.set_limits(Limits {
                                max_bytes: bytes,
                                max_work: work,
                            });
                            let a = observe!(baseline, &mut base, *count, stride, data, api);
                            let b = observe!(candidate, &mut cand, *count, stride, data, api);
                            assert_eq!(a, b, "one-shot {name} work={work} bytes={bytes} stride={stride} api={api} state={state}");
                            comparisons += 1;
                        }
                    }
                }
            }
        }
    }
    drop(cpp_in);
    assert!(cpp.wait().unwrap().success());
    std::fs::write(&args[3], snapshots).unwrap();
    println!("corpus={corpus} generated=160 noncanonical=20 total_cases={} cpp_checks={cpp_checks} chunkings={chunkings} stream_budgets={budget_checks} baseline_comparisons={comparisons} differences=0", cases.len());
}
