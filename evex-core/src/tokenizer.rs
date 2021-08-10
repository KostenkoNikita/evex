use crate::tokens::{InputToken, TokenContentType};
use crate::parsing_helpers::*;
use crate::errors::tokenization_errors::InvalidTokenError;

pub fn tokenize(input: &str, mut callback: impl FnMut(InputToken)) -> Result<(),
    InvalidTokenError> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let length = input_chars.len() as i32;
    while i < length {
        let char = input_chars[i];

        if is_digit(&char) {
            let number_string = process_digit(&input_chars, &mut i);
            callback(InputToken::new(&number_string, TokenContentType::Numeric, i));
        } else if char.is_alphabetic() {
            let alphanumeric_string = process_alpha_char(&input_chars, &mut i);
            callback(InputToken::new(&alphanumeric_string, TokenContentType::Alphabetic, i));
        } else if is_operator(&char) {
            let operator_string = process_operator(&input_chars, &mut i);
            callback(InputToken::new(&operator_string, TokenContentType::Operator, i));
        } else if is_opening_bracket(&char) {
            callback(InputToken::new(&char.to_string(), TokenContentType::OpeningBracket, i));
        } else if is_closing_bracket(&char) {
            callback(InputToken::new(&char.to_string(), TokenContentType::ClosingBracket, i));
        } else if is_separator(&char) {
            callback(InputToken::new(&char.to_string(), TokenContentType::Separator, i));
        } else if !char.is_whitespace() {
            let err = InvalidTokenError::new(i, char);
            return Err(err);
        }

        i += 1;
    }

    return Ok(());
}

fn process_digit(input_chars: &Vec<char>, outer_index: &mut i32) -> String {
    let mut index = *outer_index;
    let mut result = String::new();
    let length = input_chars.len() as i32;

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

fn process_alpha_char(input_chars: &Vec<char>, outer_index: &mut i32) -> String {
    let mut index = *outer_index;
    let mut result = String::new();
    let length = input_chars.len() as i32;

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

fn process_operator(input_chars: &Vec<char>, outer_index: &mut i32) -> String {
    let mut index = *outer_index;
    let length = input_chars.len() as i32;

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
