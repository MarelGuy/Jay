/*
Jay PEG parser
Copyright (C) 2022  Loris Cuntreri
*/
use super::ast::*;
use crate::lexer::token::{Span, Token, TokenType};

pub struct Parser<'a> {
    pub token_stream: Vec<Token<'a>>,
    pub current_token: Token<'a>,
    pub tok_i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        Self {
            current_token: Token {
                token_type: TokenType::Null,
                slice: "",
                span: Span { start: 0, end: 0 },
            },
            token_stream,
            tok_i: 0,
        }
    }

    pub fn parse(&mut self) /* -> Result<(), String> */
    {
        for token in &self.token_stream {
            println!("{:?}", &token);
        }
    }

    fn advance(&mut self) {
        self.tok_i += 1;

        if self.tok_i < self.token_stream.len() {
            self.current_token = self.token_stream[self.tok_i];
        }
    }

    pub fn factor(&mut self) -> Result<NumberNode, ()> {
        if self.current_token.token_type == TokenType::Number
            || self.current_token.token_type == TokenType::Identifier
        {
            self.advance();
            Ok(
            NumberNode {
                token: self.current_token.clone(),
            })
        } else {
            Err(())
        }
    }

    pub fn term() {
        println!("term");
    }

    pub fn expr() {
        println!("expr");
    }
}
