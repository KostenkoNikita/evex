use crate::errors::internal::parsing_errors::ValueStringParsingError;
use std::num::ParseFloatError;
use std::str::FromStr;

pub trait Value: Sized {
    fn get_as_string(&self) -> String;

    fn from_string(s: &str) -> Result<Self, ValueStringParsingError<ParseFloatError>> where Self: Sized;

    fn clone_value(&self) -> Self;
}

pub type Number = f64;

impl Value for Number {
    fn get_as_string(&self) -> String {
        return self.to_string();
    }

    fn from_string(s: &str) -> Result<Number, ValueStringParsingError<ParseFloatError>> {
        let result = <f64>::from_str(s);
        return match result {
            Ok(parsed_value) => Ok(parsed_value as Number),
            Err(err) => Err(ValueStringParsingError::new(s, Some(err))),
        }
    }

    fn clone_value(&self) -> Number {
        return *self;
    }
}
