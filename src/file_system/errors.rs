use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RecordDuplication {
    details: String,
}

impl RecordDuplication {
    pub fn new(msg: &str) -> RecordDuplication {
        RecordDuplication {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for RecordDuplication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RecordDuplication {
    fn description(&self) -> &str {
        &self.details
    }
}
