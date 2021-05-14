mod mem;
mod record;

use record::WriteableRecord;
use std::collections::HashMap;
use std::io::Result;
use std::io::SeekFrom;
use mem::InmemFile;

#[derive(Debug)]
pub struct SlottedPage {
    buffer : InmemFile,
    look_up_table: HashMap<u32, usize>,
    free_space_pointer: usize,
}

impl SlottedPage {
    fn add(&mut self, record: &dyn WriteableRecord, record_id: u32) {
        let record_size = record.contents().len();
        self.look_up_table
            .insert(record_id, self.free_space_pointer + record_size);
        self.free_space_pointer += record_size;
    }
}

struct Header {
    slots: Vec<Slot>,
}

struct Slot {
    length: usize,
    offset: usize,
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
