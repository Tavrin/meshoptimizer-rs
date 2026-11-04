//! Source meshes: PLY and glTF readers, normalisation, and the cached form
//! shared by both builders so that they see identical bytes.
use std::{fs, path::Path};

/// A triangle mesh with the attribute set Bevy accepts: position, normal, one UV set.
#[derive(Clone, Default)]
pub struct Source {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
    pub materials: Vec<u16>,
}

type R<T> = Result<T, String>;

impl Source {
    pub fn triangles(&self) -> usize {
        self.indices.len() / 3
    }

    /// Centre the bounding box on the origin and scale its longest side to `extent`.
    pub fn fit(&mut self, extent: f32) {
        let (lo, hi) = self.bounds();
        let size = (0..3).map(|a| hi[a] - lo[a]).fold(0f32, f32::max);
        let scale = if size > 0.0 { extent / size } else { 1.0 };
        for p in &mut self.positions {
            for a in 0..3 {
                p[a] = (p[a] - (lo[a] + hi[a]) * 0.5) * scale;
            }
        }
    }

    pub fn bounds(&self) -> ([f32; 3], [f32; 3]) {
        let mut lo = [f32::MAX; 3];
        let mut hi = [f32::MIN; 3];
        for p in &self.positions {
            for a in 0..3 {
                lo[a] = lo[a].min(p[a]);
                hi[a] = hi[a].max(p[a]);
            }
        }
        (lo, hi)
    }

    /// Radius of the sphere around the origin that holds every vertex.
    pub fn radius(&self) -> f32 {
        self.positions.iter().map(|p| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()).fold(0.0, f32::max)
    }

    pub fn area(&self) -> f64 {
        self.indices
            .as_chunks::<3>()
            .0
            .iter()
            .map(|t| {
                let (a, b, c) =
                    (self.positions[t[0] as usize], self.positions[t[1] as usize], self.positions[t[2] as usize]);
                0.5 * length(cross(sub(b, a), sub(c, a))) as f64
            })
            .sum()
    }

    /// Area-weighted vertex normals.
    pub fn derive_normals(&mut self) {
        let mut n = vec![[0f32; 3]; self.positions.len()];
        for t in self.indices.as_chunks::<3>().0 {
            let (a, b, c) =
                (self.positions[t[0] as usize], self.positions[t[1] as usize], self.positions[t[2] as usize]);
            let f = cross(sub(b, a), sub(c, a));
            for &i in t {
                for k in 0..3 {
                    n[i as usize][k] += f[k];
                }
            }
        }
        for v in &mut n {
            let l = length(*v);
            *v = if l > 0.0 { [v[0] / l, v[1] / l, v[2] / l] } else { [0.0, 1.0, 0.0] };
        }
        self.normals = n;
    }

    /// Box-projected UVs from the dominant normal axis, in metres of the fitted mesh.
    pub fn derive_uvs(&mut self) {
        self.uvs = self
            .positions
            .iter()
            .zip(&self.normals)
            .map(|(p, n)| {
                let (ax, ay, az) = (n[0].abs(), n[1].abs(), n[2].abs());
                if ax >= ay && ax >= az {
                    [p[1], p[2]]
                } else if ay >= az {
                    [p[0], p[2]]
                } else {
                    [p[0], p[1]]
                }
            })
            .collect();
    }

    /// Write the cached form: `VGM1`, vertex count, index count, then little-endian arrays.
    pub fn save(&self, path: &Path) -> R<()> {
        let mut b = Vec::with_capacity(12 + self.positions.len() * 32 + self.indices.len() * 4);
        b.extend_from_slice(b"VGM1");
        b.extend_from_slice(&(self.positions.len() as u32).to_le_bytes());
        b.extend_from_slice(&(self.indices.len() as u32).to_le_bytes());
        for v in self.positions.iter().flatten().chain(self.normals.iter().flatten()) {
            b.extend_from_slice(&v.to_le_bytes());
        }
        for v in self.uvs.iter().flatten() {
            b.extend_from_slice(&v.to_le_bytes());
        }
        for i in &self.indices {
            b.extend_from_slice(&i.to_le_bytes());
        }
        fs::write(path, b).map_err(|e| e.to_string())
    }

