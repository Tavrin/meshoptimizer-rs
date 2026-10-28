fn main() {
    let args: Vec<_> = env::args().collect();
    let mut checks = 0;
    if args.get(1).is_some_and(|x| x == "fuzz") {
        let seed = args[2].parse().unwrap();
        let mut rng = Rng(seed);
        for i in 0..64 {
            let n = 3 + (rng.next() % 126) as usize;
            let mut c = mesh((rng.next() % 6) as usize, n);
            c.layout = (rng.next() % 8) as u8;
            c.target = (rng.next() as usize) % (n + 1);
            c.flags = i % 2 == 0;
            c.weight = [0., 0.1, 1., f32::MAX, f32::INFINITY][(rng.next() % 5) as usize];
            c.error = [0., 0.01, 0.5, 1., f32::MAX, f32::NAN][(rng.next() % 6) as usize];
            // Keep half the inputs valid so hash and union kernels, colored
            // reservoirs and their tight-budget fallbacks remain reachable.
            if i % 2 == 0 {
                for p in &mut c.p {
                    for v in p {
                        *v = if i % 4 == 0 { f32::from_bits(rng.next()) }
                             else { (rng.next() % 128) as f32 * 0.03125 };
                    }
                }
            }
            if i % 9 == 0 { c.ix[0] = u32::MAX; }
            checks += compare(&c, if i % 2 == 0 { 8 } else { 0 }, false,
                if i % 5 == 0 { (rng.next() % 4096) as usize } else { 1 << 26 },
                i % 2 == 0);
        }
        println!("seed={seed} observations={checks}");
        return;
    }
    for kind in 0..6 {
        for layout in 0..8 {
            for flags in [false, true] {
                for target in [0, 2, 5] {
                    let mut c = mesh(kind, 5);
                    c.layout = layout;
                    c.flags = flags;
                    c.target = target;
                    for error in [0., 0.5, 1.] {
                        c.error = error;
                        for options in [0, 8] {
                            checks += compare(&c, options, true, 1 << 26, false);
                        }
                    }
                }
            }
        }
    }
    // Hash collisions, welded vertices, disconnected components and deep
    // descending union chains. Each limit is swept through success plus one.
    for n in [0, 3, 16, 65] {
        for kind in 0..7 {
            let mut c = mesh(kind.min(5), n);
            if kind == 6 {
                c.ix = (1..n).rev().flat_map(|i| [i as u32, i as u32 - 1, i as u32 - 1]).collect();
            }
            c.target = n / 2;
            for lean in [false, true] {
                checks += compare(&c, 8, true, 1 << 26, lean);
            }
        }
    }
    for bits in [0x7f7fffff, 0xff7fffff, 0x7fc12345, 0x7f800000, 0x80000000,
                 1, 0x001fffff, 0x00200000, 0x00200001, 0x00800000] {
        for layout in 0..8 {
            let mut c = mesh(2, 5);
            c.layout = layout;
            c.target = 2;
            c.p[0][0] = f32::from_bits(bits);
            for bytes in [0, 256, 1024, 1 << 26] {
                checks += compare(&c, 8, true, bytes, false);
            }
        }
    }
    println!("PASS observations={checks}");
}
