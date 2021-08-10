use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct InvalidTokenError {
    position: i32,
    caused_by: char,
}

impl Error for InvalidTokenError {}

impl InvalidTokenError {
    pub fn new(position: i32, c: char) -> InvalidTokenError {
        return InvalidTokenError {
            position,
            caused_by: c,
        };
    }
}

impl fmt::Display for InvalidTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "unable to process token '{}' found at position {}", self.caused_by, self.position + 1);
    }
}
