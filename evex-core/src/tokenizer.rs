use crate::token::Token;
use crate::invalid_token_error::InvalidTokenError;
use crate::parsing_helpers::*;

pub fn tokenize(input: &str) -> Result<Vec<Token>, InvalidTokenError> {
    let mut result: Vec<Token> = Vec::new();

    let input_chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let length = input_chars.len();
    while i < length {
        let char = input_chars[i];

        if is_digit(&char) {
            let number_string = process_digit(&input_chars, &mut i);
            let token = Token::new(&number_string);
            result.push(token);
        } else if char.is_alphabetic() {
            let alphanumeric_string = process_alpha_char(&input_chars, &mut i);
            let token = Token::new(&alphanumeric_string);
            result.push(token);
        } else if is_operator(&char) {
            let operator_string = process_operator(&input_chars, &mut i);
            let token = Token::new(&operator_string);
            result.push(token);
        } else if is_opening_bracket(&char) || is_closing_bracket(&char) || is_separator(&char) {
            let token = Token::new(&char.to_string());
            result.push(token);
        } else if !char.is_whitespace() {
            let err = InvalidTokenError::new(i as i32, char);
            return Err(err);
        }

        i += 1;
    }

    return Ok(result);
}

fn process_digit(input_chars: &Vec<char>, outer_index: &mut usize) -> String {
    let mut index = *outer_index;

    let mut result = String::new();

    let mut char: char = input_chars[index];

    while is_digit_or_dot(&char) {
        result.push(char);

        index += 1;

        if index == input_chars.len() {
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

    let mut char: char = input_chars[index];
    while char.is_alphanumeric() {
        result.push(char);

        index += 1;

        if index == input_chars.len() {
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

    let mut next_char: char = input_chars[index];
    return if is_factorial(&next_char) {
        let mut result = String::new();

        loop {
            result.push(next_char);

            index = index + 1;

            if index == input_chars.len() {
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
