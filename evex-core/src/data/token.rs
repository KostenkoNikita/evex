use std::collections::HashMap;
use crate::data::definitions::{OperatorAssociativity, OperatorType, OperatorDefinition};
use crate::math::types::{Value, Number};
use crate::errors::internal::parsing_errors::OperatorParsingError;

pub trait TokenDataSource<T: Value> {
    fn get_operator_definition(&self, s: &str) -> Result<&OperatorDefinition<T>, OperatorParsingError>;
}

pub struct DefaultDataSource<T: Value> {
    operators_table: HashMap<String, OperatorDefinition<T>>,
}

impl DefaultDataSource<Number> {
    pub fn new() -> DefaultDataSource<Number> {
        let mut operators_table: HashMap<String, OperatorDefinition<Number>> = HashMap::new();

        operators_table.insert(String::from("+"), OperatorDefinition {
            priority: 1,
            operator_type: OperatorType::Binary(Box::new(|x1, x2| {
                return x1 + x2;
            })),
            associativity: OperatorAssociativity::Left
        });

        operators_table.insert(String::from("-"), OperatorDefinition {
            priority: 1,
            operator_type: OperatorType::Binary(Box::new(|x1, x2| {
                return x1 - x2;
            })),
            associativity: OperatorAssociativity::Left
        });

        operators_table.insert(String::from("*"), OperatorDefinition {
            priority: 2,
            operator_type: OperatorType::Binary(Box::new(|x1, x2| {
                return x1 * x2;
            })),
            associativity: OperatorAssociativity::Left
        });

        operators_table.insert(String::from("/"), OperatorDefinition {
            priority: 2,
            operator_type: OperatorType::Binary(Box::new(|x1, x2| {
                return x1 / x2;
            })),
            associativity: OperatorAssociativity::Left
        });

        operators_table.insert(String::from("^"), OperatorDefinition {
            priority: 3,
            operator_type: OperatorType::Binary(Box::new(|x1, x2| {
                return x1.powf(x2);
            })),
            associativity: OperatorAssociativity::Left
        });

        return DefaultDataSource {
            operators_table,
        };
    }
}

impl TokenDataSource<Number> for DefaultDataSource<Number> {
    fn get_operator_definition(&self, s: &str) -> Result<&OperatorDefinition<Number>, OperatorParsingError> {
        let result = self.operators_table.get(s);

        return match result {
            Some(definition) => Ok(definition),
            None => Err(OperatorParsingError::new(s)),
        };
    }
}