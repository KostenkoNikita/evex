use std::{error::Error, fmt};
use crate::tokens::ValueToken;

pub trait InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "an internal error occurred");
    }
}

impl fmt::Debug for dyn InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}

impl fmt::Display for dyn InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}

impl Error for dyn InternalError {}

pub struct ValueStringParsingTokenError<E: Error> {
    caused_by: String,
    original_error: Option(E),
}

impl ValueStringParsingTokenError<E> {
    pub fn new(s: &str, original_error: E) -> ValueStringParsingTokenError<E> {
        return ValueStringParsingTokenError {
            caused_by: String::from(s),
            original_error: Some(original_error),
        };
    }
}

impl InternalError for ValueStringParsingTokenError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "unable to process \"{}\" string as a value token due to the next error: {}", self.caused_by, self.original_error);
    }
}

pub struct BracketParsingError {
    caused_by: String,
}

impl BracketParsingError {
    pub fn new(s: &str) -> BracketParsingError {
        return BracketParsingError {
            caused_by: String::from(s),
        };
    }
}

impl InternalError for BracketParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "unable to process \"{}\" string as a bracket token", self.caused_by);
    }
}
