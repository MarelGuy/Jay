use std::env::args;
use std::fs;

mod lexer;
mod token;



fn main() {
    let args: Vec<String> = args().collect();
    let mut actual_lexer: lexer::Lexer = lexer::Lexer::new(fs::read_to_string(&&args[1]).unwrap());
    print!("{:?}", actual_lexer.make_tokens());
}
