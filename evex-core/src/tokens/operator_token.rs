use crate::tokens::input_token::InputToken;

pub enum OperatorType<T> {
    Unary(fn(T) -> T),
    Binary(fn(T, T) -> T),
}

pub enum OperatorAssociativity {
    Left,
    Right,
}

pub struct OperatorToken<T> {
    content: String,
    original_position: i32,
    priority: u8,
    operator_type: OperatorType<T>,
    associativity: OperatorAssociativity,
}

impl InputToken for OperatorToken<T> {
    fn get_content(&self) -> &str {
        return &self.content;
    }

    fn get_original_position(&self) -> i32 {
        return self.original_position;
    }
}
