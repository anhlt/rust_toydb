use std::collections::HashMap;

use toydb::file_system::mem::InmemFile;
use toydb::file_system::record::WriteableRecord;
use toydb::file_system::{File, SlottedPage};

#[test]
fn test_add() {
    #[derive(Debug)]
    struct PlainStringRecord {
        content: String,
    }

    impl PlainStringRecord {
        pub fn new(input: &str) -> Self {
            Self {
                content: String::from(input),
            }
        }
    }

    impl WriteableRecord for PlainStringRecord {
        fn contents(&self) -> &[u8] {
            self.content.as_bytes()
        }
    }

    let f = InmemFile::default();

    let mut page = SlottedPage::new(f);

    page.add(&PlainStringRecord::new("Helloword"), 12);

    assert!(false)
}
