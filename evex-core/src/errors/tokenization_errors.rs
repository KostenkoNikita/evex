use std::{error::Error, fmt};
use crate::errors::internal::parsing_errors::InternalError;

pub struct InvalidInputSymbolError {
    position: i32,
    caused_by: char,
}

impl Error for InvalidInputSymbolError {}

impl InvalidInputSymbolError {
    pub fn new(position: i32, c: char) -> InvalidInputSymbolError {
        return InvalidInputSymbolError {
            position,
            caused_by: c,
        };
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "unable to process an input symbol '{}' found at position {}", self.caused_by, self.position + 1);
    }
}

impl fmt::Display for InvalidInputSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}

impl fmt::Debug for InvalidInputSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}

pub struct UnresolvedTokenError {
    position: i32,
    caused_by: String,
    internal_error: Option<Box<dyn InternalError>>,
}

impl Error for UnresolvedTokenError {}

impl UnresolvedTokenError {
    pub fn new(s: &str, position: i32, internal_error: Option<Box<dyn InternalError>>) -> UnresolvedTokenError {
        return UnresolvedTokenError {
            position,
            caused_by: String::from(s),
            internal_error,
        };
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let internal_error_text = match &self.internal_error {
            None => String::new(),
            Some(internal_error) => format!(" error: {}", internal_error),
        };

        return write!(f, "unresolved token \"{}\" found at position {};{}", self.caused_by, self.position + 1, internal_error_text);
    }
}

impl fmt::Display for UnresolvedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}

impl fmt::Debug for UnresolvedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}
