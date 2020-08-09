#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(details: String) -> Error {
        Error { details }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}
