//! Compare output, typed results (Debug names), complete Usage and failure tails.
use std::{fs, path::Path};
#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(), String>,
    output: Vec<u32>,
    usage: (u64, usize),
}
macro_rules! observe {
    ($m:ident, $ix:expr, $v:expr, $family:expr, $api:expr, $fuel:expr, $bytes:expr, $size:expr) => {{
        use $m as m;
        let ix: &[u32] = $ix;
        let v = $v;
        let family = $family;
        let mut ws = m::Workspace::new(m::Limits { max_work: $fuel, max_bytes: $bytes });
        let alloc = |ws: &mut m::Workspace| match family {
            0 => m::optimize_vertex_cache(ix, v, ws),
            1 => m::optimize_vertex_cache_strip(ix, v, ws),
            _ => m::optimize_vertex_cache_fifo(ix, v, $size, ws),
        };
        // Both cold and retained scratch; the second observation is independent
        // of first success, so allocation/work failures are also exercised warm.
        let mut observations = Vec::new();
        for _ in 0..2 {
            let mut output = Vec::new();
            let result = match $api {
                0 => alloc(&mut ws).map(|x| output = x),
                1 | 3 => {
                    output = vec![0xdeadbeef; if $api == 1 { ix.len()+7 } else { ix.len().saturating_sub(1) }];
                    match family {
                        0 => m::optimize_vertex_cache_into(&mut output, ix, v, &mut ws),
                        1 => m::optimize_vertex_cache_strip_into(&mut output, ix, v, &mut ws),
                        _ => m::optimize_vertex_cache_fifo_into(&mut output, ix, v, $size, &mut ws),
                    }
                }
                _ => {
                    output = ix.to_vec();
                    match family {
                        0 => m::optimize_vertex_cache_in_place(&mut output, v, &mut ws),
                        1 => m::optimize_vertex_cache_strip_in_place(&mut output, v, &mut ws),
                        _ => m::optimize_vertex_cache_fifo_in_place(&mut output, v, $size, &mut ws),
                    }
                }
            };
            let u = ws.usage();
            observations.push(Observation { result: result.map_err(|e| format!("{e:?}")), output, usage: (u.work,u.bytes) });
        }
        observations
    }};
}
fn check(ix: &[u32], v: usize, fuels: &[u64], bytes: &[usize], sizes: &[u32]) -> usize {
    let mut checks=0;
    for family in 0..3 {
        for &size in if family==2 { sizes } else { &[16] } {
            for api in 0..4 {
                for &fuel in fuels {
                    for &bytes in bytes {
                        let a=observe!(baseline,ix,v,family,api,fuel,bytes,size);
                        let b=observe!(candidate,ix,v,family,api,fuel,bytes,size);
                        assert_eq!(a,b,"v={v} ix={ix:?} family={family} api={api} fuel={fuel} bytes={bytes} size={size}");
                        checks+=a.len();
                    }
                }
            }
        }
    }
    checks
}
fn u32_at(b: &[u8], at: usize) -> u32 { u32::from_le_bytes(b[at..at+4].try_into().unwrap()) }
fn frozen(path: &Path) -> (Vec<u32>,usize) {
    let b=fs::read(path).unwrap();let v=u32_at(&b,0) as usize;let n=u32_at(&b,4) as usize;
    let ix=b[8+v*12..8+v*12+n*4].chunks_exact(4).map(|b|u32_at(b,0)).collect();(ix,v)
}
struct Rng(u64);
impl Rng { fn next(&mut self) -> u32 { self.0^=self.0<<13; self.0^=self.0>>7; self.0^=self.0<<17;self.0 as u32 } }
// Exact legacy cache generator, including the otherwise-unused geometry/layout
// RNG draws; only fuel is widened to include unlimited and exact-success paths.
fn seeds(seed: u64, count: usize, widened: bool) -> usize {
    let mut rng=Rng(seed);let mut checks=0;
    for execution in 0..count {
        let v=(rng.next()%65) as usize;let n=(rng.next()%257) as usize;
        for _ in 0..v*3 {rng.next();}
        let mut ix=vec![0;n];
        for x in &mut ix { *x=if v==0 {0} else if execution.is_multiple_of(5) {rng.next()} else {rng.next()%v as u32}; }
        if !execution.is_multiple_of(3) {ix.truncate(n/3*3);}
        let fuel=if execution.is_multiple_of(7) {u64::from(rng.next()%5000)} else {1<<20};
        let bytes=if execution.is_multiple_of(11) {(rng.next()%4096) as usize} else {1<<20};
        rng.next(); // threshold or index into threshold choices
        rng.next();rng.next(); // stride and offset
        let mut fuels=vec![fuel];
        if widened {fuels.push(u64::MAX);}
        checks+=check(&ix,v,&fuels,&[bytes],&[0,2,3,16,u32::MAX]);
        for _ in 0..10 {rng.next();}
    }
    checks
}
fn main() {
    let args:Vec<_>=std::env::args().collect();
    if args.get(1).is_some_and(|s|s=="fuzz") {
        let seed=args[2].parse().unwrap();let n=seeds(seed,2048,true);
        println!("PASS widened cache/strip/FIFO seed={seed} exact observations={n}");return;
    }
    if args.get(1).is_some_and(|s|s=="fifo-seeds") {
        let mut checks=0;
        for entry in fs::read_dir(&args[2]).unwrap() {
            let p=entry.unwrap().path();
            if p.extension().is_none_or(|e|e!="input") {continue;}
            let b=fs::read(&p).unwrap();
            assert_eq!(&b[..4],b"MO02");
            let v=u32_at(&b,8) as usize;let n=u32_at(&b,12) as usize;
            // Mutated headers that the transport rejects before any library call.
            if v.checked_mul(12).and_then(|x|x.checked_add(56+n.checked_mul(4)?)).is_none_or(|end|end>b.len()) {continue;}
            let ix:Vec<_>=b[56+v*12..56+v*12+n*4].chunks_exact(4).map(|b|u32_at(b,0)).collect();
            checks+=check(&ix,v,&[0,10,1<<34,u64::MAX],&[0,1<<30,usize::MAX],&[u32_at(&b,44)]);
        }
        println!("PASS legacy FIFO seed Usage/results/output {checks} observations");return;
    }
    let directory=Path::new(&args[1]);let mut checks=0;
    for name in ["tiny-smooth","medium-smooth","medium-seams","medium-sparse","million-smooth"] {
        let (ix,v)=frozen(&directory.join(format!("{name}.bin")));
        let full=observe!(baseline,&ix,v,0,0,u64::MAX,usize::MAX,16);
        let work=full[0].usage.0;let bytes=full[0].usage.1;
        checks+=check(&ix,v,&[u64::MAX,work.saturating_sub(1),work,work+1],&[usize::MAX],&[3,16]);
        checks+=check(&ix,v,&[u64::MAX],&[0,bytes.saturating_sub(1),bytes],&[3,16]);
        println!("PASS frozen {name} observations={checks}");
    }
    checks+=seeds(20261002,4096,false);
    // The last two maximize cursor stops: isolated degenerate faces, so every
    // later current vertex and face comes from the fallback cursor.
    for (ix,v) in [(vec![],0),(vec![0,0,0],1),(vec![0,1,2,0,2,1],3),(vec![0,0,1,0,1,1,1,1,1],5),(vec![0,1,2,3,4,5,0,2,1,6,7,8],12),
        ((0..8).flat_map(|i|[i,i,i]).collect(),8),((0..6).flat_map(|i|[2*i,2*i+1,2*i]).collect(),12)] {
        let mut max=0;
        for family in 0..3 {
            for size in [3,16,u32::MAX] {
                let full=observe!(baseline,&ix,v,family,0,u64::MAX,usize::MAX,size);
                max=max.max(full[0].usage.0);
            }
        }
        let mut fuels:Vec<_>=(0..=max+1).chain([u64::MAX]).collect();
        let mut counts=vec![0usize;v];for &i in &ix {counts[i as usize]+=1;}
        let max_live=counts.into_iter().max().unwrap_or(0);
        let n=ix.len();let f=n/3;
        // Standard dispatch after topology and all initialization charges.
        // FIFO kept 0.2.9's code (NO-GAIN revert); its old dispatch fuels stay.
        for bound in [3*n+3*v+2*f+f*(22*max_live+38),6*n+5*v] {
            fuels.extend([bound.saturating_sub(1) as u64,bound as u64,bound as u64+1]);
        }
        checks+=check(&ix,v,&fuels,&[usize::MAX,0,47,256,1024],&[0,2,3,16,u32::MAX]);
    }
    // Error precedence for malformed topology, invalid references and sizes.
    for ix in [vec![0],vec![0,1],vec![0,1,3],vec![u32::MAX,0,1]] {
        checks+=check(&ix,3,&[0,1,2,3,4,5,20,u64::MAX],&[0,usize::MAX],&[0,2,3,16]);
    }
    for v in [u32::MAX as usize,usize::MAX] {
        checks+=check(&[],v,&[0,u64::MAX],&[0,usize::MAX],&[0,3,16]);
    }
    let padded=[99,99,0,1,2,0,2,1,99];
    checks+=check(&padded[2..8],3,&[0,1,25,100,u64::MAX],&[0,1024,usize::MAX],&[3,16]);
    println!("PASS {checks} exact observations");
}
