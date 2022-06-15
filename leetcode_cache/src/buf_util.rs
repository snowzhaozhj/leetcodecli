use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

use anyhow::Result;

pub(crate) struct BufReaderWithPos<T: Read + Seek> {
    pub(crate) reader: BufReader<T>,
    pub(crate) pos: u64,
}

pub(crate) struct BufWriterWithPos<T: Write + Seek> {
    pub(crate) writer: BufWriter<T>,
    pub(crate) pos: u64,
}

/* Implement BufReaderWithPos */

impl<T: Read + Seek> BufReaderWithPos<T> {
    pub(crate) fn new(mut file: T) -> Result<Self> {
        let pos = file.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(file),
            pos,
        })
    }
}

impl<T: Read + Seek> Read for BufReaderWithPos<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<T: Read + Seek> Seek for BufReaderWithPos<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

/* Implement BufWriterWithPos */

impl<T: Write + Seek> BufWriterWithPos<T> {
    pub(crate) fn new(mut file: T) -> Result<Self> {
        let pos = file.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(file),
            pos,
        })
    }
}

impl<T: Write + Seek> Write for BufWriterWithPos<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<T: Write + Seek> Seek for BufWriterWithPos<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}
