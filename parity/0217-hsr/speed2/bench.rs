use meshoptimizer_rs as m;
use std::{
    error::Error,
    io::{self, BufRead, Read, Write},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn word(f: &mut impl Read) -> Result<u32> {
    let mut b = [0; 4];
    f.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}
fn main() -> Result<()> {
    let a: Vec<String> = std::env::args().collect();
    let mut f = std::fs::File::open(&a[1])?;
    let nv = usize::try_from(word(&mut f)?)?;
    let ni = usize::try_from(word(&mut f)?)?;
    let mut p = Vec::with_capacity(nv);
    for _ in 0..nv {
        p.push([
            f32::from_bits(word(&mut f)?),
            f32::from_bits(word(&mut f)?),
            f32::from_bits(word(&mut f)?),
        ]);
    }
    let i = (0..ni).map(|_| word(&mut f)).collect::<Result<Vec<_>>>()?;
    let mut o = m::HiddenOptions::default();
    if let Ok(v) = std::env::var("HSR_PIXEL_SIZE") {
        o.pixel_size = v.parse()?;
    }
    if let Ok(v) = std::env::var("HSR_SUBDIVISIONS") {
        o.subdivisions = v.parse()?;
    }
    let mut w = m::Workspace::new(m::Limits {
        max_bytes: 3 << 30,
        ..m::Limits::default()
    });
    #[cfg(feature = "parallel")]
    let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build()?;
    // Parsing, warmup and output encoding are outside every measured interval.
    for line in std::iter::once(Ok(String::from("warm"))).chain(io::stdin().lock().lines()) {
        let line = line?;
        if line == "stop" {
            break;
        }
        let start = Instant::now();
        #[cfg(feature = "parallel")]
        let r = pool
            .install(|| m::remove_hidden_triangles(&i, m::Positions::from_packed(&p), o, &mut w));
        #[cfg(not(feature = "parallel"))]
        let r = m::remove_hidden_triangles(&i, m::Positions::from_packed(&p), o, &mut w);
        let r = r.map_err(|error| io::Error::other(format!("{error:?}")))?;
        let seconds = start.elapsed().as_secs_f64();
        std::fs::write(
            &a[2],
            r.indices
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<_>>(),
        )?;
        println!("{{\"phase\":\"{line}\",\"seconds\":{seconds:.9},\"input\":{},\"output\":{},\"duplicate\":{},\"back_to_back\":{},\"occluded\":{},\"work\":{},\"bytes\":{}}}", r.report.input_triangles, r.report.output_triangles, r.report.duplicate, r.report.enclosed_back_to_back, r.report.occluded, w.usage().work, w.usage().bytes);
        io::stdout().flush()?;
    }
    Ok(())
}
