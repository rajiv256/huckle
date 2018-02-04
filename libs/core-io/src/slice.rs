use core::cmp::min;
use core::mem::{replace, uninitialized};

use void::{Void, ResultVoidExt};

use super::*;

impl<'a> Read for &'a [u8] {
    type Err = Void;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Void> {
        let len = min(buf.len(), self.len());
        let (a, b) = self.split_at(len);
        buf.clone_from_slice(&a[..len]);
        *self = b;
        Ok(len)
    }

    fn read_exact<E>(&mut self, buf: &mut [u8]) -> Result<(), E>
        where E: From<Void> + From<EndOfFile>
    {
        if buf.len() < self.len() {
            Err(E::from(EndOfFile))
        } else {
            self.read(buf).void_unwrap();
            Ok(())
        }
    }

}

impl<'a> Write for &'a mut [u8] {
    type Err = Void;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Void> {
        let len = min(buf.len(), self.len());

        let mut tmp = replace(self, unsafe { uninitialized() });
        let (a, b) = tmp.split_at_mut(len);

        a.clone_from_slice(&buf[..len]);
        *self = b;
        Ok(len)
    }

    fn write_all<E>(&mut self, buf: &[u8]) -> Result<(), E>
        where E: From<Void> + From<EndOfFile>
    {
        if self.len() < buf.len()  {
            Err(E::from(EndOfFile))
        } else {
            self.write(buf).void_unwrap();
            Ok(())
        }
    }
}
