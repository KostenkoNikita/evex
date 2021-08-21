use crate::math::types::Value;

pub enum OperatorType<T: Value> {
    Unary(Box<dyn Fn(T) -> T>),
    Binary(Box<dyn Fn(T, T) -> T>),
}

#[derive(Clone)]
pub enum OperatorAssociativity {
    Left,
    Right,
}

pub struct OperatorDefinition<T: Value> {
    pub priority: u8,
    pub operator_type: OperatorType<T>,
    pub associativity: OperatorAssociativity,
}