    pub fn load(path: &Path) -> R<Self> {
        let b = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if b.len() < 12 || &b[..4] != b"VGM1" {
            return Err("not a VGM1 file".into());
        }
        let word = |at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap_or_default());
        let (nv, ni) = (word(4) as usize, word(8) as usize);
        if b.len() != 12 + nv * 32 + ni * 4 {
            return Err("VGM1 length mismatch".into());
        }
        let f = |k: usize| f32::from_bits(word(12 + k * 4));
        let positions = (0..nv).map(|i| [f(i * 3), f(i * 3 + 1), f(i * 3 + 2)]).collect();
        let nb = nv * 3;
        let normals = (0..nv).map(|i| [f(nb + i * 3), f(nb + i * 3 + 1), f(nb + i * 3 + 2)]).collect();
        let ub = nv * 6;
        let uvs = (0..nv).map(|i| [f(ub + i * 2), f(ub + i * 2 + 1)]).collect();
        let indices = (0..ni).map(|i| word(12 + nv * 32 + i * 4)).collect();
        Ok(Self { positions, normals, uvs, indices, materials: Vec::new() })
    }
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn length(a: [f32; 3]) -> f32 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

/// Read a mesh by extension (`.ply`, `.gltf`). Normals and UVs are derived when absent.
pub fn read(path: &Path) -> R<Source> {
    let mut s = match path.extension().and_then(|e| e.to_str()) {
        Some("ply") => read_ply(path)?,
        Some("gltf") => read_gltf(path)?,
        _ => return Err("expected .ply or .gltf".into()),
    };
    if s.normals.len() != s.positions.len() {
        s.derive_normals();
    }
    if s.uvs.len() != s.positions.len() {
        s.derive_uvs();
    }
    Ok(s)
}

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Ascii,
    Little,
    Big,
}

fn type_size(t: &str) -> R<usize> {
    Ok(match t {
        "char" | "uchar" | "int8" | "uint8" => 1,
        "short" | "ushort" | "int16" | "uint16" => 2,
        "int" | "uint" | "float" | "int32" | "uint32" | "float32" => 4,
        "double" | "float64" => 8,
        _ => return Err(format!("unsupported ply type {t}")),
    })
}

struct Prop {
    name: String,
    ty: String,
    list: Option<String>,
}

