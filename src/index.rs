//! Checked adapters for vertex index widths.
use crate::{Error, Workspace};
use alloc::vec::Vec;

mod sealed {
    pub trait Sealed {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
}

/// A vertex index stored as `u16` or `u32`.
///
/// This trait is sealed. Algorithms retain their upstream u32 kernels: u32
/// buffers pass through without copying, while u16 buffers use fallible,
/// workspace-limited conversion storage. Narrowing returns [`Error::IndexOverflow`]
/// when any output index exceeds `u16::MAX`.
pub trait Index: sealed::Sealed + Copy + core::fmt::Debug + Eq + Default + Send + Sync {
    #[cfg(feature = "clusterlod")]
    #[doc(hidden)]
    fn from_groups(
        groups: Vec<crate::clusterlod::GroupOutputGeneric>,
        ws: &mut Workspace,
    ) -> Result<Vec<crate::clusterlod::GroupOutputGeneric<Self>>, Error>;
    #[cfg(feature = "clusterlod")]
    #[doc(hidden)]
    fn with_clusters<R>(
        clusters: &[crate::clusterlod::ClusterGeneric],
        live: usize,
        ws: &mut Workspace,
        f: impl FnOnce(&[crate::clusterlod::ClusterGeneric<Self>]) -> Result<R, Error>,
    ) -> Result<R, Error>;
    /// Widen one index without loss.
    fn to_u32(self) -> u32;
    /// Narrow one index, rejecting an unrepresentable value.
    fn from_u32(value: u32) -> Result<Self, Error>;
    #[doc(hidden)]
    fn with_input<R>(
        input: &[Self],
        ws: &mut Workspace,
        f: impl FnOnce(&[u32], &mut Workspace) -> Result<R, Error>,
    ) -> Result<R, Error>;
    #[doc(hidden)]
    fn with_mut<R>(
        input: &mut [Self],
        ws: &mut Workspace,
        f: impl FnOnce(&mut [u32], &mut Workspace) -> Result<R, Error>,
    ) -> Result<R, Error>;
    #[doc(hidden)]
    fn from_vec(input: Vec<u32>, ws: &mut Workspace) -> Result<Vec<Self>, Error>;
    #[doc(hidden)]
    fn with_optional<R>(
        input: Option<&[Self]>,
        ws: &mut Workspace,
        f: impl FnOnce(Option<&[u32]>, &mut Workspace) -> Result<R, Error>,
    ) -> Result<R, Error> {
        match input {
            Some(input) => Self::with_input(input, ws, |input, ws| f(Some(input), ws)),
            None => f(None, ws),
        }
    }
}
impl Index for u32 {
    #[cfg(feature = "clusterlod")]
    #[inline(always)]
    fn with_clusters<R>(
        clusters: &[crate::clusterlod::ClusterGeneric],
        _: usize,
        _: &mut Workspace,
        f: impl FnOnce(&[crate::clusterlod::ClusterGeneric<Self>]) -> Result<R, Error>,
    ) -> Result<R, Error> {
        f(clusters)
    }

    #[cfg(feature = "clusterlod")]
    #[inline(always)]
    fn from_groups(
        groups: Vec<crate::clusterlod::GroupOutputGeneric>,
        _: &mut Workspace,
    ) -> Result<Vec<crate::clusterlod::GroupOutputGeneric<Self>>, Error> {
        Ok(groups)
    }

