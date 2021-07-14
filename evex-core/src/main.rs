use crate::tokenizer::tokenize;

mod tokenizer;
mod token;
mod invalid_token_error;
mod parsing_helpers;

fn main() {
    let tokenized_string = tokenize("1 +2!-:fn(45 )^42!!").unwrap();
    for token in tokenized_string {
        println!("{}", token);
    }
}
