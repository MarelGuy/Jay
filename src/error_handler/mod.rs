use colored::{ColoredString, Colorize};
use std::process::exit;

use crate::lexer::token::Token;

pub struct Error<'a> {
    pub token: Token<'a>,
    e_str: ColoredString,
    pub line_string: String,
    file_name: String,
}

impl<'a> Error<'a> {
    pub fn new(token: Token<'a>, line_string: String, file_name: String) -> Self {
        Self {
            token,
            e_str: "error".red(),
            line_string,
            file_name,
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

    // Var error
    pub fn throw_var_not_defined(&self) {
        println!(
            "{}: cannot find variable \"{}\" in this scope",
            self.e_str, self.token.slice
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_wrong_assign_type(&self, var_name: &str, val_type: String, var_type: String) {
        println!(
            "{}: cannot assign value of type \"{}\" to variable \"{}\" which is of type \"{}\"",
            self.e_str, val_type, var_name, var_type
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_cant_start_var_num(&self) {
        println!("{}: cannot start variable name with number", self.e_str);
        self.print(0);
        exit(0)
    }

    pub fn throw_cant_use_same_var_name(&self) {
        println!("{}: variable name already used", self.e_str);
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
        println!("{}: can't access index {} in {}", self.e_str, idx, arr_name,);
        self.print(0);
        exit(0)
    }

    pub fn throw_cant_use_val_in_arr_call(&self, val: String) {
        println!(
            "{}: can't use val of type \"{}\" in array call",
            self.e_str, val,
        );
        self.print(0);
        exit(0)
    }

    // Type errors
    pub fn throw_type_name_already_used(&self, name: String) {
        println!(
            "{}, type name: \"{}\" already used in this scope",
            self.e_str, name
        );
        self.print(0);
        exit(0)
    }

    pub fn throw_ty_not_found(&self) {
        println!(
            "{}: type \"{}\" not found in this scope",
            self.e_str, self.token.slice
        );
        self.print(0);
        exit(0)
    }

    // General errors
    pub fn throw_unkown_token(&self) {
        println!("{}: unknown token: \"{}\"", self.e_str, self.token.slice);
        self.print(0);
        exit(0)
    }

    pub fn throw_unkown_token_in_math_expr(&self) {
        println!(
            "{}: unknown token in math expression: \"{}\"",
            self.e_str, self.token.slice
        );
        self.print(0);
        exit(0)
    }
}
