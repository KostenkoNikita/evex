use crate::tokenizer::tokenize;
use crate::tokens::Token;
use crate::math::types::Number;
use crate::data::token::DefaultDataSource;

mod tokenizer;
mod parsing_helpers;
mod errors;
mod math;
mod tokens;
mod data;

fn main() {
    let data_source: DefaultDataSource<Number> = DefaultDataSource::new();

    let mut tokenized_string: Vec<Token<Number>> = Vec::new();
    tokenize("1 +2!-fn(45 )^42!!", |token: Token<Number>| {
        tokenized_string.push(token);
    }, &data_source).unwrap();

    for token in tokenized_string {
        println!("{}", token);
    }
}
