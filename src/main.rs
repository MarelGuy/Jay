use {
    crate::lexer::Lexer,
    chrono::{Datelike, Utc},
    std::{env::args, fs::read_to_string, io::Write, path::Path},
};

mod lexer;
mod s_error;
mod token;

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

        // we flush the console to have a correct input
        std::io::stdout().flush().expect("Could not flush stdout");

        let mut input: String = String::new();

        // we read in input and use as a buffer the variable input
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut lexer: Lexer = lexer::Lexer::new(input);

        let mut tokens: Vec<token::Token> = Vec::new();

        let mut can_output: bool = true;

        // we loop through the file to be sure that we
        // don't go over the input
        for _ in 0..lexer.input().len() {
            // match block to verify the OK or Err
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(_) => {}
            }

            // if the token that we get is of type EOF, then we delete it
            // and stop the loop
            if tokens.last().unwrap().get_token_type() == "EOF" {
                tokens.pop();
                break;
            }

            // if we find an error, we go on but don't give any output
            if tokens.last().unwrap().get_token_type() == "ERROR" {
                can_output = false;
            }
        }
        if can_output == true {
            println!("{:#?}", tokens);
        }
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

    let mut lexer: Lexer = lexer::Lexer::new(file_content);

    let mut tokens: Vec<token::Token> = Vec::new();

    let mut can_output: bool = true;

    loop {
        // this block is the same as the one in the interpreter
        match lexer.next_token() {
            Ok(token) => tokens.push(token),
            Err(_) => {}
        }

        if tokens.last().unwrap().get_token_type() == "EOF" {
            tokens.pop();
            break;
        }

        if tokens.last().unwrap().get_token_type() == "ERROR" {
            can_output = false;
            break;
        }
    }

    if can_output == true {
        println!("{:#?}", tokens);
    }
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
