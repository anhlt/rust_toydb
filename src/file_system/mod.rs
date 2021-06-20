pub mod errors;
pub mod mem;
pub mod record;

use mem::InmemFile;
use record::WriteableRecord;
use std::collections::HashMap;
use std::io::Error;
use std::io::SeekFrom;

use self::errors::RecordDuplication;

#[derive(Debug)]
pub struct SlottedPage {
    pub buffer: InmemFile,
    look_up_table: HashMap<u32, usize>,
    free_space_pointer: usize,
}

impl SlottedPage {
    pub fn new(buffer: InmemFile) -> SlottedPage {
        SlottedPage {
            buffer: buffer,
            look_up_table: HashMap::new(),
            free_space_pointer: 0,
        }
    }

    pub fn add(
        &mut self,
        record: &dyn WriteableRecord,
        record_id: u32,
    ) -> Result<(), RecordDuplication> {
        let record_size = record.contents().len();
        match self.look_up_table.get(&record_id) {
            None => {
                self.look_up_table
                    .insert(record_id, self.free_space_pointer + record_size);

                self.free_space_pointer += record_size;
                Ok({})
            }
            Some(_) => Err(errors::RecordDuplication::new("Duplicated")),
        }
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
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;
    fn close(&mut self) -> Result<(), Error>;
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error>;
    fn len(&self) -> Result<u64, Error>;

    fn is_empty(&self) -> bool {
        if let Ok(lenght) = self.len() {
            return lenght == 0;
        }
        false
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize, Error>;
}

#[cfg(test)]
mod tests {}
