#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    #[must_use] 
    pub fn new(details: String) -> Error {
        Error { details }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