    #[inline(always)]
    fn to_u32(self) -> u32 {
        self
    }
    #[inline(always)]
    fn from_u32(value: u32) -> Result<Self, Error> {
        Ok(value)
    }
    #[inline(always)]
    fn with_input<R>(
        input: &[Self],
        ws: &mut Workspace,
        f: impl FnOnce(&[u32], &mut Workspace) -> Result<R, Error>,
    ) -> Result<R, Error> {
        f(input, ws)
    }
    #[inline(always)]
    fn with_mut<R>(
        input: &mut [Self],
        ws: &mut Workspace,
        f: impl FnOnce(&mut [u32], &mut Workspace) -> Result<R, Error>,
    ) -> Result<R, Error> {
        f(input, ws)
    }
    #[inline(always)]
    fn from_vec(input: Vec<u32>, _: &mut Workspace) -> Result<Vec<Self>, Error> {
        Ok(input)
    }
}
fn reserved<T>(n: usize, ws: &mut Workspace) -> Result<Vec<T>, Error> {
    let bytes = crate::workspace::checked_bytes(n, core::mem::size_of::<T>())?;
    if bytes > ws.limits().max_bytes {
        return Err(Error::LimitExceeded);
    }
    ws.charge_owned(bytes)?;
    let mut out = Vec::new();
    out.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    Ok(out)
}
fn widened(input: &[u16], ws: &mut Workspace) -> Result<Vec<u32>, Error> {
    let mut out = reserved(input.len(), ws)?;
    out.extend(input.iter().copied().map(u32::from));
    Ok(out)
}
struct Storage<'a> {
    ws: &'a mut Workspace,
    limits: crate::Limits,
}
impl Drop for Storage<'_> {
    fn drop(&mut self) {
        self.ws.set_lod_limits(self.limits);
    }
}
fn with_storage<R>(
    bytes: usize,
    ws: &mut Workspace,
    f: impl FnOnce(&mut Workspace) -> Result<R, Error>,
) -> Result<R, Error> {
    let limits = ws.limits();
    let mut inner = limits;
    inner.max_bytes = inner
        .max_bytes
        .checked_sub(bytes)
        .ok_or(Error::LimitExceeded)?;
    ws.set_lod_limits(inner);
    let guard = Storage { ws, limits };
    let result = f(&mut *guard.ws);
    let mut usage = guard.ws.usage();
    usage.bytes = usage.bytes.checked_add(bytes).ok_or(Error::SizeOverflow)?;
    guard.ws.set_lod_usage(usage);
    result
}
impl Index for u16 {
    #[cfg(feature = "clusterlod")]
    fn with_clusters<R>(
        clusters: &[crate::clusterlod::ClusterGeneric],
        live: usize,
        ws: &mut Workspace,
        f: impl FnOnce(&[crate::clusterlod::ClusterGeneric<Self>]) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let mut budget = crate::budget::Budget::with_bytes(ws, live);
        let mut out = budget.reserved(clusters.len())?;
        for cluster in clusters {
            let mut indices = budget.reserved(cluster.indices.len())?;
            for &value in &cluster.indices {
                indices.push(Self::from_u32(value)?);
            }
            out.push(crate::clusterlod::ClusterGeneric {
                indices,
                refined: cluster.refined,
                bounds: cluster.bounds,
                vertex_count: cluster.vertex_count,
            });
        }
        f(&out)
    }

    #[cfg(feature = "clusterlod")]
    fn from_groups(
        groups: Vec<crate::clusterlod::GroupOutputGeneric>,
        ws: &mut Workspace,
    ) -> Result<Vec<crate::clusterlod::GroupOutputGeneric<Self>>, Error> {
        crate::clusterlod::convert_groups(groups, ws)
    }

