use super::*;

impl<'a, R: Read> Read for &'a mut R {
    type Err = R::Err;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, R::Err> {
        (**self).read(buf)
    }

    fn read_exact<E>(&mut self, buf: &mut [u8]) -> Result<(), E>
        where E: From<R::Err> + From<EndOfFile>
    {
        (**self).read_exact(buf)
    }
}

impl<'a, W: Write> Write for &'a mut W {
    type Err = W::Err;

    fn write(&mut self, buf: &[u8]) -> Result<usize, W::Err> {
        (**self).write(buf)
    }

    fn write_all<E>(&mut self, buf: &[u8]) -> Result<(), E>
        where E: From<W::Err> + From<EndOfFile>
    {
        (**self).write_all(buf)
    }
}

impl<'a, S: Seek> Seek for &'a mut S {
    type Err = S::Err;

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, S::Err> {
        (**self).seek(pos)
    }

    fn tell(&mut self) -> Result<u64, S::Err> {
        (**self).tell()
    }
}
