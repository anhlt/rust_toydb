use crate::file::File;

use std::io::Result;
use std::io::{Cursor, Error as IOError, ErrorKind, Read, Seek, SeekFrom, Write};
pub struct InmemFile {
    contents: Cursor<Vec<u8>>,
}

impl InmemFile {}

impl Default for InmemFile {
    fn default() -> Self {
        Self {
            contents: Cursor::new(vec![]),
        }
    }
    // add code here
}

impl File for InmemFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let pos = self.contents.position();
        self.contents
            .set_position(self.contents.get_ref().len() as u64);
        let r = self.contents.write(buf);
        self.contents.set_position(pos);
        r
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        self.contents.set_position(0);
        Ok(())
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.contents.seek(pos)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.contents.read(buf)
    }

    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.contents.set_position(0);
        self.contents.read_to_end(buf)
    }

    fn len(&self) -> Result<u64> {
        Ok(self.contents.get_ref().len() as u64)
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        if buf.is_empty() {
            Ok(0)
        } else {
            let inner = self.contents.get_ref();
            let length = inner.len() as u64;
            if offset > length - 1 {
                return Ok(0);
            }
            let extract = if buf.len() as u64 + offset > length {
                return Err(IOError::new(ErrorKind::UnexpectedEof, "EOF"));
            } else {
                buf.len()
            };
            buf.copy_from_slice(&inner.as_slice()[offset as usize..offset as usize + extract]);
            Ok(extract as usize)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::InmemFile;
    use crate::file::File;
    impl InmemFile {
        // add code here
        fn post_and_data(&self) -> (u64, &[u8]) {
            (self.contents.position(), self.contents.get_ref().as_slice())
        }
    }

    #[test]
    fn test_mem_file_read_write() {
        assert_eq!(2 + 2, 4);
        let mut f = InmemFile::default();
        let written1 = f.write(b"h1ello world").unwrap();
        assert_eq!(written1, 12);
        let written2 = f.write(b"|hello world").unwrap();
        assert_eq!(written2, 12);

        let (pos, data) = f.post_and_data();
        assert_eq!(pos, 0);
        assert_eq!(
            String::from_utf8(Vec::from(data)).unwrap(),
            "h1ello world|hello world"
        );

        let mut read_buf = vec![0u8; 5];
        let read = f.read(&mut read_buf).unwrap();

        println!("{:?}", String::from_utf8(read_buf.clone()));
        assert_eq!(read, 5);
        let (pos, _) = f.post_and_data();
        assert_eq!(pos, 5);
        read_buf.clear();

        let all = f.read_all(&mut read_buf).unwrap();

        assert_eq!(all, written1 + written2);
        assert_eq!(
            String::from_utf8(read_buf.clone()).unwrap(),
            "h1ello world|hello world"
        );
    }
}
