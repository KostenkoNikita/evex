use std::{error::Error, fmt};

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

pub struct ValueStringParsingError<E: Error> {
    caused_by: String,
    original_error: Option<E>,
}

impl<E: Error> ValueStringParsingError<E> {
    pub fn new(s: &str, original_error: Option<E>) -> ValueStringParsingError<E> {
        return ValueStringParsingError {
            caused_by: String::from(s),
            original_error,
        };
    }
}

impl<E: Error> InternalError for ValueStringParsingError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let original_error_text = match &self.original_error {
            Some(error) => format!(" due to an error: {}", error),
            None => String::new(),
        };

        return write!(f, "unable to process \"{}\" string as a value token{}",  self.caused_by, original_error_text);
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

pub struct OperatorParsingError {
    caused_by: String,
}

impl OperatorParsingError {
    pub fn new(s: &str) -> OperatorParsingError {
        return OperatorParsingError {
            caused_by: String::from(s),
        };
    }
}

impl InternalError for OperatorParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "unable to process \"{}\" string as an operator token", self.caused_by);
    }
}