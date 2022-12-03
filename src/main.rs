// use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use std::{
    env::args,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use crate::lexer::token::{Token, TokenType};

mod error_handler;
mod lexer;
mod parser;
// mod compiler;

fn help() {
    println!("-h, --help: show this help message");
    println!("-v, --version: show version");
}

fn version() {
    println!("Jay v0.0.0 (2022-016-03)");
}

fn run(input: &str, file_name: &str) {
    let lexer: Lexer = Lexer::new(input);

    let mut tokens: Vec<Token> = Vec::new();

    for token in lexer {
        if token.token_type != TokenType::Space
            && token.token_type != TokenType::LineFeed
            && token.token_type != TokenType::CarriageReturn
            && token.token_type != TokenType::Comment
        {
            tokens.push(token);
        }
    }

    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let mut parser: Parser = Parser::new(tokens, file_name.into(), lines);
    parser.parse();

    let ast: String = parser
        .ast
        .into_iter()
        .map(|x| -> String { x.to_string() })
        .collect();

    match args().nth(2) {
        Some(ref arg) if arg == "--ast" => {
            File::create("./ast.jast")
                .unwrap()
                .write(ast.as_bytes())
                .unwrap();
        }
        _ => {}
    }

    // let compiler: Compiler = Compiler::new(parser.ast);

    // compiler.compile();
}

fn interpreter() {
    println!("Jay version 0.0.0 (c) 2022");
    loop {
        print!(">>> ");

        std::io::stdout().flush().expect("");

        let mut input: String = String::new();

        std::io::stdin().read_line(&mut input).expect("");

        run(&input, "Interpreter");
    }
}

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

    run(
        &file_content,
        file_path.file_name().unwrap().to_str().unwrap(),
    );
}

fn main() {
    match args().nth(1) {
        Some(ref arg) if arg == "-v" => version(),
        Some(ref arg) if arg == "-h" => help(),
        Some(ref arg) if arg == "--version" => version(),
        Some(ref arg) if arg == "--help" => help(),
        Some(ref arg) if arg == "-i" => interpreter(),
        _ => compiler(),
    }
}
