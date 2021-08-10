use std::num::ParseFloatError;

use crate::tokens::input_token::InputToken;
use crate::helpers::parsing_helpers::ValueString;
use crate::math::types::{Number, Value};
use crate::errors::internal::parsing_errors::ValueStringParsingTokenError;
use crate::tokens::TokenConstructor;

pub struct ValueToken<T: Value> {
    content: String,
    original_position: i32,
    value: T,
}

impl TokenConstructor for ValueToken<Number> {
    fn new(s: &str, original_position: i32) -> Result<ValueToken<Number>, ValueStringParsingTokenError<ParseFloatError>> {
        let value: Number = Number::from_string(s)?;
        let result = ValueToken {
            content: String::from(s),
            original_position,
            value,
        };

        return Ok(result);
    }
}

impl InputToken for ValueToken<T> {
    fn get_content(&self) -> &str {
        return &self.content;
    }

    fn get_original_position(&self) -> i32 {
        return self.original_position;
    }
}
