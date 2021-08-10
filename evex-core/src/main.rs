use tokens::InputToken;

use crate::tokenizer::tokenize;

mod tokenizer;
mod parsing_helpers;
mod errors;
mod math;
mod tokens;
mod helpers;

fn main() {
    let mut tokenized_string: Vec<InputToken> = Vec::new();
    tokenize("1 +2!-fn(45 )^42!!", |token: InputToken| {
        tokenized_string.push(token);
    }).unwrap();

    for token in tokenized_string {
        println!("{}", token);
    }
}
