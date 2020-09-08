use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PetrovichError {
    message: String,
}

impl PetrovichError {
    pub fn new(message: &str) -> PetrovichError {
        PetrovichError {
            message: message.to_string(),
        }
    }
}
impl fmt::Display for PetrovichError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for PetrovichError {}
