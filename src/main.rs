/*
Jay main
Copyright (C) 2022  Loris Cuntreri
*/
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::{env::args, fs::read_to_string, /* io::Write, */ path::Path};

use crate::lexer::token::{Token, TokenType};

mod lexer;
mod parser;

fn help() {
    println!("-h, --help: show this help message");
    println!("-v, --version: show version");
}

fn version() {
    println!("Jay v0.0.0 (2022-016-03)");
}

fn run(input: &str) {
    let lexer: Lexer = Lexer::new(input);

    let mut tokens: Vec<Token> = Vec::new();

    for token in lexer {
        if token.token_type != TokenType::Space
            && token.token_type != TokenType::LineFeed
            && token.token_type != TokenType::CarriageReturn
        {
            tokens.push(token);
        }
    }

    let mut parser: Parser = Parser::new(tokens);

    parser.parse();

    println!("{:#?}", parser.ast);
}

// fn interpreter() {
// println!("Jay version 0.0.0 (c) 2022");

//     loop {
//         print!(">>> ");

//         std::io::stdout().flush().expect("");

//         let mut input: String = String::new();

//         std::io::stdin().read_line(&mut input).expect("");

//         run(&input)
//     }
// }

fn compiler() {
    let args: Vec<String> = args().collect();

    println!("Jay version 0.0.0 (c) 2022");

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

    run(&file_content)
}

fn main() {
    match args().nth(1) {
        Some(ref arg) if arg == "-v" => version(),
        Some(ref arg) if arg == "-h" => help(),
        Some(ref arg) if arg == "--version" => version(),
        Some(ref arg) if arg == "--help" => help(),
        _ => compiler(),
    }
}
