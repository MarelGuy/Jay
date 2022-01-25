use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub struct Token {
    type_: String,
    value: String,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Lexer {{ Type: {}, Value: {} }}", self.type_, self.value)
    }
}

impl Token {
    pub fn get_token(self) -> String {
        if self.value == "" {
            return self.type_.to_owned();
        } else {
            return self.type_.to_owned() + ": " + &self.value.to_owned();
        }
    }

    pub fn new(type_: String, value: String) -> Token {
        Token {
            type_: type_,
            value: value,
        }
    }
}
