use crate::tokens::{TokenContentType, Token, process_input_token};
use crate::parsing_helpers::*;
use crate::errors::tokenization_errors::{InvalidInputSymbolError, UnresolvedTokenError};
use crate::math::types::Number;
use crate::data::token::TokenDataSource;
use std::error::Error;

pub fn tokenize<S: TokenDataSource<Number>>(input: &str, mut callback: impl FnMut(Token<Number>), data_source: &S) -> Result<(), Box<dyn Error>> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let length = input_chars.len();
    while i < length {
        let char = input_chars[i];

        let mut next_token_result: Option<Result<Token<Number>, UnresolvedTokenError>> = None;

        if is_digit(&char) {
            let number_string = process_digit(&input_chars, &mut i);
            let token_result = process_input_token(&number_string.clone(), i as i32, TokenContentType::Numeric, data_source);
            next_token_result = Some(token_result);
        } else if char.is_alphabetic() {
            let alphanumeric_string = process_alpha_char(&input_chars, &mut i);
            let token_result = process_input_token(&alphanumeric_string, i as i32, TokenContentType::Alphabetic, data_source);
            next_token_result = Some(token_result);
        } else if is_operator(&char) {
            let operator_string = process_operator(&input_chars, &mut i);
            let token_result = process_input_token(&operator_string, i as i32, TokenContentType::Operator, data_source);
            next_token_result = Some(token_result);
        } else if is_opening_bracket(&char) {
            let token_result = process_input_token(&char.to_string(), i as i32, TokenContentType::OpeningBracket, data_source);
            next_token_result = Some(token_result);
        } else if is_closing_bracket(&char) {
            let token_result = process_input_token(&char.to_string(), i as i32, TokenContentType::ClosingBracket, data_source);
            next_token_result = Some(token_result);
        } else if is_separator(&char) {
            let token_result = process_input_token(&char.to_string(), i as i32, TokenContentType::Separator, data_source);
            next_token_result = Some(token_result);
        } else if !char.is_whitespace() {
            let err = InvalidInputSymbolError::new(i as i32, char);
            return Err(Box::new(err));
        }

        if let Some(token_result) = next_token_result {
            if let Ok(token) = token_result {
                callback(token);
            } else if let Err(e) = token_result {
                return Err(Box::new(e));
            }
        }

        i += 1;
    }

    return Ok(());
}

fn process_digit(input_chars: &Vec<char>, outer_index: &mut usize) -> String {
    let mut index = *outer_index;
    let mut result = String::new();
    let length = input_chars.len();

    let mut char: char = input_chars[index];
    while is_digit_or_dot(&char) {
        result.push(char);

        index += 1;

        if index == length {
            break;
        }

        char = input_chars[index];
        if is_exponent(&char) {
            let char_after_exp: char = input_chars[index + 1];
            let char_after_char_after_exp: char = input_chars[index + 2];
            if is_plus_or_minus(&char_after_exp) && is_digit(&char_after_char_after_exp) {
                result.push(char);
                result.push(char_after_exp);
                result.push(char_after_char_after_exp);

                index += 3;

                char = input_chars[index];
            } else if is_digit(&char_after_exp) {
                result.push(char);
                result.push(char_after_exp);

                index += 2;

                char = input_chars[index];
            }
        }
    }

    index -= 1;

    *outer_index = index;

    return result;
}

fn process_alpha_char(input_chars: &Vec<char>, outer_index: &mut usize) -> String {
    let mut index = *outer_index;
    let mut result = String::new();
    let length = input_chars.len();

    let mut char: char = input_chars[index];
    while char.is_alphanumeric() {
        result.push(char);

        index += 1;

        if index == length {
            break;
        }

        char = input_chars[index];
    }

    index -= 1;

    *outer_index = index;

    return result;
}

fn process_operator(input_chars: &Vec<char>, outer_index: &mut usize) -> String {
    let mut index = *outer_index;
    let length = input_chars.len();

    let mut next_char: char = input_chars[index];
    return if is_factorial(&next_char) {
        let mut result = String::new();

        loop {
            result.push(next_char);

            index = index + 1;

            if index == length {
                break;
            }

            next_char = input_chars[index];
            if !is_factorial(&next_char) {
                break;
            }
        }

        index -= 1;

        *outer_index = index;

        result
    } else {
        String::from(next_char)
    }
}
