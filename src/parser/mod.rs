use crate::lexer::token::{Token, TokenType};

use self::ast::{
    math::{self, Operation, Operator},
    Nodes,
};
use math::Int;

pub(crate) mod ast;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a> {
    token_stream: Vec<Token<'a>>,
    file_name: String,
    lines: Vec<String>,
    current_token: Token<'a>,
    tok_i: usize,
    pub ast: Vec<Nodes<'a>>,
}

impl<'a> Parser<'a> {
    // * Main functions

    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        let init_tok: Token<'a> = token_stream[0];

        Self {
            file_name: file_name.clone(),
            current_token: init_tok,
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],
        }
    }

    fn next(&mut self, count: usize) {
        for _ in 0..count {
            if self.tok_i < self.token_stream.len() {
                self.current_token = self.token_stream[self.tok_i];
            }

            self.tok_i += 1;
        }
    }

    fn peek(&self) -> Token<'a> {
        self.token_stream[self.tok_i]
    }

    fn parse_list(&mut self, token: Token<'a>) -> Nodes<'a> {
        match token.token_type {
            TokenType::Number => {
                if self.peek().token_type == TokenType::Plus
                    || self.peek().token_type == TokenType::Minus
                    || self.peek().token_type == TokenType::Multiply
                    || self.peek().token_type == TokenType::Divide
                    || self.peek().token_type == TokenType::Modulo
                {
                    let lhs: Int<'_> = Int::new(self.current_token.slice);

                    self.next(1);

                    let op: Operator = Operator::get_op(self.current_token.token_type);

                    self.next(1);

                    let rhs: Int<'_> = Int::new(self.current_token.slice);

                    return Nodes::Op(Operation::new(lhs, rhs, op));
                }

                return Nodes::Int(Int::new(self.current_token.slice));
            }
            _ => Nodes::Null,
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next(1);

            let new_node: Nodes = self.parse_list(self.current_token);

            self.ast.push(new_node);
        }
    }
}
