pub trait WriteableRecord {
    fn contents(&self) -> &[u8];
}

#[cfg(test)]
mod tests {

    use super::WriteableRecord;

    pub struct PlainStringRecord {
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
        // add code here

    }

    #[test]
    fn test_init_plain_string_record() {
        let record = PlainStringRecord::new("Helloworld");
        let content = record.contents();
        let len = content.len();
        assert_eq!(len, 10)
    }
}
