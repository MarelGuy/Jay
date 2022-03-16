use {crate::token::Token, std::fmt::Error};

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
            // self.char gets assigned to the nth number of th earray that is based
            // on read_position
            self.char = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
        // read_position gets incremented by one each time the read_char() function gets called
    }

    fn read_identifier(&mut self) -> String {
        let mut result: String = String::new();

        // We first push the first char manually
        result.push(self.char);

        while self.peek_char().is_alphabetic() || self.peek_char() == '_' {
            self.read_char(); // then read another char
            result.push(self.char); // and push it in the final string
        }
        result
    }

    fn read_number(&mut self) -> String {
        let mut result: String = String::new();

        // same thing happens here
        result.push(self.char);

        while self.peek_char().is_numeric() {
            self.read_char();
            result.push(self.char);
        }

        result
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

    fn skip_whitespace(&mut self) {
        // we read a char
        self.read_char();
        while self.char == ' ' || self.char == '\t' || self.char == '\n' || self.char == '\r' {
            // and as long as there is a whitespace in the char, we go on on reading
            // until we get something else
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        // we declare that the token is defined on a switch-like block that gives back
        // a token based on the character it read
        let token: Token = match self.char {
            '=' => {
                // we check if the ! serves as == or =
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new("IS_EQUAL".to_string(), "==".to_string())
                } else {
                    Token::new("EQUALS".to_string(), "=".to_string())
                }
            }
            '!' => {
                // we check if the ! serves as != or !
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new("IS_DIFFERENT".to_string(), "!=".to_string())
                } else {
                    Token::new("NOT".to_string(), "!".to_string())
                }
            }
            '+' => Token::new("PLUS".to_string(), '+'.to_string()),
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

                Token::new("STRING".to_string(), result)
            }
            '|' => {
                // we check if the | serves as | or ||
                if self.peek_char() == '|' {
                    self.read_char();
                    Token::new("OR".to_string(), "||".to_string())
                } else {
                    Token::new("PIPE".to_string(), "|".to_string())
                }
            }
            '&' => {
                // we check if the & serves as && or &
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::new("AND".to_string(), "&&".to_string())
                } else {
                    Token::new("AMPERSAND".to_string(), "&".to_string())
                }
            }
            '0'..='9' => Token::new("INT".to_string(), self.read_number()),
            'a'..='z' | 'A'..='Z' | '_' => {
                Token::new("IDENTIFIER".to_string(), self.read_identifier())
            }
            '\0' => Token::new("EOF".to_string(), "EOF".to_string()),
            _ => {
                self.read_char();
                Token::new("UNKNOWN".to_string(), "".to_string())
            }
        };

        // we give back an OK result so that we are sure that there are no errors.
        Ok(token)
    }

    pub fn input(&self) -> &str {
        self.input.as_ref()
    }
}
