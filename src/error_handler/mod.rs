use colored::{ColoredString, Colorize};
use std::process::exit;

use crate::lexer::token::Token;

pub struct Error<'a> {
    token: Token<'a>,
    e_str: ColoredString,
    line_string: String,
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

    pub fn print(&self) {
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
            " ".repeat(self.token.column),
            error_counter.yellow()
        );
        println!("{}", "-".blue());
    }

    pub fn throw_var_not_defined(&self, var_name: &str) {
        println!(
            "{}: cannot find variable \"{}\" in this scope",
            self.e_str, var_name
        );
        self.print();
        exit(0)
    }

    pub fn throw_wrong_assign_type(&self, var_name: &str, val_type: String, var_type: String) {
        println!(
            "{}: cannot assign value of type \"{}\" to variable \"{}\": {}",
            self.e_str, val_type, var_name, var_type
        );
        self.print();
        exit(0)
    }

    pub fn throw_type_name_already_used(&self, name: String) {
        println!(
            "{}, type name: \"{}\" already used in this scope",
            self.e_str, name
        );
        self.print();
        exit(0)
    }

    pub fn throw_ty_not_found(&self) {
        println!(
            "{}: type \"{}\" not found in this scope",
            self.e_str, self.token.slice
        );
        self.print();
        exit(0)
    }

    pub fn throw_unkown_token(&self) {
        println!("{}: unknown token: \"{}\"", self.e_str, self.token.slice);
        self.print();
        exit(0)
    }

    pub fn throw_cant_start_var_num(&self) {
        println!("{}: cannot start variable name with number", self.e_str);
        self.print();
        exit(0)
    }
}
