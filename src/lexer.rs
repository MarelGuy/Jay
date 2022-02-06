use crate::token::Token;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub struct Lexer {
    text: String,
    position: usize,
    current_str: String,
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Lexer {{ text: {}, position: {}, current_str: {} }}",
            self.text, self.position, self.current_str
        )
    }
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        Lexer {
            text: text,
            position: 0,
            current_str: "\0".to_string(),
        }
    }

    pub fn advance(&mut self) {
        self.position += 1;

        if self.position > self.text.len() {
            self.current_str = "\0".to_string();
        } else {
            self.current_str = String::from(
                self.text
                    .split_whitespace()
                    .nth(self.position as usize)
                    .unwrap(),
            );

            println!("{}", self.current_str);
        }
    }

    pub fn make_number(&self, current_string: &str) -> Token {
        let mut dot_count = 0;

        for elem in current_string.chars() {
            if elem == '.' {
                dot_count += 1;
            } else if elem.is_numeric() {
                continue;
            } else {
                break;
            }
        }

        if dot_count > 1 {
            return Token::new("TT_FLOAT".to_owned(), current_string.to_string());
        } else {
            return Token::new("TT_INT".to_owned(), current_string.to_string());
        }
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.current_str != "\0" {
            if self.current_str == "+" {
                tokens.push(Token::new("TT_PLUS".to_owned(), "+".to_owned()));
            } else if self.current_str == "-" {
                tokens.push(Token::new("TT_MINUS".to_owned(), "-".to_owned()));
            } else if self.current_str == "/" {
                tokens.push(Token::new("TT_DIV".to_owned(), "/".to_owned()));
            } else if self.current_str == "*" {
                tokens.push(Token::new("TT_MUL".to_owned(), "*".to_owned()));
            } else if self.current_str == "(" {
                tokens.push(Token::new("TT_LPAREN".to_owned(), "(".to_owned()));
            } else if self.current_str == ")" {
                tokens.push(Token::new("TT_RPAREN".to_owned(), ")".to_owned()));
            } else {
                tokens.push(Token::new("\0".to_owned(), "\0".to_owned()));
            }

            self.advance();
        }

        return tokens;
    }
}
