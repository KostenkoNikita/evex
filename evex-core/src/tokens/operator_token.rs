use crate::tokens::input_token::InputToken;
use crate::math::types::Value;
use crate::data::definitions::OperatorDefinition;

pub struct OperatorToken<'a, T: Value> {
    content: String,
    position: i32,
    definition: &'a OperatorDefinition<T>,
}

impl<T: Value> OperatorToken<'_, T> {
    pub fn new<'a>(s: &str, position: i32, definition: &'a OperatorDefinition<T>) -> OperatorToken<'a, T> {
        return OperatorToken {
            content: String::from(s),
            position,
            definition,
        };
    }
}

impl<T: Value> InputToken for OperatorToken<'_, T> {
    fn get_content(&self) -> &str {
        return &self.content;
    }

    fn get_position(&self) -> i32 {
        return self.position;
    }
}
