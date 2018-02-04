use core::cmp::min;

use void::{Void, ResultVoidExt};

use super::*;

pub struct Cursor<T> {
    inner: T,
    pos: u64
}

impl<T> Cursor<T> {
    pub fn new(inner: T) -> Cursor<T> {
        Cursor {
            inner: inner,
            pos: 0
        }
    }
}

impl<'a> Read for Cursor<&'a [u8]> {
    type Err = Void;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Void> {
        let len = (&self.inner[self.pos as usize..]).read(buf).void_unwrap();
        self.pos += len as u64;
        Ok(len)
    }

    fn read_exact<E>(&mut self, buf: &mut [u8]) -> Result<(), E>
        where E: From<Void> + From<EndOfFile>
    {
        try!((&self.inner[self.pos as usize..]).read_exact::<E>(buf));
        self.pos += buf.len() as u64;
        Ok(())
    }

}

impl<'a> Write for Cursor<&'a mut [u8]> {
    type Err = Void;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Void> {
        let len = (&mut self.inner[self.pos as usize..]).write(buf).void_unwrap();
        self.pos += len as u64;
        Ok(len)
    }

    fn write_all<E>(&mut self, buf: &[u8]) -> Result<(), E>
        where E: From<Void> + From<EndOfFile>
    {
        try!((&mut self.inner[self.pos as usize..]).write_all::<E>(buf));
        self.pos += buf.len() as u64;
        Ok(())
    }

}

impl<'a> Seek for Cursor<&'a [u8]> {
    type Err = OutOfBounds;

    fn seek(&mut self, from: SeekFrom) -> Result<u64, OutOfBounds> {
        let pos = match from {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::End(offset) => self.inner.len() as i64 + offset as i64,
            SeekFrom::Current(offset) => self.pos as i64 + offset
        };
        if pos < 0 {
            return Err(OutOfBounds);
        }
        self.pos = min(pos as u64, self.inner.len() as u64);
        Ok(self.pos)
    }
}
