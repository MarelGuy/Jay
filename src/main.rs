use compiler::Codegen;
use inkwell::{
    basic_block::BasicBlock,
    context::Context,
    module::Module,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple},
    types::{FunctionType, IntType},
    values::FunctionValue,
    OptimizationLevel,
};
use lexer::Lexer;
use parser::Parser;
use std::{
    env::args,
    fs::{read_to_string, remove_file, File},
    io::Write,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};

use crate::lexer::token::Token;

mod compiler;
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
    println!("    --ast         : DEBUG ONLY - show the jast tree file");
    println!("    --llvm-output : DEBUG ONLY - show the generated LLVM-IR file");
    println!("    --asm-output  : DEBUG ONLY - show the generated ASM file");
    println!("    --no-compiler : DEBUG ONLY - don't run the compiler");
}

fn version() {
    println!("Jay v0.0.0 (2022-016-03)");
}

fn init_target(codegen: &Codegen) -> TargetMachine {
    Target::initialize_x86(&InitializationConfig::default());

    codegen
        .module
        .set_triple(&TargetTriple::create("x86_64-pc-windows-msvc19.37.32824"));

    let target = Target::from_name("x86-64").unwrap();

    target
        .create_target_machine(
            &codegen.module.get_triple(),
            "x86-64",
            "+avx2",
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap()
}

fn check_props(props: RunProps, parser: Parser) {
    if props.want_ast {
        let ast: String = parser
            .ast
            .clone()
            .into_iter()
            .map(|x| -> String { x.to_string() })
            .collect();

        File::create("./ast.jast")
            .unwrap()
            .write_all(ast.as_bytes())
            .unwrap()
    }

    if !props.want_compile {
        return;
    }

    let context: Context = Context::create();
    let module: Module = context.create_module(&props.file_name[0..props.file_name.len() - 4]);

    let i32_t: IntType<'_> = context.i32_type();
    let fn_type: FunctionType<'_> = i32_t.fn_type(&[], false);

    let fn_main: FunctionValue<'_> = module.add_function("main", fn_type, None);
    let fn_main_basic_block: BasicBlock<'_> = context.append_basic_block(fn_main, "entry");

    let compiler: Codegen = Codegen::new(
        &context,
        module,
        context.create_builder(),
        // fn_main,
        fn_main_basic_block,
        parser.ast.clone(),
    );
    let target_machine = init_target(&compiler);

    compiler.compile();

    compiler.builder.position_at_end(compiler.main_block);
    compiler
        .builder
        .build_return(Some(&i32_t.const_int(0, false)))
        .unwrap();

    let mut did_compile: bool = false;

    target_machine
        .write_to_file(
            &compiler.module,
            inkwell::targets::FileType::Assembly,
            Path::new("./output.s").as_ref(),
        )
        .and_then(|_| {
            Ok({
                Command::new("clang")
                    .args(["output.s", "-o", "output.exe"])
                    .spawn()
                    .unwrap();
                did_compile = true;
            })
        })
        .unwrap();

    if props.want_run {
        todo!()
    }
    if !props.want_asm {
        while did_compile == false {}
        remove_file("./output.s").unwrap();
    }
    if props.want_llvm {
        compiler.module.print_to_file("./output.ll").unwrap();
    } else {
        remove_file("./output.ll").unwrap();
    }
}

fn run(props: RunProps) {
    let lexer: Lexer = Lexer::new(props.file_content);

    let tokens: Vec<Token> = lexer.into();

    let lines: Vec<String> = props
        .file_content
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut parser: Parser = Parser::new(tokens, props.file_name.into(), lines);
    parser.parse();

    check_props(props, parser);
}

fn compiler() {
    let mut args: Vec<String> = args().collect();

    println!("Jay version 0.0.0 (c) 2022");

    let mut props: RunProps = RunProps::new();

    args.remove(0);

    if args.is_empty() {
        println!("Error: No file specified");
        return;
    }

    let file_path: &Path = Path::new(&args[0]);
    props.file_name = file_path.file_name().unwrap().to_str().unwrap();

    let did_find_file: bool = true;

    if !file_path.exists() {
        println!("Error: file does not exist");
        return;
    }

    let binding: String = read_to_string(file_path).expect("Error: failed to read file");
    props.file_content = binding.as_str();

    args.clone().into_iter().for_each(|arg| match arg.as_str() {
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
        _ => {
            let file_path: &Path = Path::new(&arg);

            if file_path.exists() && !did_find_file {
                println!("Error: the input file needs to be the first argument");
            }
        }
    });

    run(props);
}

fn main() {
    let now: Instant = Instant::now();

    // TODO: Add commands
    {
        match args().nth(1) {
            Some(ref arg) if arg == "-v" || arg == "--version" => version(),
            Some(ref arg) if arg == "-h" || arg == "--help" => help(),
            // Some(ref arg) if arg == "-i" => interpreter(),
            _ => compiler(),
        }
    }

    let elapsed: Duration = now.elapsed();
    println!("completed in: {:.2?}", elapsed);
}

#[cfg(test)]
mod test {
    // TODO: Addition and subtraction tests
}
