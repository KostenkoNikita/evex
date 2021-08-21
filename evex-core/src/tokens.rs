pub use crate::tokens::value_token::ValueToken;
pub use crate::tokens::operator_token::OperatorToken;
pub use crate::tokens::bracket_token::BracketToken;
use crate::math::types::{Value, Number};
use crate::data::token::TokenDataSource;
use crate::errors::tokenization_errors::UnresolvedTokenError;
use std::fmt;

pub mod value_token;
pub mod input_token;
pub mod operator_token;
pub mod bracket_token;

#[derive(Debug)]
pub enum TokenContentType {
    Numeric,
    Alphabetic,
    Separator,
    Operator,
    OpeningBracket,
    ClosingBracket,
}

pub enum Token<'a, TValue: Value> {
    Value(ValueToken<TValue>),
    Operator(OperatorToken<'a, TValue>),
    Bracket(BracketToken),
}

impl Token<'_, Number> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl fmt::Debug for Token<'_, Number> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}

impl fmt::Display for Token<'_, Number> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }
}

pub fn process_input_token<'a, S: TokenDataSource<Number>>(s: &'a str, position: i32, content_type: TokenContentType, data_source: &'a S) -> Result<Token<'a, Number>, UnresolvedTokenError> {
    return match content_type {
        TokenContentType::Numeric => process_numeric_input(s, position),
        TokenContentType::Operator => process_operator(s, position, data_source),
        TokenContentType::OpeningBracket | TokenContentType::ClosingBracket => process_bracket(s, position),
        _ => Err(UnresolvedTokenError::new(s, position, None)),
    }
}

fn process_numeric_input(s: &str, position: i32) -> Result<Token<Number>, UnresolvedTokenError> {
    let value_token_result = ValueToken::new(s, position);

    return match value_token_result {
        Ok(value_token) => Ok(Token::Value(value_token)),
        Err(value_string_parsing_err) => Err(UnresolvedTokenError::new(s, position, Some(Box::new(value_string_parsing_err))))
    }
}

fn process_bracket(s: &str, position: i32) -> Result<Token<Number>, UnresolvedTokenError> {
    let bracket_token_result = BracketToken::new(s, position);

    return match bracket_token_result {
        Ok(bracket_token) => Ok(Token::Bracket(bracket_token)),
        Err(bracket_parsing_err) => Err(UnresolvedTokenError::new(s, position, Some(Box::new(bracket_parsing_err))))
    }
}

fn process_operator<'a, S: TokenDataSource<Number>>(s: &str, position: i32, data_source: &'a S) -> Result<Token<'a, Number>, UnresolvedTokenError> {
    let operator_definition_result = data_source.get_operator_definition(s);

    return match operator_definition_result {
        Ok(operator_definition) => {
            let operator_token = OperatorToken::new(s, position, operator_definition);
            return Ok(Token::Operator(operator_token));
        },
        Err(operator_parsing_err) => Err(UnresolvedTokenError::new(s, position, Some(Box::new(operator_parsing_err))))
    };
}