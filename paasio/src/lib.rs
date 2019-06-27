#![warn(clippy::pedantic)]

use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
    reader: R,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            bytes_through: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
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
        let bytes_read = self.reader.read(buf)?;
        self.bytes_through += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W: Write> {
    writer: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            bytes_through: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
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
        let bytes_written = self.writer.write(buf)?;
        self.bytes_through += bytes_written;
        self.writes += 1;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
