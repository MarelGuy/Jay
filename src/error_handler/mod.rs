use colored::{ColoredString, Colorize};

use std::process::exit;

use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Error<'a> {
    pub token: Token<'a>,
    e_str: ColoredString,
    pub line_string: String,
    file_name: String,
    using_local_scope: bool,
}

impl<'a> Error<'a> {
    pub fn new(token: Token<'a>, line_string: String, file_name: String) -> Self {
        Self {
            token,
            e_str: "error".red().bold(),
            line_string,
            file_name,
            using_local_scope: false,
        }
    }

    pub fn print(&self, offset: usize) {
        let error_counter: String = "^".repeat(self.token.slice.len());
        println!("{} file: {}", "-".blue(), self.file_name);
        println!("{}", "|".blue());
        println!(
            "{} {}.   {}",
            "|".blue(),
            self.token.line + 1,
            self.line_string
        );
        println!(
            "{}      {}{}",
            "|".blue(),
            " ".repeat(self.token.column + offset),
            error_counter.yellow()
        );
        println!("{}", "-".blue());
    }

    // General errors
    pub fn throw_name_already_used(&self, e_type: u8) {
        println!(
            "{}: {} name \"{}\" already used",
            self.e_str,
            match e_type {
                0 => "variable",
                1 => "function",
                2 => "type",
                _ => todo!(),
            }
            .magenta()
            .italic(),
            self.token.slice.magenta().italic()
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_name_not_defined(&self, e_type: u8) {
        println!(
            "{}: cannot find {} name: \"{}\" in current {} scope",
            self.e_str,
            match e_type {
                0 => "variable",
                1 => "function",
                2 => "type",
                _ => "",
            }
            .magenta()
            .italic(),
            self.token.slice.magenta().italic(),
            if self.using_local_scope {
                "local"
            } else {
                "global"
            }
            .magenta()
            .italic()
        );
        self.print(0);
        exit(0)
    }

    // Var errors

    pub fn throw_wrong_assign_type(&self, var_name: &str, val_type: String, var_type: String) {
        println!(
            "{}: cannot assign value of type \"{}\" to variable \"{}\" which is of type \"{}\"",
            self.e_str,
            val_type.magenta().italic(),
            var_name.magenta().italic(),
            var_type.magenta().italic()
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_cant_start_var_num(&self) {
        println!(
            "{}: cannot start variable name \"{}\" with number",
            self.e_str,
            self.token.slice.magenta().italic()
        );
        self.print(0);
        exit(0)
    }

    // Array errors

    pub fn throw_array_out_of_bounds(&self, arr_len: &isize) {
        println!("{}: expected an array of size {}", self.e_str, arr_len,);
        self.print(0);
        exit(0)
    }

    pub fn throw_cant_use_num_array(&self, arr_name: &str, idx: isize) {
        println!(
            "{}: can't access index {} in {}",
            self.e_str,
            idx,
            arr_name.magenta().italic(),
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_cant_use_val_in_arr_call(&self, val: String) {
        println!(
            "{}: can't use val of type \"{}\" in array call",
            self.e_str,
            val.magenta().italic(),
        );
        self.print(0);
        exit(0)
    }

    // Functions errors

    pub fn throw_arg_alreay_used(&self, arg_name: String) {
        println!(
            "{}: argument name: \"{}\" already used",
            self.e_str,
            arg_name.magenta().italic()
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_used_return_when_no_return(&self, name: String) {
        println!(
            "{}: function \"{}\" doesn't have any return type",
            self.e_str,
            name.magenta().italic()
        );
        self.print(0);
        exit(0)
    }

    // General errors
    pub fn throw_unkown_token(&self) {
        println!(
            "{}: unknown token: \"{}\"",
            self.e_str,
            self.token.slice.magenta().italic()
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_unkown_token_in_math_expr(&self) {
        println!(
            "{}: unknown token in math expression: \"{}\"",
            self.e_str,
            self.token.slice.magenta().italic()
        );
        self.print(0);
        exit(0)
    }
}
