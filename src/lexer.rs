use {crate::s_error::SError, crate::token::Token};

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let lexer: Lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            char: ' ',
            //    ^^^ char is initizlied with a space because we use the skip_whitespace()
            //        at the start.
        };
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = '\0';
        } else {
            // self.char gets assigned to the nth number of the array that is based
            // on read_position
            self.char = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
        // read_position gets incremented by one each time the read_char() function gets called
    }

    // TODO: Add number support for identifiers
    fn read_identifier(&mut self) -> String {
        let mut result: String = String::new();

        // We first push the first char manually
        result.push(self.char);

        while self.peek_char().is_alphabetic() || self.peek_char() == '_' {
            self.skip_whitespace(); // then read another char
            result.push(self.char); // and push it in the final string
        }
        result
    }

    fn read_number(&mut self) -> Token {
        let mut result: String = String::new();

        // same thing happens here
        result.push(self.char);

        while self.peek_char().is_numeric() {
            self.skip_whitespace();
            result.push(self.char);
        }

        // if we find a dot, we know that we have a float
        if self.peek_char() == '.' {
            self.skip_whitespace();
            result.push(self.char);

            while self.peek_char().is_numeric() {
                self.skip_whitespace();
                result.push(self.char);
            }

            return Token::new("FLOAT".into(), result);
        }

        Token::new("INT".into(), result)
    }

    fn peek_char(&self) -> char {
        // this function works exactly like read_char(), only difference is that it
        // doesn't incrememnt read_position

        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn peek_back_char(&self) -> char {
        // this function is the opposite of peek_char()

        if self.position == 0 {
            '\0'
        } else {
            self.input.chars().nth(self.position - 1).unwrap()
        }
    }

    fn skip_whitespace(&mut self) {
        // we read a char
        self.read_char();
        while self.char == ' ' || self.char == '\t' || self.char == '\n' || self.char == '\r' {
            // and as long as there is a whitespace in the char, we go on on reading
            // until we get something else
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, SError> {
        self.skip_whitespace();

        // we declare that the token is defined on a switch-like block that gives back
        // a token based on the character it read
        let token: Token = match self.char {
            '=' => {
                // we check if the ! serves as == or =
                if self.peek_char() == '=' {
                    self.skip_whitespace();
                    Token::new("IS_EQUAL_TO".into(), "==".into())
                } else {
                    Token::new("EQUALS".into(), "=".into())
                }
            }
            '!' => {
                // we check if the ! serves as != or !
                if self.peek_char() == '=' {
                    self.skip_whitespace();
                    Token::new("IS_DIFFERENT_FROM".into(), "!=".into())
                } else {
                    Token::new("NOT".into(), "!".into())
                }
            }
            '+' => Token::new("PLUS".into(), '+'.into()),
            '-' => {
                // we check if the - is a minus sign or a negative number
                if self.peek_char().is_numeric() {
                    if self.peek_back_char().is_numeric() {
                        Token::new("MINUS".into(), "-".into())
                    } else {
                        self.read_number()
                    }
                } else {
                    Token::new("MINUS".into(), "-".into())
                }
            }
            '*' => Token::new("TIMES".into(), "*".into()),
            '/' => Token::new("DIVIDED".into(), "/".into()),
            '<' => Token::new("MINOR".into(), "<".into()),
            '>' => Token::new("GREATER".into(), ">".into()),
            ';' => Token::new("SEMICOLON".into(), ";".into()),
            ',' => Token::new("COMMA".into(), ",".into()),
            '.' => Token::new("DOT".into(), ".".into()),
            '(' => Token::new("RPAREN".into(), "(".into()),
            ')' => Token::new("LPAREN".into(), ")".into()),
            '{' => Token::new("RCURLY".into(), "{".into()),
            '}' => Token::new("LCURLY".into(), "}".into()),
            '[' => Token::new("RSQUARE".into(), "[".into()),
            ']' => Token::new("LSQUARE".into(), "]".into()),
            '"' => {
                let mut result: String = String::new();
                let mut error: bool = false;

                self.read_char();

                // we read the string until we find a " if we don't we throw an error
                while self.char != '"' {
                    result.push(self.char);

                    self.read_char();

                    if self.char == '\0' || self.char == ';' {
                        error = true;

                        SError::new(
                            "Unterminated string".into(),
                            "The string was unterminated.".into(),
                        )
                        .throw_error();
                        break;
                    }
                }

                if error != true {
                    Token::new("STRING".into(), result)
                } else {
                    Token::new("ERROR".into(), "".into())
                }
            }
            '|' => {
                // we check if the | serves as | or ||
                if self.peek_char() == '|' {
                    self.skip_whitespace();
                    Token::new("OR".into(), "||".into())
                } else {
                    Token::new("PIPE".into(), "|".into())
                }
            }
            '&' => {
                // we check if the & serves as && or &
                if self.peek_char() == '&' {
                    self.skip_whitespace();
                    Token::new("AND".into(), "&&".into())
                } else {
                    Token::new("AMPERSAND".into(), "&".into())
                }
            }
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' | '_' => Token::new("IDENTIFIER".into(), self.read_identifier()),
            '\0' => Token::new("EOF".into(), "EOF".into()),
            _ => {
                SError::new("Unknown token".into(), "Token not implemented.".into()).throw_error();
                Token::new("ERROR".into(), "".into())
            }
        };

        // we give back an OK result so that we are sure that there are no errors.
        Ok(token)
    }

    pub fn input(&self) -> &str {
        self.input.as_ref()
    }
}
