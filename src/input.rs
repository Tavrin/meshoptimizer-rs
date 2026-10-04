use crate::Error;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Debug)]
enum MutableStorage<'a> {
    Packed(&'a mut [[f32; 3]]),
    Floats(&'a mut [f32]),
    Bytes(&'a mut [u8], ByteOrder),
}
/// Checked mutable positions for destructive update simplification.
#[derive(Debug)]
pub struct PositionsMut<'a> {
    storage: MutableStorage<'a>,
    count: usize,
    stride: usize,
    offset: usize,
}
impl<'a> PositionsMut<'a> {
    /// Borrow tightly packed positions.
    pub fn from_packed(data: &'a mut [[f32; 3]]) -> Self {
        let count = data.len();
        Self {
            storage: MutableStorage::Packed(data),
            count,
            stride: 3,
            offset: 0,
        }
    }
    /// Borrow interleaved floats with a stride and offset in float components.
    pub fn from_interleaved(
        data: &'a mut [f32],
        count: usize,
        stride: usize,
        offset: usize,
    ) -> Result<Self, Error> {
        layout(data.len(), count, 3, stride, offset)?;
        Ok(Self {
            storage: MutableStorage::Floats(data),
            count,
            stride,
            offset,
        })
    }
    /// Borrow initialized byte positions with explicit byte order and byte addresses.
    pub fn from_bytes(
        data: &'a mut [u8],
        count: usize,
        stride: usize,
        offset: usize,
        order: ByteOrder,
    ) -> Result<Self, Error> {
        View::bytes(data, count, 3, stride, offset, order)?;
        Ok(Self {
            storage: MutableStorage::Bytes(data, order),
            count,
            stride,
            offset,
        })
    }
    /// Number of positions.
    pub const fn len(&self) -> usize {
        self.count
    }
    /// Whether the view is empty.
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Borrow the same positions read-only.
    #[inline]
    pub fn as_view(&self) -> Positions<'_> {
        match &self.storage {
            MutableStorage::Packed(p) => Positions::from_packed(p),
            MutableStorage::Floats(p) => {
                Positions::from_interleaved(p, self.count, self.stride, self.offset)
                    .expect("validated layout")
            }
            MutableStorage::Bytes(p, o) => {
                Positions::from_bytes(p, self.count, self.stride, self.offset, *o)
                    .expect("validated layout")
            }
        }
    }
    pub(crate) fn set(&mut self, i: usize, value: [f32; 3]) {
        match &mut self.storage {
            MutableStorage::Packed(p) => p[i] = value,
            MutableStorage::Floats(p) => {
                let start = self.offset + i * self.stride;
                p[start..start + 3].copy_from_slice(&value);
            }
            MutableStorage::Bytes(p, o) => {
                let start = self.offset + i * self.stride;
                for (k, v) in value.into_iter().enumerate() {
                    let b = match o {
                        ByteOrder::LittleEndian => v.to_bits().to_le_bytes(),
                        ByteOrder::BigEndian => v.to_bits().to_be_bytes(),
                    };
                    p[start + k * 4..start + k * 4 + 4].copy_from_slice(&b);
                }
            }
        }
    }
}
/// Checked mutable attribute components for destructive update simplification.
#[derive(Debug)]
pub struct AttributesMut<'a> {
    storage: MutableStorage<'a>,
    count: usize,
    width: usize,
    stride: usize,
    offset: usize,
}
impl<'a> AttributesMut<'a> {
    /// Borrow up to 32 components per interleaved float record.
    pub fn from_interleaved(
        data: &'a mut [f32],
        count: usize,
        width: usize,
        stride: usize,
        offset: usize,
    ) -> Result<Self, Error> {
        Attributes::from_interleaved(data, count, width, stride, offset)?;
        Ok(Self {
            storage: MutableStorage::Floats(data),
            count,
            width,
            stride,
            offset,
        })
    }
    /// Borrow initialized byte attributes with explicit byte order.
    pub fn from_bytes(
        data: &'a mut [u8],
        count: usize,
        width: usize,
        stride: usize,
        offset: usize,
        order: ByteOrder,
    ) -> Result<Self, Error> {
        Attributes::from_bytes(data, count, width, stride, offset, order)?;
        Ok(Self {
            storage: MutableStorage::Bytes(data, order),
            count,
            width,
            stride,
            offset,
        })
    }
    /// Number of attribute records.
    pub const fn len(&self) -> usize {
        self.count
    }
    /// Whether the view is empty.
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Number of components per record.
    pub const fn components(&self) -> usize {
        self.width
    }
    /// Borrow the same attributes read-only.
    #[inline]
    pub fn as_view(&self) -> Attributes<'_> {
        match &self.storage {
            MutableStorage::Floats(p) => {
                Attributes::from_interleaved(p, self.count, self.width, self.stride, self.offset)
                    .expect("validated layout")
            }
            MutableStorage::Bytes(p, o) => {
                Attributes::from_bytes(p, self.count, self.width, self.stride, self.offset, *o)
                    .expect("validated layout")
            }
            MutableStorage::Packed(_) => unreachable!("attributes are interleaved"),
        }
    }
    pub(crate) fn set(&mut self, i: usize, k: usize, value: f32) {
        match &mut self.storage {
            MutableStorage::Floats(p) => p[self.offset + i * self.stride + k] = value,
            MutableStorage::Bytes(p, o) => {
                let start = self.offset + i * self.stride + k * 4;
                let b = match o {
                    ByteOrder::LittleEndian => value.to_bits().to_le_bytes(),
                    ByteOrder::BigEndian => value.to_bits().to_be_bytes(),
                };
                p[start..start + 4].copy_from_slice(&b);
            }
            MutableStorage::Packed(_) => unreachable!("attributes are interleaved"),
        }
    }
}

