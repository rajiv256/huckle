use core::cmp;

use super::*;

/// Reader adaptor which limits the bytes read from an underlying reader.
///
/// This struct is generally created by calling [`take()`][take] on a reader.
/// Please see the documentation of `take()` for more details.
///
/// [take]: trait.Read.html#method.take
pub struct Take<T> {
    inner: T,
    limit: u64,
}

impl<T> Take<T> {
    /// Returns the number of bytes that can be read before this instance will
    /// return EOF.
    ///
    /// # Note
    ///
    /// This instance may reach EOF after reading fewer bytes than indicated by
    /// this method if the underlying `Read` instance reaches EOF.
    pub fn limit(&self) -> u64 { self.limit }
}

pub fn new<T>(inner: T, limit: u64) -> Take<T> {
    Take { inner: inner, limit: limit }
}

impl<T: Read> Read for Take<T> {
    type Err = T::Err;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, T::Err> {
        // Don't call into inner reader at all at EOF because it may still block
        if self.limit == 0 {
            return Ok(0);
        }

        let max = cmp::min(buf.len() as u64, self.limit) as usize;
        let n = try!(self.inner.read(&mut buf[..max]));
        self.limit -= n as u64;
        Ok(n)
    }
}

/*
impl<T: BufRead> BufRead for Take<T> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        // Don't call into inner reader at all at EOF because it may still block
        if self.limit == 0 {
            return Ok(&[]);
        }

        let buf = self.inner.fill_buf()?;
        let cap = cmp::min(buf.len() as u64, self.limit) as usize;
        Ok(&buf[..cap])
    }

    fn consume(&mut self, amt: usize) {
        // Don't let callers reset the limit by passing an overlarge value
        let amt = cmp::min(amt as u64, self.limit) as usize;
        self.limit -= amt as u64;
        self.inner.consume(amt);
    }
}
*/
