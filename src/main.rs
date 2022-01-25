use std::env::args;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

mod lexer;
mod token;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

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

    if get_extension_from_filename(&args[1]) != Some("jay") {
        println!("File extension must be .jay");
        return;
    }

    let contents: String = fs::read_to_string(&args[1]).unwrap();
    let mut lexer: lexer::Lexer = lexer::Lexer::new(contents);

    println!("{:?}", lexer.make_tokens());
}
