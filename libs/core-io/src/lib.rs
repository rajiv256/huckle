#![no_std]

extern crate void;

use core::fmt;

use void::{unreachable, Void};

pub use cursor::Cursor;
pub use chain::Chain;
pub use take::Take;

mod chain;
mod slice;
mod cursor;
mod wrapper;
mod take;

#[macro_export]
macro_rules! try {
    ($expr:expr) => (match $expr {
        ::core::result::Result::Ok(val) => val,
        ::core::result::Result::Err(err) => {
            return ::core::result::Result::Err(::core::convert::From::from(err))
        }
    })
}

#[derive(Copy, Clone, Debug)]
pub struct EndOfFile;

impl From<Void> for EndOfFile {
    fn from(v: Void) -> EndOfFile {
        unreachable(v)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OutOfBounds;

impl From<Void> for OutOfBounds {
    fn from(v: Void) -> OutOfBounds {
        unreachable(v)
    }
}

/// The `Read` trait allows for reading bytes from a source.
///
/// Implementors of the `Read` trait are sometimes called 'readers'.
///
/// Readers are defined by one required method, `read()`. Each call to `read`
/// will attempt to pull bytes from this source into a provided buffer. A
/// number of other methods are implemented in terms of `read()`, giving
/// implementors a number of ways to read bytes while only needing to implement
/// a single method.
pub trait Read {
    type Err;

    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    ///
    /// This function does not provide any guarantees about whether it blocks
    /// waiting for data, but if an object needs to block for a read but cannot
    /// it will typically signal this via an `Err` return value.
    ///
    /// If the return value of this method is `Ok(n)`, then it must be
    /// guaranteed that `0 <= n <= buf.len()`. A nonzero `n` value indicates
    /// that the buffer `buf` has been filled in with `n` bytes of data from this
    /// source. If `n` is `0`, then it can indicate one of two scenarios:
    ///
    /// 1. This reader has reached its "end of file" and will likely no longer
    ///    be able to produce bytes. Note that this does not mean that the
    ///    reader will *always* no longer be able to produce bytes.
    /// 2. The buffer specified was 0 bytes in length.
    ///
    /// No guarantees are provided about the contents of `buf` when this
    /// function is called, implementations cannot rely on any property of the
    /// contents of `buf` being true. It is recommended that implementations
    /// only write data to `buf` instead of reading its contents.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of I/O or other error, an error
    /// variant will be returned. If an error is returned then it must be
    /// guaranteed that no bytes were read.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Err>;

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// This function reads as many bytes as necessary to completely fill the
    /// specified buffer `buf`.
    ///
    /// No guarantees are provided about the contents of `buf` when this
    /// function is called, implementations cannot rely on any property of the
    /// contents of `buf` being true. It is recommended that implementations
    /// only write data to `buf` instead of reading its contents.
    ///
    /// # Errors
    ///
    /// If this function encounters an "end of file" before completely filling
    /// the buffer, it returns an error of `E::from(EndOfFile)`.  The contents
    /// of `buf` are unspecified in this case.
    ///
    /// If any other read error, `e`, is encountered then this function
    /// immediately returns with `E::from(e)`. The contents of `buf` are
    /// unspecified in this case.
    ///
    /// If this function returns an error, it is unspecified how many bytes it
    /// has read, but it will never read more than would be necessary to
    /// completely fill the buffer.
    fn read_exact<E>(&mut self, mut buf: &mut [u8]) -> Result<(), E>
        where E: From<Self::Err> + From<EndOfFile>
    {
        while buf.len() > 0 {
            match try!(self.read(&mut buf)) {
                0 => return Err(E::from(EndOfFile)),
                n => {
                    let tmp = buf;
                    buf = &mut tmp[n..]
                }
            }
        }
        Ok(())
    }

    /// Creates a "by reference" adaptor for this instance of `Read`.
    ///
    /// The returned adaptor also implements `Read` and will simply borrow this
    /// current reader.
    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }

    /// Creates an adaptor which will chain this stream with another.
    ///
    /// The returned `Read` instance will first read all bytes from this object
    /// until EOF is encountered. Afterwards the output is equivalent to the
    /// output of `next`.
    fn chain<R: Read, E>(self, next: R) -> Chain<Self, R, E>
        where Self: Sized, E: From<Self::Err> + From<R::Err>
    {
        chain::new(self, next)
    }

    /// Creates an adaptor which will read at most `limit` bytes from it.
    ///
    /// This function returns a new instance of `Read` which will read at most
    /// `limit` bytes, after which it will always return EOF (`Ok(0)`). Any
    /// read errors will not count towards the number of bytes read and future
    /// calls to `read` may succeed.
    ///
    /// # Examples
    ///
    /// [`File`][file]s implement `Read`:
    ///
    /// [file]: ../fs/struct.File.html
    ///
    /// ```
    /// use std::io;
    /// use std::io::prelude::*;
    /// use std::fs::File;
    ///
    /// # fn foo() -> io::Result<()> {
    /// let mut f = try!(File::open("foo.txt"));
    /// let mut buffer = [0; 5];
    ///
    /// // read at most five bytes
    /// let mut handle = f.take(5);
    ///
    /// try!(handle.read(&mut buffer));
    /// # Ok(())
    /// # }
    /// ```
    fn take(self, limit: u64) -> Take<Self> where Self: Sized {
         take::new(self, limit)
    }
}


pub trait Write {
    type Err;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Err>;

    fn write_all<E>(&mut self, mut buf: &[u8]) -> Result<(), E>
        where E: From<Self::Err> + From<EndOfFile>
    {
        while buf.len() > 0 {
            match try!(self.write(buf)) {
                0 => return Err(E::from(EndOfFile)),
                n => buf = &buf[n..]
            }
        }
        Ok(())
    }

    fn write_fmt<E>(&mut self, fmt: fmt::Arguments) -> Result<(), E>
        where E: From<Self::Err> + From<EndOfFile>
    {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adaptor<'a, T: ?Sized + 'a, E> {
            inner: &'a mut T,
            result: Result<(), E>,
        }

        impl<'a, T: ?Sized, F> fmt::Write for Adaptor<'a, T, F>
            where T: Write,
                  F: From<EndOfFile> + From<T::Err>
        {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.result = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adaptor { inner: self, result: Ok(()) };
        let _ = fmt::write(&mut output, fmt);
        output.result
    }
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait Seek {
    type Err;

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Err>;

    fn tell(&mut self) -> Result<u64, Self::Err> {
        self.seek(SeekFrom::Current(0))
    }
}
