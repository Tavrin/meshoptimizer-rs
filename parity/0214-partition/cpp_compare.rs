use candidate::{partition_clusters, Positions, Workspace};
use std::io::{self, Read, Write};
fn main() {
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data).unwrap();
    let mut words = data
        .chunks_exact(4)
        .map(|v| u32::from_le_bytes(v.try_into().unwrap()));
    let nv = words.next().unwrap() as usize;
    let ni = words.next().unwrap() as usize;
    let nc = words.next().unwrap() as usize;
    let target = words.next().unwrap() as usize;
    let spatial = words.next().unwrap() != 0;
    let positions: Vec<[f32; 3]> = (0..nv)
        .map(|_| std::array::from_fn(|_| f32::from_bits(words.next().unwrap())))
        .collect();
    let indices: Vec<u32> = (0..ni).map(|_| words.next().unwrap()).collect();
    let counts: Vec<u32> = (0..nc).map(|_| words.next().unwrap()).collect();
    assert!(words.next().is_none());
    let p = spatial.then(|| Positions::from_packed(&positions));
    let result =
        partition_clusters(&indices, &counts, nv, p, target, &mut Workspace::default()).unwrap();
    let mut out = io::stdout().lock();
    out.write_all(&(result.count as u32).to_le_bytes()).unwrap();
    for id in result.assignments {
        out.write_all(&id.to_le_bytes()).unwrap();
    }
}
