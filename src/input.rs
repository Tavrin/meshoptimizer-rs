use crate::Error;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

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
    #[inline]
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

/// Checked borrowed XYZ view, preserving all supplied vertices, including unused ones.
///
/// Layout constructors do not scan values. Geometric operations reject
/// non-finite components before changing destinations.
#[derive(Clone, Copy, Debug)]
pub struct Positions<'a>(PositionStorage<'a>);

impl<'a> Positions<'a> {
    /// Borrow tightly packed XYZ components without allocation.
    pub const fn from_packed(data: &'a [[f32; 3]]) -> Self {
        Self(PositionStorage::Packed(data))
    }
    /// Borrow interleaved floats; `stride` and `offset` are in f32 elements.
    /// The stride must be between 3 and 64 elements.
    pub fn from_interleaved(
        data: &'a [f32],
        count: usize,
        stride: usize,
        offset: usize,
    ) -> Result<Self, Error> {
        Ok(Self(PositionStorage::View(View::floats(
            data, count, 3, stride, offset,
        )?)))
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
        Ok(Self(PositionStorage::View(View::bytes(
            data, count, 3, stride, offset, order,
        )?)))
    }
    /// Number of supplied vertices.
    pub const fn len(self) -> usize {
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
        match self.0 {
            PositionStorage::Packed(p) => p.get(index).copied(),
            PositionStorage::View(v) => {
                Some([v.get(index, 0)?, v.get(index, 1)?, v.get(index, 2)?])
            }
        }
    }
    /// Dispatch the storage layout once for sequential scans.
    pub(crate) fn for_each(
        self,
        mut visit: impl FnMut([f32; 3]) -> Result<(), Error>,
    ) -> Result<(), Error> {
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
    pub(crate) fn for_each_counted(
        self,
        work: &mut crate::workspace::Work,
        mut visit: impl FnMut([f32; 3]) -> Result<(), Error>,
    ) -> Result<(), Error> {
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
pub struct Attributes<'a>(View<'a>);

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
        Ok(Self(View::floats(data, count, components, stride, offset)?))
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
        Ok(Self(View::bytes(
            data, count, components, stride, offset, order,
        )?))
    }
    /// Number of supplied vertices.
    pub const fn len(self) -> usize {
        self.0.count
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
    #[inline]
    pub fn get(self, vertex: usize, component: usize) -> Option<f32> {
        self.0.get(vertex, component)
    }
}
