use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Token {
    content: String,
}

impl Token {
    pub fn new(s: &str) -> Token {
        return Token {
            content: String::from(s),
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{}", self.content);
    }
}
