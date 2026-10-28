//! Typed vertex layouts, an opt-in extension not in C++ meshoptimizer.
//!
//! Struct padding cannot safely be borrowed as initialized bytes. These helpers
//! copy only the selected float field into a zero-filled byte buffer, preserving
//! the struct's byte stride and field offset. Construction allocates and copies;
//! the returned existing views borrow that snapshot, not the original vertices.

use alloc::vec::Vec;
use core::mem::{align_of, size_of};

use crate::{Attributes, ByteOrder, Positions};

/// A construction failure for a typed vertex layout or snapshot.
///
/// New variants may be added in a minor release.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VertexLayoutError {
    /// The struct stride is not a multiple of four in the range 4..=256.
    InvalidStride,
    /// The struct alignment or field offset is unsuitable for `f32`.
    MisalignedField,
    /// The field has zero or more than 32 components, or extends past the record.
    InvalidFieldSize,
    /// A position field does not contain exactly three components.
    InvalidPositionSize,
    /// The selector returned a field at a different address from the given offset.
    FieldOffsetMismatch,
    /// The snapshot length overflowed the platform allocation range.
    SizeOverflow,
    /// The snapshot could not be allocated.
    AllocationFailed,
}

impl core::fmt::Display for VertexLayoutError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::InvalidStride => "unsupported vertex stride",
            Self::MisalignedField => "misaligned float field",
            Self::InvalidFieldSize => "unsupported field size",
            Self::InvalidPositionSize => "positions require three float components",
            Self::FieldOffsetMismatch => "field selector does not match offset",
            Self::SizeOverflow => "vertex snapshot size overflow",
            Self::AllocationFailed => "vertex snapshot allocation failed",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VertexLayoutError {}

/// Checked float-array field layout, not in C++ meshoptimizer.
///
/// `V` need not be plain old data: padding and other fields are never read.
/// Use `#[repr(C)]` for a stable layout. The selector enforces the `f32` element
/// type and array size at compile time. Its address is checked against `offset`
/// for every vertex when copying; an empty slice has no field address to check.
///
/// ```
/// use core::mem::{offset_of, size_of};
/// use meshoptimizer_rs::{VertexFieldSnapshot, VertexLayout};
///
/// #[repr(C)]
/// struct Vertex {
///     position: [f32; 3],
///     normal: [f32; 3],
///     uv: [f32; 2],
/// }
/// let vertices = [Vertex {
///     position: [1.0, 2.0, 3.0], normal: [0.0, 0.0, 1.0], uv: [0.5, 0.5],
/// }];
/// let layout = VertexLayout::new(offset_of!(Vertex, position), |v: &Vertex| &v.position)?;
/// assert_eq!(layout.stride(), size_of::<Vertex>());
/// let positions = layout.copy_from(&vertices)?;
/// assert_eq!(positions.positions()?.get(0), Some([1.0, 2.0, 3.0]));
/// let normals = VertexFieldSnapshot::from_vertices(
///     &vertices, offset_of!(Vertex, normal), |v| &v.normal,
/// )?;
/// assert_eq!(normals.attributes()?.components(), 3);
/// let uv = VertexFieldSnapshot::from_vertices(&vertices, offset_of!(Vertex, uv), |v| &v.uv)?;
/// assert_eq!(uv.attributes()?.components(), 2);
/// # Ok::<(), meshoptimizer_rs::VertexLayoutError>(())
/// ```
///
/// Non-float fields cannot be selected:
///
/// ```compile_fail
/// use meshoptimizer_rs::VertexLayout;
/// #[repr(C)]
/// struct Vertex { position: [u32; 3] }
/// let layout = VertexLayout::new(0, |v: &Vertex| &v.position);
/// ```
///
/// The selector may be any `Fn(&V) -> &[f32; N]`, including a capturing closure;
/// the layout is `Clone`/`Copy` when the selector is.
pub struct VertexLayout<V, const N: usize, F> {
    offset: usize,
    field: F,
    marker: core::marker::PhantomData<fn(&V) -> &[f32; N]>,
}

// Manual impls: derives would needlessly require `V: Clone`/`V: Debug`.
impl<V, const N: usize, F: Clone> Clone for VertexLayout<V, N, F> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            field: self.field.clone(),
            marker: core::marker::PhantomData,
        }
    }
}

impl<V, const N: usize, F: Copy> Copy for VertexLayout<V, N, F> {}

impl<V, const N: usize, F> core::fmt::Debug for VertexLayout<V, N, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VertexLayout")
            .field("stride", &self.stride())
            .field("offset", &self.offset)
            .field("components", &N)
            .finish()
    }
}

impl<V, const N: usize, F> VertexLayout<V, N, F> {
    /// Byte stride, equal to `size_of::<V>()`.
    pub const fn stride(&self) -> usize {
        size_of::<V>()
    }

    /// Byte offset of the selected field.
    pub const fn offset(&self) -> usize {
        self.offset
    }
}

