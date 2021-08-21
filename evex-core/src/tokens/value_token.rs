use std::num::ParseFloatError;

use crate::tokens::input_token::InputToken;
use crate::math::types::{Number, Value};
use crate::errors::internal::parsing_errors::ValueStringParsingError;

pub struct ValueToken<T: Value> {
    content: String,
    position: i32,
    value: T,
}

impl ValueToken<Number> {
    pub fn new(s: &str, position: i32) -> Result<ValueToken<Number>, ValueStringParsingError<ParseFloatError>> {
        let value: Number = Number::from_string(s)?;
        let result = ValueToken {
            content: String::from(s),
            position,
            value,
        };

        return Ok(result);
    }
}

impl InputToken for ValueToken<Number> {
    fn get_content(&self) -> &str {
        return &self.content;
    }

    fn get_position(&self) -> i32 {
        return self.position;
    }
}
