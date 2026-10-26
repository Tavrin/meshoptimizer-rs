// Scratch-only adapter for the retained r2 MeshLodBackend stage trace.
// Uses the exported comparator's Input, Replay, Output and allocating APIs.
fn chain(a: &[String]) {
    let text = std::fs::read_to_string(&a[2]).unwrap();
    let mut lines = text.lines();
    let header: Vec<_> = lines.next().unwrap().split('\t').collect();
    let raw = std::fs::read(header[0]).unwrap();
    let data: Vec<f32> = raw
        .chunks_exact(4)
        .map(|v| f32::from_le_bytes(v.try_into().unwrap()))
        .collect();
    let n: usize = header[1].parse().unwrap();
    let stride: usize = header[2].parse().unwrap();
    assert_eq!(raw.len(), n * stride);
    drop(raw);
    let p = data
        .chunks_exact(stride / 4)
        .map(|v| [v[0], v[1], v[2]])
        .collect();
    let mut x = Input {
        p,
        i: vec![],
        raw: vec![],
        vb: vec![],
        ib: vec![],
        seq: vec![],
        filters: std::array::from_fn(|_| vec![]),
        floats: vec![],
        quat: vec![],
        replay: Some(Replay {
            data,
            stride,
            target: 0,
            error: 0.5,
            options: 0,
            threshold: 1.05,
        }),
    };
    #[cfg(feature = "ours")]
    let mut w = m::Workspace::new(m::Limits {
        max_bytes: 1 << 30,
        max_work: 1 << 34,
    });
    #[cfg(any(feature = "peer7", feature = "optimesh", feature = "theirs"))]
    let mut w = ();
    let mut report = String::new();
    let mut active_vertices = header[0].to_string();
    for (index, line) in lines.enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        let op = fields[0];
        if fields.len() > 6 && fields[6] != active_vertices {
            // Positions change for Moss's aggregate-pruned packaging stages.
            // Load the captured/reconstructed buffer outside the stage timer.
            drop(std::mem::take(&mut x.replay.as_mut().unwrap().data));
            let raw = std::fs::read(fields[6]).unwrap();
            x.replay.as_mut().unwrap().data = raw
                .chunks_exact(4)
                .map(|v| f32::from_le_bytes(v.try_into().unwrap()))
                .collect();
            active_vertices = fields[6].to_string();
        }
        x.i = if fields[1] == "-" {
            vec![]
        } else {
            decode_words(&std::fs::read(fields[1]).unwrap())
        };
        let r = x.replay.as_mut().unwrap();
        r.target = fields[2].parse().unwrap();
        r.error = f32::from_bits(fields[3].parse().unwrap());
        r.options = fields[4].parse().unwrap();
        r.threshold = f32::from_bits(fields[5].parse().unwrap());
        // Same clone/output/copy boundaries in all arms. r2 saves "before"
        // inside the reorder timer and uses an allocating temporary result.
        let timer = (a[1] == "chain").then(Instant::now);
        let before = if op == "cache" || op == "overdraw" {
            Some(x.i.clone())
        } else {
            None
        };
        let output = run(op, &x, &mut w);
        if let Output::Indices(ref indices) = output {
            x.i.copy_from_slice(indices);
        }
        let ns = timer.map_or(0, |t| t.elapsed().as_nanos());
        black_box(&before);
        let bytes = output.bytes();
        let path = format!("{}.{}.bin", a[3], index);
        std::fs::write(&path, bytes).unwrap();
        report.push_str(&format!("{index}\t{op}\t{ns}\t{path}\n"));
    }
    std::fs::write(&a[3], report).unwrap();
}
