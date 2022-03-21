/*
Jay main
Copyright (C) 2022  Loris Cuntreri
*/
use {
    chrono::{Datelike, Utc},
    std::{env::args, fs::read_to_string, io::Write, path::Path},
};

mod lexer;

fn help() {
    println!("-h, --help: show this help message");
    println!("-v, --version: show version");
    println!("-c, --compiler: use the compiler");
}

fn version() {
    println!("Jay v0.0.0 (2022-016-03)");
}

fn lex_code(input: String) -> Vec<lexer::token::Token<'static>> {
    let lexer = lexer::lexer::Lexer::new(&input);

    let mut vet_token: Vec<lexer::token::Token> = Vec::new();

    for token in lexer {
        if token.token_type != lexer::token::TokenType::Error {
            vet_token.push(token);
        }
    }

    vet_token
}

fn interpreter() {
    println!("Jay version 0.0.0 (c) {}", Utc::now().date().year());

    loop {
        print!(">>> ");

        // we flush the console to have a correct input
        std::io::stdout().flush().expect("");

        let mut input: String = String::new();

        // we read in input and use as a buffer the variable input
        std::io::stdin().read_line(&mut input).expect("");

        lex_code(input);
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

    let lexed_content = lex_code(file_content);
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
