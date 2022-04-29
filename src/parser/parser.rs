/*
Jay PEG parser
Copyright (C) 2022  Loris Cuntreri
*/
use super::ast::*;
use crate::lexer::token::Token;

pub struct Parser<'a> {
    pub token_stream: Vec<Token<'a>>,
    pub current_token: Token<'a>,
    pub tok_i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        Self {
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
        }
    }

    pub fn parse(&mut self) {
        for _ in 0..self.token_stream.len() {
            self.next();
            println!("{:?}", self.current_token);
        }
    }

    fn next(&mut self) {
        if self.tok_i < self.token_stream.len() {
            self.current_token = self.token_stream[self.tok_i];
        }

        self.tok_i += 1;
    }

    fn parse_number(&mut self) -> Node<NumberNode> {
        let token: Token = self.current_token.clone();
        self.next();
        Node::new(vec![], NumberNode::new(token))
    }
}
