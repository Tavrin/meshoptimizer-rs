//! Shared input decoding for the 0.2.17 fuzz targets. Layout: flags, vertex
//! count - 1, triangle count, 3 bytes per vertex, 3 index bytes per triangle,
//! then target parameters read with `Cur::u8`.
#![allow(dead_code)]

pub struct Cur<'a>(&'a [u8], usize);
impl<'a> Cur<'a> {
    pub fn new(d: &'a [u8]) -> Self {
        Self(d, 0)
    }
    pub fn u8(&mut self) -> u8 {
        let v = self.0.get(self.1).copied().unwrap_or(0);
        self.1 += 1;
        v
    }
    pub fn rest(&self) -> &'a [u8] {
        self.0.get(self.1..).unwrap_or(&[])
    }
    pub fn unit(&mut self) -> f32 {
        f32::from(self.u8()) / 255.
    }
}

pub struct Mesh {
    pub pos: Vec<[f32; 3]>,
    pub idx: Vec<u32>,
    pub flags: u8,
}

/// Bit 7 of the flags admits non-finite and huge coordinates (bytes 253-255);
/// bit 6 admits indices up to two past the vertex count.
pub fn mesh(c: &mut Cur<'_>, max_vertices: usize) -> Mesh {
    let flags = c.u8();
    let nv = 1 + usize::from(c.u8()) % max_vertices;
    let nt = usize::from(c.u8());
    let coord = |c: &mut Cur<'_>| {
        let b = c.u8();
        if flags & 0x80 != 0 {
            match b {
                255 => return f32::NAN,
                254 => return f32::INFINITY,
                253 => return 1e30,
                _ => {}
            }
        }
        (f32::from(b) - 128.) / 16.
    };
    let pos = (0..nv)
        .map(|_| [coord(c), coord(c), coord(c)])
        .collect::<Vec<_>>();
    let span = nv + if flags & 0x40 != 0 { 2 } else { 0 };
    let idx = (0..nt * 3)
        .map(|_| (usize::from(c.u8()) % span) as u32)
        .collect();
    Mesh { pos, idx, flags }
}

pub fn bytes_of(pos: &[[f32; 3]]) -> Vec<u8> {
    pos.iter()
        .flat_map(|p| p.iter().flat_map(|x| x.to_le_bytes()))
        .collect()
}

pub fn narrow(idx: &[u32]) -> Vec<u16> {
    idx.iter().map(|&i| u16::try_from(i).unwrap()).collect()
}
