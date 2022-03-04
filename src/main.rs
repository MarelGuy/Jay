use std::{env::args, fs::read_to_string, io::Write, path::Path};

use chrono::{Datelike, Utc};

mod lexer;
mod token;

fn help() {
    println!("-h, --help: show this help message");
    println!("-v, --version: show version");
    println!("-c, --compiler: use the compiler");
}

fn version() {
    println!("Jay v0.0.0 (date)");
}

fn interpreter() {
    println!("Jay version 0.0.0 (c) {}", Utc::now().date().year());

    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Could not flush stdout");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut lexer = lexer::Lexer::new(input);

        println!("{:?}", lexer.next_token().unwrap());
    }
}

fn compiler() {
    let args: Vec<String> = args().collect();

    println!("Jay version 0.0.0 (c) {}", Utc::now().date().year());

    if args.len() < 2 {
        println!("Error: No file specified");
        return;
    }

    let file_path = Path::new(&args[2]);

    if !file_path.exists() {
        println!("Error: file does not exist");
        return;
    }

    let file_content: String = read_to_string(file_path).expect("Error: failed to read file");

    let mut lexer = lexer::Lexer::new(file_content);

    let mut tokens: Vec<token::Token> = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(token) => tokens.push(token),
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        }

        if tokens.last().unwrap().get_token_type() == "UNKNOWN" {
            tokens.pop();
            break;
        }
    }

    println!("{:#?}", tokens);
}

fn main() {
    match args().nth(1) {
        Some(ref arg) if arg == "--compiler" => compiler(),
        Some(ref arg) if arg == "-c" => compiler(),
        Some(ref arg) if arg == "--version" => version(),
        Some(ref arg) if arg == "-v" => version(),
        Some(ref arg) if arg == "--help" => help(),
        Some(ref arg) if arg == "-h" => help(),
        _ => {
            interpreter();
        }
    }
}
