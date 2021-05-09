use std::io::Result;

use std::io::SeekFrom;
mod mem;
mod record;

#[derive(Debug)]
pub struct SlottedPage {}

struct Header<T> {
    slot_count: u8,
    first_free_slot: Box<Slot<T>>,
}

struct Slot<T> {
    length: u8,
    offset: Option<T>,
}

struct TID {
    page_id: u32,
    slot_id: u8,
}

pub trait File {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn close(&mut self) -> Result<()>;
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize>;
    fn len(&self) -> Result<u64>;

    fn is_empty(&self) -> bool {
        if let Ok(lenght) = self.len() {
            return lenght == 0;
        }
        false
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize>;
}
