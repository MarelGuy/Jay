use crate::{
    lexer::token::{Token, TokenType},
    parser::ast::math,
};

use self::ast::{functions::NodeFunctionDecl, Nodes};

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
        let init_tok: Token = token_stream[0];

        Self {
            file_name: file_name,
            current_token: init_tok,
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next(1);

            let new_node: Nodes<'a> = self.parse_list(self.current_token);

            if &new_node != &Nodes::NextLine {
                self.ast.push(new_node);
            }
        }
    }

    fn next(&mut self, count: usize) {
        for _ in 0..count {
            if self.tok_i < self.token_stream.len() {
                self.current_token = self.token_stream[self.tok_i];
            }

            self.tok_i += count;
        }
    }

    fn back(&mut self, count: usize) {
        self.tok_i -= count;

        self.current_token = self.token_stream[self.tok_i];
    }

    fn peek(&self, add: usize) -> Token {
        self.token_stream[self.tok_i + add]
    }

    fn parse_list(&mut self, token: Token) -> Nodes<'a> {
        match token.token_type {
            TokenType::Number | TokenType::Float => {
                if self.peek(0).token_type == TokenType::Plus
                    || self.peek(0).token_type == TokenType::Minus
                    || self.peek(0).token_type == TokenType::Multiply
                    || self.peek(0).token_type == TokenType::Divide
                {
                    match self.peek(0).token_type {
                        TokenType::Plus
                        | TokenType::Minus
                        | TokenType::Divide
                        | TokenType::Multiply => {
                            let mut tok_stream: Vec<Token<'a>> = vec![];

                            loop {
                                tok_stream.push(self.current_token);

                                self.next(1);

                                if self.current_token.token_type == TokenType::Semicolon {
                                    self.back(1);
                                    break;
                                }
                            }

                            return Nodes::ProcessedBinOpNode(math::process_math_node(tok_stream));
                        }
                        _ => todo!(),
                    }
                }

                todo!()
            }
            TokenType::FunctionDecl => Nodes::NodeFunctionDecl(self.parse_function()),
            TokenType::Semicolon => Nodes::NextLine,
            _ => Nodes::Null,
        }
    }

    fn parse_function(&mut self) -> NodeFunctionDecl<'a> {
        todo!()
    }
}
