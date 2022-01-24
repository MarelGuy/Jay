use std::env::args;
use std::fs;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];

    let file_content = fs::read_to_string(&filename).unwrap();
    let mut actual_lexer: lexer::Lexer = lexer::Lexer::new(file_content);
    print!("{:?}", actual_lexer.make_tokens());
}