fn read_ply(path: &Path) -> R<Source> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let end = bytes.windows(11).position(|w| w == b"end_header\n").ok_or("no ply header end")? + 11;
    let header = std::str::from_utf8(&bytes[..end]).map_err(|e| e.to_string())?;
    let mut format = Format::Ascii;
    let mut elements: Vec<(String, usize, Vec<Prop>)> = Vec::new();
    for line in header.lines() {
        let w: Vec<&str> = line.split_whitespace().collect();
        match w.as_slice() {
            ["format", f, ..] => {
                format = match *f {
                    "ascii" => Format::Ascii,
                    "binary_little_endian" => Format::Little,
                    "binary_big_endian" => Format::Big,
                    _ => return Err("unknown ply format".into()),
                }
            }
            ["element", n, c] => elements.push((n.to_string(), c.parse().map_err(|_| "bad count")?, Vec::new())),
            ["property", "list", ct, it, n] => elements.last_mut().ok_or("property before element")?.2.push(Prop {
                name: n.to_string(),
                ty: it.to_string(),
                list: Some(ct.to_string()),
            }),
            ["property", t, n] => elements.last_mut().ok_or("property before element")?.2.push(Prop {
                name: n.to_string(),
                ty: t.to_string(),
                list: None,
            }),
            _ => {}
        }
    }
    let body = &bytes[end..];
    let mut out = Source::default();
    let text: Vec<&str> = if format == Format::Ascii {
        std::str::from_utf8(body).map_err(|e| e.to_string())?.split_ascii_whitespace().collect()
    } else {
        Vec::new()
    };
    let (mut tok, mut at) = (0usize, 0usize);
    for (name, count, props) in &elements {
        for _ in 0..*count {
            let mut xyz = [0f32; 3];
            let mut face: Vec<u32> = Vec::new();
            for p in props {
                let mut read = |ty: &str| -> R<f64> {
                    if format == Format::Ascii {
                        let t = text.get(tok).ok_or("ply truncated")?;
                        tok += 1;
                        return t.parse::<f64>().map_err(|e| e.to_string());
                    }
                    let n = type_size(ty)?;
                    let raw = body.get(at..at + n).ok_or("ply truncated")?;
                    at += n;
                    let mut a = [0u8; 8];
                    a[..n].copy_from_slice(raw);
                    if format == Format::Big {
                        a[..n].reverse();
                    }
                    Ok(match ty {
                        "char" | "int8" => a[0] as i8 as f64,
                        "uchar" | "uint8" => a[0] as f64,
                        "short" | "int16" => i16::from_le_bytes([a[0], a[1]]) as f64,
                        "ushort" | "uint16" => u16::from_le_bytes([a[0], a[1]]) as f64,
                        "int" | "int32" => i32::from_le_bytes([a[0], a[1], a[2], a[3]]) as f64,
                        "uint" | "uint32" => u32::from_le_bytes([a[0], a[1], a[2], a[3]]) as f64,
                        "float" | "float32" => f32::from_le_bytes([a[0], a[1], a[2], a[3]]) as f64,
                        _ => f64::from_le_bytes(a),
                    })
                };
                if let Some(ct) = &p.list {
                    let n = read(ct)? as usize;
                    for _ in 0..n {
                        face.push(read(&p.ty)? as u32);
                    }
                } else {
                    let v = read(&p.ty)?;
                    match p.name.as_str() {
                        "x" => xyz[0] = v as f32,
                        "y" => xyz[1] = v as f32,
                        "z" => xyz[2] = v as f32,
                        _ => {}
                    }
                }
            }
            match name.as_str() {
                "vertex" => out.positions.push(xyz),
                "face" => {
                    for k in 1..face.len().saturating_sub(1) {
                        out.indices.extend([face[0], face[k], face[k + 1]]);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(out)
}

fn read_gltf(path: &Path) -> R<Source> {
    let dir = path.parent().unwrap_or(Path::new("."));
    let g = gltf::Gltf::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let mut buffers = Vec::new();
    for b in g.buffers() {
        buffers.push(match b.source() {
            gltf::buffer::Source::Uri(u) => fs::read(dir.join(u)).map_err(|e| e.to_string())?,
            gltf::buffer::Source::Bin => g.blob.clone().ok_or("missing glb blob")?,
        });
    }
    let mut out = Source::default();
    let (mut have_normals, mut have_uvs) = (true, true);
    for mesh in g.meshes() {
        for prim in mesh.primitives() {
            if prim.mode() != gltf::mesh::Mode::Triangles {
                continue;
            }
            let r = prim.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let base = out.positions.len() as u32;
            let pos: Vec<[f32; 3]> = r.read_positions().ok_or("no positions")?.collect();
            let n = pos.len();
            out.positions.extend(pos);
            match r.read_normals() {
                Some(it) => out.normals.extend(it),
                None => have_normals = false,
            }
            match r.read_tex_coords(0) {
                Some(it) => out.uvs.extend(it.into_f32()),
                None => have_uvs = false,
            }
            let first = out.indices.len();
            match r.read_indices() {
                Some(it) => out.indices.extend(it.into_u32().map(|i| i + base)),
                None => out.indices.extend(base..base + n as u32),
            }
            let slot = u16::try_from(prim.material().index().unwrap_or(0)).map_err(|_| "too many material slots")?;
            out.materials.extend(std::iter::repeat_n(slot, (out.indices.len() - first) / 3));
        }
    }
    if !have_normals {
        out.normals.clear();
    }
    if !have_uvs {
        out.uvs.clear();
    }
    Ok(out)
}
