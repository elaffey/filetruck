use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TruckError {
    details: String,
}

impl TruckError {
    pub fn new(details: String) -> TruckError {
        TruckError { details }
    }
}

impl fmt::Display for TruckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self.details)
    }
}

impl Error for TruckError {
    fn description(&self) -> &str {
        &self.details
    }
}
