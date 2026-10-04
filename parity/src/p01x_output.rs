use meshoptimizer_rs::*;
pub(super) enum Payload {
    Words(Vec<u32>),
    Scalar(u32),
    Remap(VertexRemap),
    Custom(VertexRemap, Vec<u32>),
    Bytes(Vec<u8>),
    Fetch(FetchedMesh),
    Provoking(ProvokingMesh),
    Simplified(SimplifiedMesh),
    Updated(SimplifyResult, Vec<u32>, Vec<[f32; 3]>, Vec<f32>),
    Caller { count: usize, header: Option<u32> },
    CallerBytes(usize),
    CallerFetch(usize, Vec<u32>, usize),
    CallerCustom(usize, Vec<u32>, usize),
    CallerProvoking(usize, usize),
}
impl Payload {
    pub(super) fn words(self, destination: &[u32], bytes: &[u8], reorder: &[u32]) -> Vec<u32> {
        let mut values = Vec::new();
        match self {
            Self::Words(v) => return v,
            Self::Scalar(v) => values.push(v),
            Self::Remap(r) => {
                values.push(r.vertex_count as u32);
                values.extend(r.remap);
            }
            Self::Custom(r, calls) => {
                values.push(r.vertex_count as u32);
                values.extend(r.remap);
                values.push(calls.len() as u32);
                values.extend(calls);
            }
            Self::Bytes(b) => values.extend(b.into_iter().map(u32::from)),
            Self::Fetch(r) => {
                values.push(r.vertex_count as u32);
                values.extend(r.indices);
                values.extend(r.vertices.into_iter().map(u32::from));
            }
            Self::Provoking(r) => {
                values.push(r.reorder.len() as u32);
                values.extend(r.indices);
                values.extend(r.reorder);
            }
            Self::Simplified(r) => {
                values.push(r.error.to_bits());
                values.extend(r.indices);
            }
            Self::Updated(r, i, p, a) => {
                values.extend([r.error.to_bits(), r.index_count as u32]);
                values.extend_from_slice(&i[..r.index_count]);
                values.extend(p.into_iter().flatten().map(f32::to_bits));
                values.extend(a.into_iter().map(f32::to_bits));
            }
            Self::Caller { count, header } => {
                values.extend(header);
                values.extend_from_slice(&destination[..count]);
            }
            Self::CallerBytes(n) => values.extend(bytes[..n].iter().copied().map(u32::from)),
            Self::CallerFetch(count, i, n) => {
                values.push(count as u32);
                values.extend(i);
                values.extend(bytes[..n].iter().copied().map(u32::from));
            }
            Self::CallerCustom(count, calls, n) => {
                values.push(count as u32);
                values.extend_from_slice(&destination[..n]);
                values.push(calls.len() as u32);
                values.extend(calls);
            }
            Self::CallerProvoking(n, ic) => {
                values.push(n as u32);
                values.extend_from_slice(&destination[..ic]);
                values.extend_from_slice(&reorder[..n]);
            }
        }
        values
    }
}
