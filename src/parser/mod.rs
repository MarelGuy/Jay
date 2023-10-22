use std::vec;

use bumpalo::Bump;

use crate::lexer::token::{Token, TokenType};

use self::ast::{
    functions::{NodeArg, NodeFunction, NodeFunctionDecl, NodeReturn, NodeScope},
    math::{ast::NodeNumber, NodeProcessedBinOp, NodeProcessedUnOp},
    Nodes,
};

pub(crate) mod ast;

#[derive(Debug)]
pub struct Parser<'a> {
    token_stream: Vec<Token<'a>>,
    // file_name: String,
    // lines: Vec<String>,
    current_token: Token<'a>,
    tok_i: usize,
    pub ast: Vec<Nodes<'a>>,
}

impl<'a> Parser<'a> {
    // * Main functions

    pub fn new(token_stream: Vec<Token<'a>>, /*file_name: String, lines: Vec<String>*/) -> Self {
        let init_tok: Token = token_stream[0];

        Self {
            // file_name,
            current_token: init_tok,
            token_stream,
            tok_i: 0,
            // lines,
            ast: vec![],
        }
    }

    pub fn parse(&mut self, arena: &'a Bump) -> Vec<Nodes<'a>> {
        let mut ast: Vec<Nodes> = vec![];

        while self.tok_i < self.token_stream.len() {
            self.next(1);

            let new_node: Nodes = self.parse_list(self.current_token, arena);

            if new_node != Nodes::NextLine {
                ast.push(new_node);
            }
        }

        ast
    }

    fn parse_list(&mut self, token: Token<'a>, arena: &'a Bump) -> Nodes<'a> {
        match token.token_type {
            TokenType::Minus | TokenType::Number | TokenType::Float => {
                let peek_token: Token<'a> = self.peek(0);

                if peek_token.token_type == TokenType::Plus
                    || peek_token.token_type == TokenType::Minus
                    || peek_token.token_type == TokenType::Multiply
                    || peek_token.token_type == TokenType::Divide
                {
                    // BinOp
                    return Nodes::NodeProcessedBinOp(arena.alloc(self.parse_bin_op(arena)));
                }

                if token.token_type == TokenType::Minus
                    || (peek_token.token_type == TokenType::UnMinus
                        || peek_token.token_type == TokenType::UnPlus)
                {
                    return Nodes::NodeProcessedUnOp(arena.alloc(self.parse_un_op()));
                }

                return Nodes::NodeNumber(arena.alloc(NodeNumber(self.current_token)));
            }
            TokenType::FunctionDecl => {
                Nodes::NodeFunction(arena.alloc(self.parse_function_decl(arena)))
            }
            TokenType::Return => Nodes::NodeReturn(arena.alloc(self.parse_return(arena))),
            TokenType::Semicolon => Nodes::NextLine,
            _ => {
                println!("Unknown token: {}", self.current_token.slice);
                Nodes::Null
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

    // // Operations

    fn parse_bin_op(&mut self, arena: &'a Bump) -> NodeProcessedBinOp<'a> {
        let mut tok_stream: Vec<Token<'a>> = vec![];

        loop {
            tok_stream.push(self.current_token);

            self.next(1);

            if self.current_token.token_type == TokenType::Semicolon {
                self.back(1);
                break;
            }
        }

        NodeProcessedBinOp::new(tok_stream, arena)
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

    fn parse_function_decl(&mut self, arena: &'a Bump) -> NodeFunction<'a> {
        self.next(1);

        let name: &str = self.current_token.slice;

        self.next(2);

        let args: Vec<NodeArg<'a>> = self.parse_args();

        self.next(2);

        let ty: TokenType = self.current_token.token_type;

        self.next(1);

        self.parse_function(NodeFunctionDecl::new(name, ty, args), arena)
    }

    fn parse_function(&mut self, decl: NodeFunctionDecl<'a>, arena: &'a Bump) -> NodeFunction<'a> {
        let mut vec_scope: Vec<Nodes<'a>> = vec![];

        loop {
            self.next(1);
            if self.current_token.token_type == TokenType::BlockEnd {
                break;
            }

            vec_scope.push(self.parse_list(self.current_token, arena));
        }

        NodeFunction::new(decl, NodeScope::new(vec_scope))
    }

    fn parse_return(&mut self, arena: &'a Bump) -> NodeReturn<'a> {
        self.next(1);

        let val: Nodes<'a> = if self.current_token.token_type == TokenType::Semicolon {
            Nodes::NullValue
        } else {
            self.parse_list(self.current_token, arena)
        };

        NodeReturn::new(val)
    }
}
