use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reads: usize,
    bytes_through: usize,
    wrapped: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            reads: 0,
            bytes_through: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.wrapped.read(buf) {
            Ok(len) => {
                self.bytes_through += len;
                self.reads += 1;

                Ok(len)
            }
            Err(err) => Err(err),
        }
    }
}

pub struct WriteStats<W> {
    writes: usize,
    bytes_through: usize,
    wrapped: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writes: 0,
            bytes_through: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.wrapped.write(buf) {
            Ok(len) => {
                self.bytes_through += len;
                self.writes += 1;

                Ok(len)
            }
            Err(err) => Err(err),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
