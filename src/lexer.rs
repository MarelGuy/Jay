use crate::token::Token;

pub struct Lexer {
    text: String,
    position: isize,
    current_char: char,
}

impl Lexer {
    pub fn new(self, text: String) -> Lexer {
        self.advance();

        Lexer {
            text: text,
            position: -1,
            current_char: '\0',
        }
    }

    pub fn advance(mut self) {
        self.position += 1;

        if self.position > self.text.len() as isize {
            self.current_char = '\0';
        } else {
            self.current_char = self
                .text
                .chars()
                .nth(self.position.try_into().unwrap())
                .unwrap();
        }
    }

    pub fn make_number(self) -> Token {
        let num_str = "";
        let dot_count = 0;

        while assert_eq!(self.current_char, '\0') && self.current_char.is_numeric() + '.' {
            if self.current_char == '.' {
                if dot_count == 1 {
                    break;
                } else {
                    dot_count += 1;
                    num_str = &(num_str.to_owned() + ".");
                }
            } else {
                num_str = &self.current_char.to_string();
            }

            num_str = &self.current_char.to_string();
        }

        let finished_token = Token::new("TT_FLOAT".to_owned(), num_str.to_string());

        return finished_token;
    }

    pub fn make_tokens(self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while assert_eq!(self.current_char, '\0') {
            if self.current_char.is_whitespace() {
                self.advance();
            } else if self.current_char.is_numeric() {
                tokens.push(self.make_number());
            } else if assert_eq!(self.current_char, '+') {
                tokens.push(Token::new("TT_PLUS".to_owned(), "+".to_owned()));
                self.advance();
            } else if assert_eq!(self.current_char, '-') {
                tokens.push(Token::new("TT_MINUS".to_owned(), "-".to_owned()));
                self.advance();
            } else if assert_eq!(self.current_char, '/') {
                tokens.push(Token::new("TT_DIV".to_owned(), "/".to_owned()));
                self.advance();
            } else if assert_eq!(self.current_char, '*') {
                tokens.push(Token::new("TT_MUL".to_owned(), "*".to_owned()));
                self.advance();
            } else if assert_eq!(self.current_char, '(') {
                tokens.push(Token::new("TT_LPAREN".to_owned(), "(".to_owned()));
                self.advance();
            } else if assert_eq!(self.current_char, ')') {
                tokens.push(Token::new("TT_RPAREN".to_owned(), ")".to_owned()));
                self.advance();
            }
        }

        return tokens;
    }
}
