use core::fmt::{Debug, Formatter, Result};

pub(crate) struct Token {
    token_type: String,
    value: String,
}

// we implemented the debug just for logging
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

    pub fn get_token_type(&self) -> &String {
        &self.token_type
    }
}
