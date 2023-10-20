use std::vec;

use crate::{
    lexer::token::{Token, TokenType},
    parser::ast::functions::NodeArg,
};

use self::ast::{
    functions::{NodeFunction, NodeFunctionDecl, NodeReturn, NodeScope},
    math::{ast::NodeNumber, NodeProcessedBinOp, NodeProcessedUnOp},
    Nodes,
};

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
            file_name,
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

            if new_node != Nodes::NextLine {
                self.ast.push(new_node);
            }
        }
    }

    fn next(&mut self, add: usize) {
        for _ in 0..add {
            if self.tok_i < self.token_stream.len() {
                self.current_token = self.token_stream[self.tok_i];
            }

            self.tok_i += 1;
        }
    }

    fn back(&mut self, add: usize) {
        self.tok_i -= add;

        self.current_token = self.token_stream[self.tok_i];
    }

    fn peek(&self, add: usize) -> Token<'a> {
        self.token_stream[self.tok_i + add]
    }

    fn parse_list(&mut self, token: Token) -> Nodes<'a> {
        match token.token_type {
            TokenType::Minus | TokenType::Number | TokenType::Float => {
                let peek_token: Token<'a> = self.peek(0);

                if peek_token.token_type == TokenType::Plus
                    || peek_token.token_type == TokenType::Minus
                    || peek_token.token_type == TokenType::Multiply
                    || peek_token.token_type == TokenType::Divide
                {
                    // BinOp
                    return Nodes::NodeProcessedBinOp(self.parse_bin_op());
                }

                if self.current_token.token_type == TokenType::Minus
                    || (peek_token.token_type == TokenType::UnMinus
                        || peek_token.token_type == TokenType::UnPlus)
                {
                    return Nodes::NodeProcessedUnOp(self.parse_un_op());
                }

                Nodes::NodeNumber(NodeNumber(self.current_token))
            }
            TokenType::FunctionDecl => Nodes::NodeFunction(self.parse_function_decl()),
            TokenType::Return => Nodes::NodeReturn(self.parse_return()),
            TokenType::Semicolon => Nodes::NextLine,
            _ => {
                println!("Unknown token: {}", self.current_token.slice);
                Nodes::Null
            }
        }
    }

    // Operations

    fn parse_bin_op(&mut self) -> NodeProcessedBinOp<'a> {
        match self.peek(0).token_type {
            TokenType::Plus | TokenType::Minus | TokenType::Divide | TokenType::Multiply => {
                let mut tok_stream: Vec<Token<'a>> = vec![];

                loop {
                    tok_stream.push(self.current_token);

                    self.next(1);

                    if self.current_token.token_type == TokenType::Semicolon {
                        self.back(1);
                        break;
                    }
                }

                return NodeProcessedBinOp::new(tok_stream);
            }
            _ => panic!(),
        }
    }

    fn parse_un_op(&mut self) -> NodeProcessedUnOp<'a> {
        let peek_token: Token<'a> = self.peek(0);

        let toks: Vec<Token<'a>> = vec![self.current_token, peek_token];

        self.next(2);

        NodeProcessedUnOp::new(toks)
    }

    // Args

    fn parse_args(&mut self) -> Vec<NodeArg<'a>> {
        let mut args: Vec<NodeArg<'_>> = vec![];

        loop {
            let arg_name: &str = self.current_token.slice;

            self.next(2);

            let arg_ty: TokenType = self.current_token.token_type;

            args.push(NodeArg::new(arg_name, arg_ty));

            self.next(1);

            match self.current_token.token_type {
                TokenType::Comma => self.next(1),
                TokenType::ParenClose => break,
                _ => (),
            }
        }

        args
    }

    // Functions

    fn parse_function_decl(&mut self) -> NodeFunction<'a> {
        self.next(1);

        let name: &str = self.current_token.slice;

        self.next(2);

        let args: Vec<NodeArg<'a>> = self.parse_args();

        self.next(2);

        let ty: TokenType = self.current_token.token_type;

        self.next(1);

        self.parse_function(NodeFunctionDecl::new(name, ty, args))
    }

    fn parse_function(&mut self, decl: NodeFunctionDecl<'a>) -> NodeFunction<'a> {
        let mut vec_scope: Vec<Nodes<'a>> = vec![];

        loop {
            self.next(1);
            if self.current_token.token_type == TokenType::BlockEnd {
                break;
            }

            vec_scope.push(self.parse_list(self.current_token));
        }

        NodeFunction::new(decl, NodeScope::new(vec_scope))
    }

    fn parse_return(&mut self) -> NodeReturn<'a> {
        self.next(1);

        let val: Nodes<'a> = if self.current_token.token_type == TokenType::Semicolon {
            Nodes::NullValue
        } else {
            self.parse_list(self.current_token)
        };

        NodeReturn::new(val)
    }
}
