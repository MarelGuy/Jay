/*
Jay main
Copyright (C) 2022  Loris Cuntreri
*/
use chrono::{Datelike, Utc};
use lexer::{lexer::Lexer, token::TokenType};
use parser::parser::Parser;
use std::{env::args, fs::read_to_string, io::Write, path::Path};

mod lexer;
mod parser;

fn help() {
    println!("-h, --help: show this help message");
    println!("-v, --version: show version");
    println!("-c, --compiler: use the compiler");
}

fn version() {
    println!("Jay v0.0.0 (2022-016-03)");
}

fn interpreter() {
    println!("Jay version 0.0.0 (c) {}", Utc::now().date().year());

    loop {
        print!(">>> ");

        std::io::stdout().flush().expect("");

        let mut input: String = String::new(); // we use this variable as a buffer for the input

        std::io::stdin().read_line(&mut input).expect("");

        let lexer: Lexer = Lexer::new(&input);

        let mut tokens: Vec<lexer::token::Token> = Vec::new();

        for token in lexer {
            // This for loop is used to iterate over the tokens and push the items that are not whitespace or comments to the tokens vector
            if token.token_type != TokenType::CarriageReturn
                && token.token_type != TokenType::LineFeed
                && token.token_type != TokenType::Space
                && token.token_type != TokenType::Tab
                && token.token_type != TokenType::Comment
                && token.token_type != TokenType::BlockComment
            {
                tokens.push(token);
            }
        }

        let mut parser = Parser::new(tokens);

        parser.parse();
    }
}

fn compiler() {
    let args: Vec<String> = args().collect();

    println!("Jay version 0.0.0 (c) {}", Utc::now().date().year());

    // if we less than three arguments, it means that we only got: jay -c
    // meaning that we don't have any file in the arguments
    if args.len() < 3 {
        println!("Error: No file specified");
        return;
    }

    let file_path: &Path = Path::new(&args[2]);

    if !file_path.exists() {
        println!("Error: file does not exist");
        return;
    }

    // we read the content to the file to string avoiding the problem of distinguishing EOL and EOF
    let file_content: String = read_to_string(file_path).expect("Error: failed to read file");

    let lexer: Lexer = Lexer::new(&file_content);

    let mut tokens: Vec<lexer::token::Token> = Vec::new();

    for token in lexer {
        if token.token_type != TokenType::CarriageReturn
            && token.token_type != TokenType::LineFeed
            && token.token_type != TokenType::Space
            && token.token_type != TokenType::Tab
        {
            tokens.push(token);
        }
    }

    let mut parser = Parser::new(tokens);

    parser.parse();
}

fn main() {
    // match block for the arguments
    match args().nth(1) {
        Some(ref arg) if arg == "-c" => compiler(),
        Some(ref arg) if arg == "-v" => version(),
        Some(ref arg) if arg == "-h" => help(),
        Some(ref arg) if arg == "--compiler" => compiler(),
        Some(ref arg) if arg == "--version" => version(),
        Some(ref arg) if arg == "--help" => help(),
        _ => {
            interpreter();
        }
    }
}
