/*
Jay parser
Copyright (C) 2022  Loris Cuntreri
*/
use crate::lexer::token::{Span, Token, TokenType};

use super::ast::math_ops::{BinOpNode, UnOpNode};
use super::ast::types::NumberNode;
use super::ast::Node::Node;

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

    fn next(&mut self) {
        if self.tok_i < self.token_stream.len() {
            self.current_token = self.token_stream[self.tok_i];
        }

        self.tok_i += 1;
    }

    fn peek(&self) -> Token<'a> {
        if self.tok_i < self.token_stream.len() {
            self.token_stream[self.tok_i].clone()
        } else {
            Token {
                token_type: TokenType::Null,
                slice: "",
                span: Span { start: 0, end: 0 },
            }
        }
    }

    pub fn parse(&mut self) {
        for _ in 0..self.token_stream.len() {
            self.next();
            match self.current_token.token_type {
                TokenType::Number => {
                    if self.peek().token_type == TokenType::Plus
                        || self.peek().token_type == TokenType::Minus
                        || self.peek().token_type == TokenType::Multiply
                        || self.peek().token_type == TokenType::Divide
                        || self.peek().token_type == TokenType::Modulo
                    {
                        self.parse_bin_op();
                    } else if self.peek().token_type == TokenType::PlusPlus
                        || self.peek().token_type == TokenType::MinusMinus
                    {
                        self.parse_un_op();
                    } else {
                        self.parse_number();
                    }
                }
                _ => self.next(),
            }
        }
    }

    fn parse_number(&self) -> Node<NumberNode<'a>> {
        println!("{:?}", self.current_token);
        let token: Token = self.current_token.clone();
        Node::new(vec![], NumberNode::new(token))
    }

    fn parse_bin_op(&mut self) -> Node<BinOpNode<'a>> {
        let left_node: Node<NumberNode> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        println!("{:?}", self.current_token);
        self.next();

        let right_node: Node<NumberNode> = self.parse_number();

        Node::new(vec![], BinOpNode::new(left_node, op_token, right_node))
    }

    fn parse_un_op(&mut self) -> Node<UnOpNode<'a>> {
        let number_node: Node<NumberNode> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        println!("{:?}", self.current_token);

        Node::new(vec![], UnOpNode::new(op_token, number_node))
    }
}
