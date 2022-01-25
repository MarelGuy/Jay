use crate::token::Token;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub struct Lexer<'a> {
    text: String,
    position: isize,
    current_str: &'a str,
}

impl Debug for Lexer<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Lexer {{ text: {}, position: {}, current_str: {} }}",
            self.text, self.position, self.current_str
        )
    }
}

impl Lexer<'_> {
    pub fn new(text: String) -> Lexer<'static> {
        Lexer {
            text: text,
            position: 0,
            current_str: "\0",
        }
    }

    pub fn advance(&mut self) {
        self.position += 1;

        if self.position > self.text.len() as isize {
            self.current_str = "\0";
        } else {
            self.current_str = self
                .text
                .split_whitespace()
                .nth(self.position.try_into().unwrap())
                .unwrap();
        }
    }

    pub fn make_number(&self, current_string: &str) -> Token {
        let mut dot_count = 0;

        for elem in current_string.chars() {
            if elem == '.' {
                dot_count += 1;
            }
        }

        if dot_count > 1 {
            return Token::new("TT_FLOAT".to_owned(), current_string.to_string());
        } else {
            return Token::new("TT_INT".to_owned(), current_string.to_string());
        }
    }
    pub fn make_tokens(&mut self) -> Vec<Token> {
        self.advance();
        let mut tokens: Vec<Token> = Vec::new();

        while self.current_str != "\0" {
            print!("{}", self.current_str);

            for elem in self.current_str.chars() {
                if elem.is_numeric() {
                    tokens.push(Token::new("TT_INT".to_owned(), elem.to_string()));
                }
            }

            if self.current_str == "+" {
                tokens.push(Token::new("TT_PLUS".to_owned(), "+".to_owned()));
                self.advance();
            } else if self.current_str == "-" {
                tokens.push(Token::new("TT_MINUS".to_owned(), "-".to_owned()));
                self.advance();
            } else if self.current_str == "/" {
                tokens.push(Token::new("TT_DIV".to_owned(), "/".to_owned()));
                self.advance();
            } else if self.current_str == "*" {
                tokens.push(Token::new("TT_MUL".to_owned(), "*".to_owned()));
                self.advance();
            } else if self.current_str == "(" {
                tokens.push(Token::new("TT_LPAREN".to_owned(), "(".to_owned()));
                self.advance();
            } else if self.current_str == ")" {
                tokens.push(Token::new("TT_RPAREN".to_owned(), ")".to_owned()));
                self.advance();
            } else {
                tokens.push(Token::new("\0".to_owned(), "\0".to_owned()));
                self.advance();
            }
        }

        return tokens;
    }
}
