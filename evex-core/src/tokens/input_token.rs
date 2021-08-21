use std::fmt::{Display, Formatter, Result as IoResult};

pub trait InputToken {
    fn get_content(&self) -> &str;

    fn get_position(&self) -> i32;
}

impl Display for dyn InputToken {
    fn fmt(&self, f: &mut Formatter) -> IoResult {
        return write!(f, "{}", self.get_content());
    }
}
