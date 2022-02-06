use std::env::args;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 || args.len() > 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file_path: &Path = Path::new(&args[1]);

    if !file_path.exists() {
        println!("File not found: {}", args[1]);
        return;
    }

    if file_path.extension().and_then(OsStr::to_str) != Some("jay") {
        println!("File extension must be .jay: {}", args[1]);
        return;
    }

    let mut lexer: lexer::Lexer = lexer::Lexer::new(fs::read_to_string(&args[1]).unwrap());

    for elem in lexer.make_tokens() {
        print!("{:}", elem.get_token());
    }
}
