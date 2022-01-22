use std::env::args;
use std::fs;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = args().collect();

    let file_content = fs::read_to_string(&args[1]).unwrap();
}
