use core::marker::PhantomData;

use super::*;

/// Adaptor to chain together two readers.
///
/// This struct is generally created by calling [`chain()`][chain] on a reader.
/// Please see the documentation of `chain()` for more details.
///
/// [chain]: trait.Read.html#method.chain
pub struct Chain<T, U, E> {
    first: T,
    second: U,
    done_first: bool,
    _phantom: PhantomData<fn() -> E>,
}

// Workaround for no stable pub(super)
pub fn new<T, U, E>(first: T, second: U) -> Chain<T, U, E> {
    Chain {
        first: first, second: second, done_first: false,
        _phantom: PhantomData
    }
}

impl<T: Read, U: Read, E> Read for Chain<T, U, E>
    where E: From<T::Err> + From<U::Err>
{
    type Err = E;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, E> {
        if !self.done_first {
            match try!(self.first.read(buf)) {
                0 => { self.done_first = true; }
                n => return Ok(n),
            }
        }
        self.second.read(buf).map_err(From::from)
    }
}
/*
impl<T: BufRead, U: BufRead> BufRead for Chain<T, U> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if !self.done_first {
            match try!(self.first.fill_buf()) {
                buf if buf.len() == 0 => { self.done_first = true; }
                buf => return Ok(buf),
            }
        }
        self.second.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        if !self.done_first {
            self.first.consume(amt)
        } else {
            self.second.consume(amt)
        }
    }
}
*/
