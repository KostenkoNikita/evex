use crate::tokens::input_token::InputToken;
use crate::errors::internal::parsing_errors::{BracketParsingError};

pub enum BracketType {
    Opening,
    Closing,
}

pub struct BracketToken {
    content: String,
    position: i32,
    bracket_type: BracketType,
}

impl BracketToken {
    pub fn new(s: &str, position: i32) -> Result<BracketToken, BracketParsingError> {
        let bracket_type = match s {
            "(" => BracketType::Opening,
            ")" => BracketType::Closing,
            _ => return Err(BracketParsingError::new(s)),
        };

        let result = BracketToken {
            content: String::from(s),
            position,
            bracket_type,
        };

        return Ok(result);
    }
}

impl InputToken for BracketToken {
    fn get_content(&self) -> &str {
        return &self.content;
    }

    fn get_position(&self) -> i32 {
        return self.position;
    }
}