/// Explicit byte order of floating-point components in byte-backed views.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    /// Least significant byte first.
    LittleEndian,
    /// Most significant byte first.
    BigEndian,
}

/// Independent upstream simplification vertex flags; unknown bits are rejected.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VertexFlags(u8);

impl VertexFlags {
    /// No vertex preferences or restrictions.
    pub const EMPTY: Self = Self(0);
    /// Prevent removal of this vertex in simplification.
    pub const LOCK: Self = Self(1);
    /// Protect an attribute discontinuity in permissive simplification.
    pub const PROTECT: Self = Self(2);
    /// Prefer this vertex; this is not a preservation guarantee.
    pub const PRIORITY: Self = Self(4);

    /// Construct a flag combination, rejecting all unknown bits.
    pub const fn from_bits(bits: u8) -> Result<Self, Error> {
        if bits & !7 != 0 {
            Err(Error::UnknownFlags)
        } else {
            Ok(Self(bits))
        }
    }
    /// Return the upstream bit representation.
    pub const fn bits(self) -> u8 {
        self.0
    }
    /// Test whether every bit in `other` is set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for VertexFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl BitAnd for VertexFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl BitOrAssign for VertexFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl BitAndAssign for VertexFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Validate that optional vertex flags have exactly one entry per vertex.
///
/// Function-specific flag restrictions belong to the consuming operation;
/// the sloppy simplifier must not treat `PROTECT` or `PRIORITY` as locks.
pub fn validate_vertex_flags(flags: Option<&[VertexFlags]>, count: usize) -> Result<(), Error> {
    if flags.is_some_and(|f| f.len() != count) {
        Err(Error::InvalidLayout)
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum Storage<'a> {
    Floats(&'a [f32]),
    Bytes(&'a [u8], ByteOrder),
}

#[derive(Clone, Copy, Debug)]
struct View<'a> {
    storage: Storage<'a>,
    count: usize,
    width: usize,
    stride: usize,
    offset: usize,
}

fn layout(
    len: usize,
    count: usize,
    width: usize,
    stride: usize,
    offset: usize,
) -> Result<(), Error> {
    if stride < width || stride > 64 {
        return Err(Error::InvalidLayout);
    }
    let end = if count == 0 {
        offset
    } else {
        (count - 1)
            .checked_mul(stride)
            .and_then(|n| n.checked_add(offset))
            .and_then(|n| n.checked_add(width))
            .ok_or(Error::SizeOverflow)?
    };
    if end > len {
        return Err(Error::InvalidLayout);
    }
    Ok(())
}

impl<'a> View<'a> {
    fn floats(
        data: &'a [f32],
        count: usize,
        width: usize,
        stride: usize,
        offset: usize,
    ) -> Result<Self, Error> {
        layout(data.len(), count, width, stride, offset)?;
        Ok(Self {
            storage: Storage::Floats(data),
            count,
            width,
            stride,
            offset,
        })
    }
    fn bytes(
        data: &'a [u8],
        count: usize,
        width: usize,
        stride: usize,
        offset: usize,
        order: ByteOrder,
    ) -> Result<Self, Error> {
        if !stride.is_multiple_of(4) {
            return Err(Error::InvalidLayout);
        }
        let width_bytes = width.checked_mul(4).ok_or(Error::SizeOverflow)?;
        // Check byte addresses directly: byte offsets need not be aligned.
        if stride < width_bytes || stride > 256 {
            return Err(Error::InvalidLayout);
        }
        let end = if count == 0 {
            offset
        } else {
            (count - 1)
                .checked_mul(stride)
                .and_then(|n| n.checked_add(offset))
                .and_then(|n| n.checked_add(width_bytes))
                .ok_or(Error::SizeOverflow)?
        };
        if end > data.len() {
            return Err(Error::InvalidLayout);
        }
        Ok(Self {
            storage: Storage::Bytes(data, order),
            count,
            width,
            stride,
            offset,
        })
    }
    #[inline(always)]
    fn get(self, vertex: usize, component: usize) -> Option<f32> {
        if vertex >= self.count || component >= self.width {
            return None;
        }
        match self.storage {
            Storage::Floats(data) => data
                .get(self.offset + vertex * self.stride + component)
                .copied(),
            Storage::Bytes(data, order) => {
                let p = self.offset + vertex * self.stride + component * 4;
                let b = [
                    *data.get(p)?,
                    *data.get(p + 1)?,
                    *data.get(p + 2)?,
                    *data.get(p + 3)?,
                ];
                Some(f32::from_bits(match order {
                    ByteOrder::LittleEndian => u32::from_le_bytes(b),
                    ByteOrder::BigEndian => u32::from_be_bytes(b),
                }))
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum PositionStorage<'a> {
    Packed(&'a [[f32; 3]]),
    View(View<'a>),
}

type PackedPositionSource<'a> = (&'a [[f32; 3]], Option<&'a [u32]>);

/// Checked borrowed XYZ view, preserving all supplied vertices, including unused ones.
///
/// Layout constructors do not scan values. Geometric operations reject
/// non-finite components before changing destinations.
#[derive(Clone, Copy, Debug)]
pub struct Positions<'a>(PositionStorage<'a>, Option<&'a [u32]>);

impl<'a> Positions<'a> {
    /// Borrow tightly packed XYZ components without allocation.
    pub const fn from_packed(data: &'a [[f32; 3]]) -> Self {
        Self(PositionStorage::Packed(data), None)
    }
    /// Borrow interleaved floats; `stride` and `offset` are in f32 elements.
    /// The stride must be between 3 and 64 elements.
    pub fn from_interleaved(
        data: &'a [f32],
        count: usize,
        stride: usize,
        offset: usize,
    ) -> Result<Self, Error> {
        let view = View::floats(data, count, 3, stride, offset)?;
        let storage = if stride == 3 {
            let components = count.checked_mul(3).ok_or(Error::SizeOverflow)?;
            PositionStorage::Packed(data[offset..][..components].as_chunks::<3>().0)
        } else {
            PositionStorage::View(view)
        };
        Ok(Self(storage, None))
    }
    /// Borrow initialized bytes; stride and offset are in bytes, with an explicit byte order.
    /// Stride must be a multiple of four between 12 and 256 bytes.
    pub fn from_bytes(
        data: &'a [u8],
        count: usize,
        stride: usize,
        offset: usize,
        order: ByteOrder,
    ) -> Result<Self, Error> {
        Ok(Self(
            PositionStorage::View(View::bytes(data, count, 3, stride, offset, order)?),
            None,
        ))
    }
    /// Number of supplied vertices.
    pub const fn len(self) -> usize {
        if let Some(remap) = self.1 {
            return remap.len();
        }
        match self.0 {
            PositionStorage::Packed(p) => p.len(),
            PositionStorage::View(v) => v.count,
        }
    }
    /// Whether there are no supplied vertices.
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Read XYZ for a vertex, or `None` if its index is outside this view.
    #[inline(always)]
    pub fn get(self, index: usize) -> Option<[f32; 3]> {
        let index = if let Some(remap) = self.1 {
            *remap.get(index)? as usize
        } else {
            index
        };
        match self.0 {
            PositionStorage::Packed(p) => p.get(index).copied(),
            PositionStorage::View(v) => {
                Some([v.get(index, 0)?, v.get(index, 1)?, v.get(index, 2)?])
            }
        }
    }
    pub(crate) fn packed_values(self) -> Option<&'a [[f32; 3]]> {
        if self.1.is_none() {
            if let PositionStorage::Packed(values) = self.0 {
                return Some(values);
            }
        }
        None
    }
    /// Original packed coordinates and an optional sparse source mapping.
    pub(crate) fn packed_source(self) -> Option<PackedPositionSource<'a>> {
        match self.0 {
            PositionStorage::Packed(values) => Some((values, self.1)),
            PositionStorage::View(_) => None,
        }
    }
    pub(crate) fn mapped(self, remap: &'a [u32]) -> Self {
        debug_assert!(self.1.is_none());
        Self(self.0, Some(remap))
    }
    /// Dispatch the storage layout once for sequential scans.
    #[inline(always)]
    pub(crate) fn for_each(
        self,
        mut visit: impl FnMut([f32; 3]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if self.1.is_some() {
            for i in 0..self.len() {
                visit(self.at(i)?)?;
            }
            return Ok(());
        }
        match self.0 {
            PositionStorage::Packed(values) => {
                for &value in values {
                    visit(value)?;
                }
            }
            PositionStorage::View(_) => {
                for i in 0..self.len() {
                    visit(self.at(i)?)?;
                }
            }
        }
        Ok(())
    }
    #[inline(always)]
    pub(crate) fn for_each_counted(
        self,
        work: &mut crate::workspace::Work,
        mut visit: impl FnMut([f32; 3]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if self.1.is_some() {
            return work.scan(0..self.len(), |i| visit(self.at(i)?));
        }
        match self.0 {
            PositionStorage::Packed(values) => work.scan(values.iter().copied(), visit),
            PositionStorage::View(_) => work.scan(0..self.len(), |i| visit(self.at(i)?)),
        }
    }
    #[inline(always)]
    pub(crate) fn at(self, index: usize) -> Result<[f32; 3], Error> {
        self.get(index).ok_or(Error::IndexOutOfBounds)
    }
}

/// Checked borrowed attribute view with at most 32 components per vertex.
#[derive(Clone, Copy, Debug)]
pub struct Attributes<'a>(View<'a>, Option<&'a [u32]>);

impl<'a> Attributes<'a> {
    /// Borrow interleaved attributes; stride and offset are in f32 elements.
    /// Zero components are supported; stride is at most 64 elements.
    pub fn from_interleaved(
        data: &'a [f32],
        count: usize,
        components: usize,
        stride: usize,
        offset: usize,
    ) -> Result<Self, Error> {
        if components > 32 {
            return Err(Error::InvalidLayout);
        }
        Ok(Self(
            View::floats(data, count, components, stride, offset)?,
            None,
        ))
    }
    /// Borrow initialized attribute bytes with an explicit byte order.
    /// Byte stride is a multiple of four and at most 256 bytes.
    pub fn from_bytes(
        data: &'a [u8],
        count: usize,
        components: usize,
        stride: usize,
        offset: usize,
        order: ByteOrder,
    ) -> Result<Self, Error> {
        if components > 32 {
            return Err(Error::InvalidLayout);
        }
        Ok(Self(
            View::bytes(data, count, components, stride, offset, order)?,
            None,
        ))
    }
    /// Number of supplied vertices.
    pub const fn len(self) -> usize {
        if let Some(remap) = self.1 {
            remap.len()
        } else {
            self.0.count
        }
    }
    /// Whether this view has no vertices.
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Components per vertex.
    pub const fn components(self) -> usize {
        self.0.width
    }
    /// Read a component, or `None` for an out-of-range vertex or component.
    #[inline(always)]
    pub fn get(self, vertex: usize, component: usize) -> Option<f32> {
        let vertex = if let Some(remap) = self.1 {
            *remap.get(vertex)? as usize
        } else {
            vertex
        };
        self.0.get(vertex, component)
    }
    #[inline(always)]
    pub(crate) fn float_record(self, vertex: usize) -> Option<&'a [f32]> {
        let vertex = if let Some(remap) = self.1 {
            *remap.get(vertex)? as usize
        } else {
            vertex
        };
        if vertex >= self.0.count {
            return None;
        }
        let Storage::Floats(values) = self.0.storage else {
            return None;
        };
        let start = self.0.offset + vertex * self.0.stride;
        values.get(start..start + self.0.width)
    }
    pub(crate) fn mapped(self, remap: &'a [u32]) -> Self {
        debug_assert!(self.1.is_none());
        Self(self.0, Some(remap))
    }
}
