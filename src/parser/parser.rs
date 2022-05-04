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

    fn parse_number(&self) -> Node<NumberNode<'a>> {
        let token: Token = self.current_token.clone();
        Node::new(vec![], NumberNode::new(token))
    }

    fn parse_bin_op(&mut self) -> Node<BinOpNode<'a>> {
        let left_node: Node<NumberNode> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        self.next();

        let right_node: Node<NumberNode> = self.parse_number();
        self.next();

        Node::new(vec![], BinOpNode::new(left_node, op_token, right_node))
    }

    fn parse_un_op(&mut self) -> Node<UnOpNode<'a>> {
        let number_node: Node<NumberNode> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        self.next();

        Node::new(vec![], UnOpNode::new(op_token, number_node))
    }
}
