mod lexer;

use std::{env::args, fs::read_to_string, path::Path};

use lexer::{
    lexer::Lexer,
    token::{Token, TokenType},
};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Error: No file specified");
        return;
    }

    let file_path: &Path = Path::new(&args[1]);

    if !file_path.exists() {
        println!("Error: file does not exist");
        return;
    }

    let file_content: String = read_to_string(file_path).expect("Error: failed to read file");

    let lexer: Lexer = Lexer::new(&file_content);

    let mut tokens: Vec<Token> = Vec::new();

    for token in lexer {
        if token.token_type != TokenType::Space {
            tokens.push(token);
        }
    }

    for token in tokens {
        println!("{:?}", token);
    }
}
