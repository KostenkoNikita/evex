use crate::tokens::input_token::InputToken;
use crate::errors::internal::parsing_errors::{BracketParsingError, InternalError};
use crate::tokens::TokenConstructor;

pub enum BracketType {
    Opening,
    Closing,
}

pub struct BracketToken {
    content: String,
    original_position: i32,
    bracket_type: BracketType,
}

impl TokenConstructor for BracketToken {
    fn new(s: &str, original_position: i32) -> Result<BracketToken, BracketParsingError> {
        let bracket_type = match s {
            "(" => BracketType::Opening,
            ")" => BracketType::Closing,
            _ => return Err(BracketParsingError::new(s)),
        };

        let result = BracketToken {
            content: String::from(s),
            original_position,
            bracket_type,
        };

        return Ok(result);
    }
}

impl InputToken for BracketToken {
    fn get_content(&self) -> &str {
        return &self.content;
    }

    fn get_original_position(&self) -> i32 {
        return self.original_position;
    }
}
