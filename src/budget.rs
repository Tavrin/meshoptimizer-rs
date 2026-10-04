use crate::{workspace::checked_bytes, Error, Workspace};
use alloc::vec::Vec;

pub(crate) struct Budget<'a> {
    pub(crate) ws: &'a mut Workspace,
    bytes: usize,
    retained: Result<usize, Error>,
    charged: bool,
}
impl<'a> Budget<'a> {
    pub(crate) fn new(ws: &'a mut Workspace) -> Self {
        Self::with_bytes(ws, 0)
    }
    pub(crate) fn with_bytes(ws: &'a mut Workspace, bytes: usize) -> Self {
        let retained = ws.retained_bytes();
        Self {
            ws,
            bytes,
            retained,
            charged: false,
        }
    }
    pub(crate) fn bytes(&self) -> usize {
        self.bytes
    }
    pub(crate) fn release<T>(&mut self, v: Vec<T>) {
        self.bytes -= v.capacity() * core::mem::size_of::<T>();
        drop(v);
    }
    fn charge_current(&mut self) -> Result<(), Error> {
        if !self.charged {
            self.ws.charge_owned(self.retained?, self.bytes)?;
            self.charged = true;
        }
        Ok(())
    }
    pub(crate) fn reuse<T: Clone>(
        &mut self,
        v: &mut Vec<T>,
        n: usize,
        value: T,
    ) -> Result<(), Error> {
        let size = core::mem::size_of::<T>();
        let old = checked_bytes(v.capacity(), size)?;
        if self.bytes < old {
            return Err(Error::SizeOverflow);
        }
        if n <= v.capacity() {
            self.charge_current()?;
            v.resize(n, value);
            return Ok(());
        }
        let minimum = n;
        let base = self
            .bytes
            .checked_sub(checked_bytes(v.capacity(), size)?)
            .ok_or(Error::SizeOverflow)?;
        self.ws.charge_owned(
            self.retained?,
            base.checked_add(checked_bytes(minimum, size)?)
                .ok_or(Error::SizeOverflow)?,
        )?;
        if n > v.capacity() {
            v.try_reserve_exact(n.saturating_sub(v.len()))
                .map_err(|_| Error::AllocationFailed)?;
        }
        self.bytes = base
            .checked_add(checked_bytes(v.capacity(), size)?)
            .ok_or(Error::SizeOverflow)?;
        if v.capacity() != minimum {
            self.ws.charge_owned(self.retained?, self.bytes)?;
        }
        v.resize(n, value);
        self.charged = true;
        Ok(())
    }
    pub(crate) fn filled<T: Clone>(&mut self, n: usize, value: T) -> Result<Vec<T>, Error> {
        let bytes = checked_bytes(n, core::mem::size_of::<T>())?;
        self.ws.charge_owned(
            self.retained?,
            self.bytes.checked_add(bytes).ok_or(Error::SizeOverflow)?,
        )?;
        let mut v = Vec::new();
        v.try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        self.bytes = self
            .bytes
            .checked_add(checked_bytes(v.capacity(), core::mem::size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        if v.capacity() != n {
            self.ws.charge_owned(self.retained?, self.bytes)?;
        }
        v.resize(n, value);
        self.charged = true;
        Ok(v)
    }
    pub(crate) fn reserved<T>(&mut self, n: usize) -> Result<Vec<T>, Error> {
        let bytes = checked_bytes(n, core::mem::size_of::<T>())?;
        self.ws.charge_owned(
            self.retained?,
            self.bytes.checked_add(bytes).ok_or(Error::SizeOverflow)?,
        )?;
        let mut v = Vec::new();
        v.try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        self.bytes = self
            .bytes
            .checked_add(checked_bytes(v.capacity(), core::mem::size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        if v.capacity() != n {
            self.ws.charge_owned(self.retained?, self.bytes)?;
        }
        self.charged = true;
        Ok(v)
    }
    pub(crate) fn copied<T: Copy>(&mut self, source: &[T]) -> Result<Vec<T>, Error> {
        let bytes = checked_bytes(source.len(), core::mem::size_of::<T>())?;
        self.ws.charge_owned(
            self.retained?,
            self.bytes.checked_add(bytes).ok_or(Error::SizeOverflow)?,
        )?;
        let mut v = Vec::new();
        v.try_reserve_exact(source.len())
            .map_err(|_| Error::AllocationFailed)?;
        self.bytes = self
            .bytes
            .checked_add(checked_bytes(v.capacity(), core::mem::size_of::<T>())?)
            .ok_or(Error::SizeOverflow)?;
        if v.capacity() != source.len() {
            self.ws.charge_owned(self.retained?, self.bytes)?;
        }
        v.extend_from_slice(source);
        self.charged = true;
        Ok(v)
    }
}
