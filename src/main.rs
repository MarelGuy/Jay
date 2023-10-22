use bumpalo::Bump;
use lexer::Lexer;
use parser::Parser;
use std::{
    env::args,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
    time::{Duration, Instant},
};

use crate::{lexer::token::Token, parser::ast::Nodes};

mod lexer;
mod parser;

struct RunProps<'a> {
    want_ast: bool,
    want_run: bool,
    want_llvm: bool,
    want_asm: bool,
    want_compile: bool,

    file_content: &'a str,
    file_name: &'a str,
}

impl<'a> RunProps<'a> {
    fn new() -> Self {
        Self {
            want_ast: false,
            want_run: false,
            want_llvm: false,
            want_asm: false,
            want_compile: true,
            file_content: "",
            file_name: "",
        }
    }
}

fn help() {
    println!("-h, --help    : show this help message");
    println!("-v, --version : show version");
    println!("-r, --run     : runs the program after compiling");
    println!(" --- DEBUG OPTIONS --- ");
    println!("    --ast         : show the jast tree file");
    println!("    --llvm-output : show the generated LLVM-IR file");
    println!("    --asm-output  : show the generated ASM file");
    println!("    --no-compiler : don't run the compiler");
}

fn check_props(props: RunProps, ast: Vec<Nodes<'_>>) {
    if props.want_ast {
        let ast_a: String = ast
            .into_iter()
            .map(|x| -> String { x.to_string() })
            .collect();

        File::create("./ast.jast")
            .unwrap()
            .write_all(ast_a.as_bytes())
            .unwrap()
    }
}

fn lex_and_parse(props: RunProps) {
    let now: Instant = Instant::now();

    let lexer: Lexer = Lexer::new(props.file_content);

    let tokens: Vec<Token<'_>> = lexer.into();

    let mut parser: Parser = Parser::new(tokens);

    let arena: Bump = Bump::new();

    let ast: Vec<Nodes<'_>> = parser.parse(&arena);

    let elapsed: Duration = now.elapsed();
    println!("completed in: {:.2?}", elapsed);

    check_props(props, ast);
}

fn main() {
    let mut props: RunProps = RunProps::new();

    let mut args: Vec<String> = args().collect::<Vec<String>>();

    args.remove(0);

    if args.is_empty() {
        println!("Error: No file specified");
        return;
    }

    let binding: String = args[0].clone();
    let file_path: &Path = Path::new(binding.as_str());
    props.file_name = file_path.file_name().unwrap().to_str().unwrap();

    if !file_path.exists() {
        println!("Error: file does not exist");
        return;
    }

    let binding: String = read_to_string(file_path).expect("Error: failed to read file");
    props.file_content = binding.as_str();

    args.remove(0);

    args.into_iter().for_each(|arg| match arg.as_str() {
        "-r" | "--run" => {
            props.want_run = true;
        }
        "--ast" => {
            props.want_ast = true;
        }
        "--llvm-output" => {
            props.want_llvm = true;
        }
        "--asm-output" => {
            props.want_asm = true;
        }
        "--no-compiler" => {
            props.want_compile = false;
        }
        "v" | "--version" => {
            println!("Jay v0.0.0 (2022-016-03)")
        }
        "-h" | "--help" => help(),
        _ => todo!("{}", arg),
    });

    lex_and_parse(props)
}
