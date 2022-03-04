use std::{fmt::Error, process::exit};

use crate::token::Token;

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer: Lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            char: ' ',
        };
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = '\0';
        } else {
            self.char = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let mut result: String = String::new();
        while self.char.is_alphabetic() || self.char == '_' {
            result.push(self.char);
            self.read_char();
        }
        result
    }

    fn read_number(&mut self) -> String {
        let mut result: String = String::new();
        while self.char.is_numeric() {
            result.push(self.char);
            self.read_char();
        }
        result
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn skip_whitespace(&mut self) {
        self.read_char();
        while self.char == ' ' || self.char == '\t' || self.char == '\n' || self.char == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) /* -> Result<Token, Error> */
    {
        self.skip_whitespace();
        let token: Token = match self.char {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new("IS_EQUAL".to_string(), "==".to_string())
                } else {
                    Token::new("EQUALS".to_string(), "=".to_string())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new("IS_DIFFERENT".to_string(), "!=".to_string())
                } else {
                    Token::new("NOT".to_string(), "!".to_string())
                }
            }
            '+' => Token::new("PLUS".to_string(), "+".to_string()),
            '-' => Token::new("MINUS".to_string(), "-".to_string()),
            '*' => Token::new("TIMES".to_string(), "*".to_string()),
            '/' => Token::new("DIVIDED".to_string(), "/".to_string()),
            '<' => Token::new("MINOR".to_string(), "<".to_string()),
            '>' => Token::new("GREATER".to_string(), ">".to_string()),
            ';' => Token::new("SEMICOLON".to_string(), ";".to_string()),
            '(' => Token::new("RPAREN".to_string(), "(".to_string()),
            ')' => Token::new("LPAREN".to_string(), ")".to_string()),
            '{' => Token::new("RCURLY".to_string(), "{".to_string()),
            '}' => Token::new("LCURLY".to_string(), "}".to_string()),
            ',' => Token::new("COMMA".to_string(), ",".to_string()),
            '.' => Token::new("DOT".to_string(), ".".to_string()),
            '[' => Token::new("RSQUARE".to_string(), "[".to_string()),
            ']' => Token::new("LSQUARE".to_string(), "]".to_string()),
            '"' => {
                let mut result: String = String::new();
                self.read_char();
                while self.char != '"' {
                    result.push(self.char);
                    self.read_char();
                }
                self.read_char();
                Token::new("STRING".to_string(), result)
            }
            '0'..='9' => Token::new("NUM".to_string(), self.read_number()),
            'a'..='z' | 'A'..='Z' | '_' => {
                Token::new("IDENTIFIER".to_string(), self.read_identifier())
            }
            _ => {
                self.read_char();
                Token::new("UNKNOWN".to_string(), "".to_string())
            }
        };

        println!("{:#?}", token);

        if token.get_token_type() == "UNKNOWN" {
            exit(1);
        }
        // Ok(token) // TODO: Return error if token is unknown.
    }
}
