use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

const TT_INT: &str = "TT_INT";
const TT_FLOAT: &str = "TT_FLOAT";
const TT_PLU: &str = "TT_PLUS";
const TT_MINUS: &str = "TT_MINUS";
const TT_MUL: &str = "TT_MUL";
const TT_DIV: &str = "TT_DIV";
const TT_LPAREN: &str = "TT_LPAREN";
const TT_RPAREN: &str = "TT_RPAREN";

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
    pub fn new(type_: String, value: String) -> Token {
        Token {
            type_: type_,
            value: value,
        }
    }

    pub fn getToken(self) -> String {
        if self.value == "" {
            return self.type_.to_owned();
        } else {
            return self.type_.to_owned() + ": " + &self.value.to_owned();
        }
    }
}
