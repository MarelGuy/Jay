use crate::{error_handler::Error, parser::ast::primitive_nodes::NumberNode};
use std::vec;

use crate::lexer::token::{Token, TokenType};

use self::ast::{Node, Nodes};

mod ast;

pub struct Parser<'a> {
    token_stream: Vec<Token<'a>>,
    file_name: String,
    lines: Vec<String>,
    current_token: Token<'a>,
    tok_i: usize,
    pub ast: Vec<Node<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        Self {
            file_name,
            current_token: token_stream[0].clone(),
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next();

            let new_node: Node<'a> = self.parse_list(self.current_token);

            if new_node != Node::new(Nodes::NullNode) {
                self.ast.push(new_node);
            }
        }
    }

    fn next(&mut self) {
        if self.tok_i < self.token_stream.len() {
            self.current_token = self.token_stream[self.tok_i];
        }

        self.tok_i += 1;
    }

    fn get_line(&self, line: usize) -> String {
        self.lines.clone().into_iter().nth(line).unwrap()
    }

    fn parse_list(&mut self, token: Token<'a>) -> Node<'a> {
        match token.token_type {
            TokenType::Number => Node::new(Nodes::NumberNode(self.parse_int())),
            _ => {
                Error::new(
                    self.current_token,
                    self.get_line(self.current_token.line),
                    self.file_name.clone(),
                )
                .throw_unkown_token();
                return Node::new(Nodes::NullNode);
            }
        }
    }

    fn parse_int(&mut self) -> NumberNode<'a> {
        let number_node: Token = self.current_token;

        self.next();

        return NumberNode::new(number_node);
    }
}
