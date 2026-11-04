//! Benchmark harness: prepares shared input, times both cluster-DAG builders,
//! and evaluates their cuts. See `README.md`.
#[cfg(feature = "bevy")]
mod bevy_build;
mod bevyfile;
mod camera;
mod cut;
mod ours;
mod report;
mod source;

use report::Json;
use source::Source;
use std::{fs, path::Path};

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}
fn num<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    flag(args, name).and_then(|v| v.parse().ok()).unwrap_or(default)
}
fn positional(args: &[String]) -> Vec<&String> {
    let mut out = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
        } else if a.starts_with("--") {
            skip = true;
        } else {
            out.push(a);
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("prep") => prep(&args[1..]),
        Some("build") => build(&args[1..]),
        Some("eval") => eval(&args[1..]),
        _ => Err("usage: vgbench prep|build|eval ... (see README.md)".into()),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

/// `prep <mesh.ply|gltf> <out.vgm> [--extent 2.0]`
fn prep(args: &[String]) -> Result<(), String> {
    let p = positional(args);
    let [input, output] = p.as_slice() else {
        return Err("prep <mesh> <out.vgm> [--extent metres]".into());
    };
    let mut s = source::read(Path::new(input.as_str()))?;
    s.fit(num(args, "--extent", 2.0f32));
    s.save(Path::new(output.as_str()))?;
    println!(
        "{}",
        Json::new().text("kind", "prep").with("vertices", s.positions.len()).with("triangles", s.triangles())
    );
    Ok(())
}

fn load(p: &[&String]) -> Result<(Source, String), String> {
    let path = p.get(1).ok_or("missing .vgm path")?;
    let name = Path::new(path.as_str()).file_stem().and_then(|s| s.to_str()).unwrap_or("mesh").to_string();
    Ok((Source::load(Path::new(path.as_str()))?, name))
}

/// `build <ours|bevy> <in.vgm> --threads N [--out file] [--quant 4]`: one timed build.
fn build(args: &[String]) -> Result<(), String> {
    let p = positional(args);
    let builder = p.first().ok_or("missing builder")?.as_str().to_string();
    let (src, name) = load(&p)?;
    let threads: usize = num(args, "--threads", 1);
    let label =
        Json::new().text("kind", "build").text("builder", &builder).text("mesh", &name).with("threads", threads);
    let line = match builder.as_str() {
        "ours" => {
            let (o, t) = ours::build(&src, threads)?;
            let (file, write_ms) = o.write_file(&src)?;
            if let Some(out) = flag(args, "--out") {
                fs::write(out, &file).map_err(|e| e.to_string())?;
            }
            label
                .with("build_ms", t.dag_ms + t.hierarchy_ms)
                .with("dag_ms", t.dag_ms)
                .with("hierarchy_ms", t.hierarchy_ms)
                .with("serialise_ms", write_ms)
                .with("file_bytes", file.len())
        }
        #[cfg(feature = "bevy")]
        "bevy" => {
            let b = bevy_build::build(&src, threads, num(args, "--quant", 4u8))?;
            if let Some(out) = flag(args, "--out") {
                fs::write(out, &b.file).map_err(|e| e.to_string())?;
            }
            label.with("build_ms", b.build_ms).with("serialise_ms", b.save_ms).with("file_bytes", b.file.len())
        }
        other => return Err(format!("unknown or disabled builder {other}")),
    };
    println!("{}", line.with("triangles", src.triangles()).with("vmhwm_kib", report::peak_rss_kib()));
    Ok(())
}

/// `eval <ours|bevy> <in.vgm> [--artifact file] [--dirs 64] [--jobs 8] [--samples 200000]`
fn eval(args: &[String]) -> Result<(), String> {
    let p = positional(args);
    let builder = p.first().ok_or("missing builder")?.as_str().to_string();
    let (src, name) = load(&p)?;
    let label = Json::new().text("builder", &builder).text("mesh", &name);
    let settings = cut::Settings {
        dirs: num(args, "--dirs", 64),
        spans: vec![64.0, 128.0, 256.0, 512.0, 1024.0, 2048.0],
        // The first entry selects the source clusters: it isolates the reference
        // format's position quantisation from simplification error.
        thresholds: vec![1e-9, 0.5, 1.0, 2.0, 4.0],
        samples: num(args, "--samples", 200_000.0),
        jobs: num(args, "--jobs", 8),
    };
    let (dag, shape): (Box<dyn cut::Dag>, String) = match builder.as_str() {
        "ours" => {
            let (o, _) = ours::build(&src, settings.jobs)?;
            let shape = ours::shape(&o, &src, &label)?;
            (Box::new(o) as Box<dyn cut::Dag>, shape)
        }
        "bevy" => {
            let path = flag(args, "--artifact").ok_or("--artifact required")?;
            let file = fs::read(&path).map_err(|e| format!("{path}: {e}"))?;
            let b = bevyfile::Bevy::parse(&file)?;
            let shape = b.shape(&file, src.triangles(), &label);
            (Box::new(b) as Box<dyn cut::Dag>, shape)
        }
        other => return Err(format!("unknown builder {other}")),
    };
    println!("{shape}");
    let clusters = shape_clusters(&shape);
    for (check, pass, detail) in cut::self_checks(dag.as_ref(), &src, clusters) {
        println!(
            "{}",
            label.clone().text("kind", "check").text("name", &check).with("pass", pass).text("detail", &detail)
        );
    }
    if args.iter().any(|a| a == "--no-cuts") {
        return Ok(());
    }
    for line in cut::evaluate(dag.as_ref(), &src, &settings, &label)? {
        println!("{line}");
    }
    Ok(())
}

fn shape_clusters(shape: &str) -> usize {
    shape
        .split("\"clusters\":")
        .nth(1)
        .and_then(|s| s.split([',', '}']).next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(usize::MAX)
}