    fn to_u32(self) -> u32 {
        u32::from(self)
    }
    fn from_u32(value: u32) -> Result<Self, Error> {
        Self::try_from(value).map_err(|_| Error::IndexOverflow)
    }
    fn with_input<R>(
        input: &[Self],
        ws: &mut Workspace,
        f: impl FnOnce(&[u32], &mut Workspace) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let wide = widened(input, ws)?;
        let bytes = crate::workspace::checked_bytes(wide.capacity(), 4)?;
        with_storage(bytes, ws, |ws| f(&wide, ws))
    }
    fn with_mut<R>(
        input: &mut [Self],
        ws: &mut Workspace,
        f: impl FnOnce(&mut [u32], &mut Workspace) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let mut wide = widened(input, ws)?;
        let bytes = crate::workspace::checked_bytes(wide.capacity(), 4)?;
        let result = with_storage(bytes, ws, |ws| f(&mut wide, ws))?;
        if wide.iter().any(|&v| v > u32::from(u16::MAX)) {
            return Err(Error::IndexOverflow);
        }
        for (dst, value) in input.iter_mut().zip(wide) {
            *dst = Self::from_u32(value)?;
        }
        Ok(result)
    }
    fn from_vec(input: Vec<u32>, ws: &mut Workspace) -> Result<Vec<Self>, Error> {
        if input.iter().any(|&v| v > u32::from(u16::MAX)) {
            return Err(Error::IndexOverflow);
        }
        let input_bytes = crate::workspace::checked_bytes(input.capacity(), 4)?;
        let live = ws.usage().bytes.max(input_bytes);
        let output_bytes = crate::workspace::checked_bytes(input.len(), 2)?;
        if live.checked_add(output_bytes).ok_or(Error::SizeOverflow)? > ws.limits().max_bytes {
            return Err(Error::LimitExceeded);
        }
        let mut out = reserved(input.len(), ws)?;
        let peak = live
            .checked_add(crate::workspace::checked_bytes(out.capacity(), 2)?)
            .ok_or(Error::SizeOverflow)?;
        if peak > ws.limits().max_bytes {
            return Err(Error::LimitExceeded);
        }
        for value in input {
            out.push(Self::from_u32(value)?);
        }
        let mut usage = ws.usage();
        usage.bytes = usage.bytes.max(peak);
        ws.set_lod_usage(usage);
        Ok(out)
    }
}
pub(crate) trait Convert<I: Index> {
    type Output;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error>;
}
impl<I: Index> Convert<I> for Vec<u32> {
    type Output = Vec<I>;
    #[inline(always)]
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        I::from_vec(self, ws)
    }
}
impl<I: Index> Convert<I> for crate::OrientedMeshGeneric {
    type Output = crate::OrientedMeshGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::OrientedMeshGeneric {
            indices: I::from_vec(self.indices, ws)?,
            report: self.report,
        })
    }
}
impl<I: Index> Convert<I> for crate::FilledMeshGeneric {
    type Output = crate::FilledMeshGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::FilledMeshGeneric {
            indices: I::from_vec(self.indices, ws)?,
            report: self.report,
        })
    }
}
impl<I: Index> Convert<I> for crate::SanitizedMeshGeneric {
    type Output = crate::SanitizedMeshGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::SanitizedMeshGeneric {
            indices: I::from_vec(self.indices, ws)?,
            report: self.report,
        })
    }
}
impl<I: Index> Convert<I> for crate::SimplifiedMeshGeneric {
    type Output = crate::SimplifiedMeshGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::SimplifiedMeshGeneric {
            indices: I::from_vec(self.indices, ws)?,
            error: self.error,
        })
    }
}
impl<I: Index> Convert<I> for crate::MeshletsGeneric {
    type Output = crate::MeshletsGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::MeshletsGeneric {
            meshlets: self.meshlets,
            vertices: I::from_vec(self.vertices, ws)?,
            triangles: self.triangles,
        })
    }
}
impl<I: Index> Convert<I> for crate::VisibleMeshGeneric {
    type Output = crate::VisibleMeshGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::VisibleMeshGeneric {
            indices: I::from_vec(self.indices, ws)?,
            report: self.report,
        })
    }
}
impl<I: Index> Convert<I> for crate::FetchedMeshGeneric {
    type Output = crate::FetchedMeshGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::FetchedMeshGeneric {
            indices: I::from_vec(self.indices, ws)?,
            vertices: self.vertices,
            vertex_count: self.vertex_count,
        })
    }
}
impl<I: Index> Convert<I> for crate::ProvokingMeshGeneric {
    type Output = crate::ProvokingMeshGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::ProvokingMeshGeneric {
            indices: I::from_vec(self.indices, ws)?,
            reorder: self.reorder,
        })
    }
}
impl<I: Index> Convert<I> for crate::LocalMeshletGeneric {
    type Output = crate::LocalMeshletGeneric<I>;
    fn convert(self, ws: &mut Workspace) -> Result<Self::Output, Error> {
        Ok(crate::LocalMeshletGeneric {
            vertices: I::from_vec(self.vertices, ws)?,
            triangles: self.triangles,
        })
    }
}
