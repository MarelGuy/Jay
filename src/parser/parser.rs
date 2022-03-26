/*
Jay PEG parser
Copyright (C) 2022  Loris Cuntreri
*/
use crate::lexer::token::Token;

pub struct Parser<'a> {
    pub token_stream: Vec<Token<'a>>,
    pub current_token: usize,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        Self {
            token_stream,
            current_token: 0,
        }
    }

    pub fn parse(&mut self) /* -> Result<(), String> */
    {
        for token in &self.token_stream {
            println!("{:?}", &token)
        }
    }
}