impl<V, const N: usize, F: Fn(&V) -> &[f32; N]> VertexLayout<V, N, F> {
    /// Check a byte offset and typed field selector using `V`'s size/alignment.
    ///
    /// # Errors
    /// Returns `InvalidStride`, `MisalignedField` or `InvalidFieldSize` for an
    /// unsupported layout. A selector/offset mismatch is checked by `copy_from`.
    pub fn new(offset: usize, field: F) -> Result<Self, VertexLayoutError> {
        let stride = size_of::<V>();
        if stride == 0 || stride > 256 || !stride.is_multiple_of(4) {
            return Err(VertexLayoutError::InvalidStride);
        }
        if align_of::<V>() < align_of::<f32>() || !offset.is_multiple_of(align_of::<f32>()) {
            return Err(VertexLayoutError::MisalignedField);
        }
        if N == 0 || N > 32 || offset.checked_add(N * 4).is_none_or(|end| end > stride) {
            return Err(VertexLayoutError::InvalidFieldSize);
        }
        Ok(Self {
            offset,
            field,
            marker: core::marker::PhantomData,
        })
    }

    /// Copy the selected field into an initialized snapshot with the same layout.
    ///
    /// Unselected bytes are zero, including padding. Float bit patterns are
    /// preserved, including non-finite values; geometric operations validate them.
    /// Allocation is caller-owned and outside `Workspace` limits.
    ///
    /// # Errors
    /// Returns `SizeOverflow` or `AllocationFailed` if the snapshot cannot be
    /// allocated; `FieldOffsetMismatch` if any selector result differs from the
    /// declared field address. Selector panics propagate to the caller.
    pub fn copy_from(&self, vertices: &[V]) -> Result<VertexFieldSnapshot<N>, VertexLayoutError> {
        let len = vertices
            .len()
            .checked_mul(self.stride())
            .filter(|&len| len <= isize::MAX as usize)
            .ok_or(VertexLayoutError::SizeOverflow)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(len)
            .map_err(|_| VertexLayoutError::AllocationFailed)?;
        bytes.resize(len, 0);
        for (vertex, record) in vertices.iter().zip(bytes.chunks_exact_mut(self.stride())) {
            let field = (self.field)(vertex);
            if (field.as_ptr() as usize).checked_sub(vertex as *const V as usize)
                != Some(self.offset)
            {
                return Err(VertexLayoutError::FieldOffsetMismatch);
            }
            for (component, value) in field.iter().enumerate() {
                let start = self.offset + component * 4;
                record[start..start + 4].copy_from_slice(&value.to_le_bytes());
            }
        }
        Ok(VertexFieldSnapshot {
            bytes,
            count: vertices.len(),
            stride: self.stride(),
            offset: self.offset,
        })
    }
}

/// Owned field snapshot exposing existing views, not in C++ meshoptimizer.
///
/// Stores `vertex_count * size_of::<V>()` bytes. Only the selected field is
/// copied; remaining bytes are zero. This is not a serialization of the struct.
#[derive(Debug)]
pub struct VertexFieldSnapshot<const N: usize> {
    bytes: Vec<u8>,
    count: usize,
    stride: usize,
    offset: usize,
}

impl<const N: usize> VertexFieldSnapshot<N> {
    /// Check and copy a typed field directly from a vertex slice and byte offset.
    ///
    /// # Errors
    /// Returns the layout and snapshot errors described by `VertexLayout::new`
    /// and `VertexLayout::copy_from`.
    pub fn from_vertices<V>(
        vertices: &[V],
        offset: usize,
        field: impl Fn(&V) -> &[f32; N],
    ) -> Result<Self, VertexLayoutError> {
        VertexLayout::new(offset, field)?.copy_from(vertices)
    }

    /// Borrow XYZ from this snapshot.
    ///
    /// # Errors
    /// Returns `InvalidPositionSize` unless the selected field has three floats.
    pub fn positions(&self) -> Result<Positions<'_>, VertexLayoutError> {
        if N != 3 {
            return Err(VertexLayoutError::InvalidPositionSize);
        }
        // Empty byte views require offset <= buffer length.
        Positions::from_bytes(
            &self.bytes,
            self.count,
            self.stride,
            if self.count == 0 { 0 } else { self.offset },
            ByteOrder::LittleEndian,
        )
        .map_err(|_| VertexLayoutError::InvalidFieldSize)
    }

    /// Borrow the selected float components as attributes.
    ///
    /// # Errors
    /// Returns `InvalidFieldSize` if the underlying view rejects the layout.
    pub fn attributes(&self) -> Result<Attributes<'_>, VertexLayoutError> {
        Attributes::from_bytes(
            &self.bytes,
            self.count,
            N,
            self.stride,
            if self.count == 0 { 0 } else { self.offset },
            ByteOrder::LittleEndian,
        )
        .map_err(|_| VertexLayoutError::InvalidFieldSize)
    }

    /// Initialized little-endian bytes, including zero-filled unselected bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Byte stride of each original vertex.
    pub const fn stride(&self) -> usize {
        self.stride
    }

    /// Byte offset of the selected field in each original vertex.
    pub const fn offset(&self) -> usize {
        self.offset
    }
}
