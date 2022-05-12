use crate::lexer::token::{Span, Token, TokenType};

use super::ast::{BinOpNode, Node, Nodes, NumberNode, UnOpNode};

pub struct Parser<'a> {
    pub token_stream: Vec<Token<'a>>,
    pub current_token: Token<'a>,
    pub tok_i: usize,
    pub ast: Box<Node<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        Self {
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
            ast: Box::new(Node::new(vec![], Box::new(Nodes::NullNode))),
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

    fn parse_list(&mut self, current_token: Token) -> Box<Node<'a>> {
        match current_token.token_type {
            TokenType::Number | TokenType::Float => {
                if self.peek().token_type == TokenType::Plus
                    || self.peek().token_type == TokenType::Minus
                    || self.peek().token_type == TokenType::Multiply
                    || self.peek().token_type == TokenType::Divide
                    || self.peek().token_type == TokenType::Power
                    || self.peek().token_type == TokenType::Modulo
                {
                    self.parse_bin_op()
                } else if self.peek().token_type == TokenType::PlusPlus
                    || self.peek().token_type == TokenType::MinusMinus
                {
                    self.parse_un_op()
                } else {
                    self.parse_number()
                }
            }
            _ => Box::new(Node::new(vec![], Box::new(Nodes::NullNode))),
        }
    }

    pub fn parse(&mut self) {
        let mut children: Vec<Box<Node>> = Vec::new();

        while self.tok_i < self.token_stream.len() {
            self.next();

            let node = self.parse_list(self.current_token);

            if node.node != Box::new(Nodes::NullNode) {
                children.push(node);
            }
        }

        self.ast = Box::new(Node::new(children, Box::new(Nodes::NullNode)));
    }

    fn parse_number(&self) -> Box<Node<'a>> {
        let token: Token = self.current_token.clone();

        return Box::new(Node::new(
            vec![],
            Box::new(Nodes::NumberNode(NumberNode::new(token))),
        ));
    }

    fn parse_bin_op(&mut self) -> Box<Node<'a>> {
        let left_node: Box<Node> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;
        self.next();

        let right_node: Box<Node> = self.parse_number();

        self.next();

        return Box::new(Node::new(
            vec![],
            Box::new(Nodes::BinOpNode(BinOpNode::new(
                left_node, op_token, right_node,
            ))),
        ));
    }

    fn parse_un_op(&mut self) -> Box<Node<'a>> {
        let number_node: Box<Node> = self.parse_number();
        self.next();

        let op_token: Token = self.current_token;

        self.next();

        Box::new(Node::new(
            vec![],
            Box::new(Nodes::UnOpNode(UnOpNode::new(op_token, number_node))),
        ))
    }
}
