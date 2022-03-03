use core::fmt::{Debug, Formatter, Result};

pub(crate) struct Token {
    token_type: String,
    value: String,
}

// implement debug in token
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Token {{ token_type: {}, value: {} }}",
            self.token_type, self.value
        )
    }
}

impl Token {
    pub fn new(token_type: String, value: String) -> Token {
        Token { token_type, value }
    }
}
