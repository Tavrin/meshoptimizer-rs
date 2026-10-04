use crate::clusterlod::support::{workspace::checked_bytes, Error, Workspace};
use alloc::vec::Vec;

pub(crate) struct Budget<'a> {
    pub(crate) ws: &'a mut Workspace,
    bytes: usize,
}
impl<'a> Budget<'a> {
    pub(crate) fn new(ws: &'a mut Workspace) -> Self {
        Self { ws, bytes: 0 }
    }
    pub(crate) fn with_bytes(ws: &'a mut Workspace, bytes: usize) -> Self {
        Self { ws, bytes }
    }
    pub(crate) fn bytes(&self) -> usize {
        self.bytes
    }
    pub(crate) fn release<T>(&mut self, v: Vec<T>) {
        self.bytes -= v.capacity() * core::mem::size_of::<T>();
        drop(v);
    }
    pub(crate) fn filled<T: Clone>(&mut self, n: usize, value: T) -> Result<Vec<T>, Error> {
        let bytes = checked_bytes(n, core::mem::size_of::<T>())?;
        self.ws.prepare(
            [0; 4],
            self.bytes.checked_add(bytes).ok_or(Error::SizeOverflow)?,
        )?;
        let mut v = Vec::new();
        v.try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        self.bytes = self
            .bytes
            .checked_add(checked_bytes(v.capacity(), core::mem::size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        self.ws.prepare([0; 4], self.bytes)?;
        v.resize(n, value);
        Ok(v)
    }
}
