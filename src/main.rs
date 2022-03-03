use std::io::Write;

use chrono::{Datelike, Utc};

mod lexer;
mod token;

// When the interpreter is run, everything before the first whitespace.
fn main() {
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
