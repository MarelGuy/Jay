use crate::{error_handler::Error, lexer::token::Span};
use std::vec;

use crate::lexer::token::{Token, TokenType};

use self::ast::{primitive_node::PrimitiveTypeNode, variables::VarNode, Node, Nodes};

mod ast;
mod math;

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

            if new_node != Node(Nodes::NullNode) {
                self.ast.push(new_node);
            }
        }
    }

    fn back(&mut self) {
        self.tok_i -= 1;

        self.current_token = self.token_stream[self.tok_i];
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
                line: 0,
                column: 0,
            }
        }
    }

    #[rustfmt::skip]    fn get_line(&self, line: usize) -> String { self.lines.clone().into_iter().nth(line).unwrap() }

    fn parse_list(&mut self, token: Token<'a>) -> Node<'a> {
        match token.token_type {
            TokenType::Semicolon => Node(Nodes::EOL),
            TokenType::Number
            | TokenType::Float
            | TokenType::String
            | TokenType::Char
            | TokenType::Bool
            | TokenType::Identifier
            | TokenType::NegativeFloat
            | TokenType::NegativeNumber => {
                if self.peek().token_type == TokenType::Plus
                    || self.peek().token_type == TokenType::Minus
                    || self.peek().token_type == TokenType::Divide
                    || self.peek().token_type == TokenType::Multiply
                {
                    let mut tok_stream: Vec<Token> = vec![];

                    loop {
                        tok_stream.push(self.current_token);

                        self.next();

                        if self.current_token.token_type == TokenType::Semicolon {
                            self.back();
                            break;
                        }
                    }

                    Node(Nodes::ProcessedMathNode(math::process_math_node(
                        tok_stream,
                        self.get_line(self.current_token.line),
                        self.file_name.clone(),
                    )))
                } else {
                    Node(Nodes::PrimitiveTypeNode(self.parse_primitive_type_node()))
                }
            }
            TokenType::Let | TokenType::Var | TokenType::Const => {
                Node(Nodes::VarNode(self.parse_var()))
            }
            _ => {
                Error::new(
                    self.current_token,
                    self.get_line(self.current_token.line),
                    self.file_name.clone(),
                )
                .throw_unkown_token();
                Node(Nodes::NullNode)
            }
        }
    }

    #[rustfmt::skip]    fn parse_primitive_type_node(&mut self) -> PrimitiveTypeNode<'a> { PrimitiveTypeNode(self.current_token) }

    fn parse_var(&mut self) -> VarNode<'a> {
        todo!()
    }
}
