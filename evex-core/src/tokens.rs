pub use crate::tokens::value_token::ValueToken;
pub use crate::tokens::operator_token::OperatorToken;
pub use crate::tokens::bracket_token::BracketToken;
use crate::math::types::Value;
use crate::errors::internal::parsing_errors::InternalError;

mod value_token;
mod input_token;
mod operator_token;
mod bracket_token;

#[derive(Debug)]
pub enum TokenContentType {
    Numeric,
    Alphabetic,
    Separator,
    Operator,
    OpeningBracket,
    ClosingBracket,
}

pub enum Token<TValue: Value> {
    Value(ValueToken<TValue>),
    Operator(OperatorToken<TValue>),
    Bracket(BracketToken),
}

pub trait TokenConstructor {
    fn new(s: &str, original_position: i32) -> Result<Self, dyn InternalError>;
}
