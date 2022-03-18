/*
Jay tokenizer
Copyright (C) 2022  Loris Cuntreri
*/
pub(crate) struct Token {
    token_type: String,
    value: String,
}

impl Token {
    pub fn new(token_type: String, value: String) -> Token {
        Token { token_type, value }
    }

    pub fn output(&self) {
        println!(
            "Token {{ token_type: {}, value: {} }}",
            self.token_type, self.value
        )
    }

    pub fn get_token_type(&self) -> &String {
        &self.token_type
    }
}
