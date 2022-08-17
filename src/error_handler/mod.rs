use colored::Colorize;
use std::process::exit;

pub struct Error<'a> {
    line: usize,
    line_string: String,
    slice: &'a str,
    column: usize,
    file_name: String,
}

impl<'a> Error<'a> {
    pub fn new(
        line: usize,
        line_string: String,
        slice: &'a str,
        column: usize,
        file_name: String,
    ) -> Self {
        Self {
            line,
            line_string,
            slice,
            column,
            file_name,
        }
    }

    pub fn print(&self) {
        let error_counter: String = "^".repeat(self.slice.len());
        println!("{} file: {}", "-".blue(), self.file_name);
        println!("{}", "|".blue());
        println!("{} {}.   {}", "|".blue(), self.line, self.line_string);
        println!(
            "{}      {}{}",
            "|".blue(),
            " ".repeat(self.column),
            error_counter.yellow()
        );
        println!("{}", "-".blue());
    }

    pub fn throw_var_not_defined(&self, var_name: &str) {
        println!(
            "{}: cannot find variable {} in this scope",
            "error".red(),
            var_name
        );
        self.print();
        exit(0)
    }

    pub fn throw_wrong_assign_type(&self, var_name: &str, val_type: String, var_type: String) {
        println!(
            "{}: cannot assign value of type {} to variable {}: {}",
            "error".red(),
            val_type,
            var_name,
            var_type
        );
        self.print();
        exit(0)
    }
}
