pub mod ast;

use bumpalo::Bump;

use crate::lexer::token::{Token, TokenType};

use self::ast::{
    NodeBinOpType, NodeNumber,
    OpType::{self, Divide, Minus, Multiply, Plus},
};

use super::Nodes;

#[derive(Debug, PartialEq)]
pub struct NodeProcessedBinOp<'a> {
    out_stream: Vec<Nodes<'a>>,
}

impl<'a> NodeProcessedBinOp<'a> {
    pub fn new(tok_stream: Vec<Token<'a>>, arena: &'a Bump) -> Self {
        let mut out_stream: Vec<Nodes<'a>> = vec![];
        let mut old_tok: Token<'a> = Token::placeholder();
        let mut is_un_op: bool = false;

        tok_stream.into_iter().for_each(|tok| {
            if is_un_op {
                out_stream.pop();
                out_stream.push(Nodes::NodeProcessedUnOp(
                    arena.alloc(NodeProcessedUnOp::new(vec![old_tok, tok])),
                ));
                is_un_op = false;
            } else {
                out_stream.push(match tok.token_type {
                    TokenType::Number => Nodes::NodeNumber(arena.alloc(NodeNumber(tok))),
                    TokenType::Plus => {
                        Nodes::NodeBinOpType(arena.alloc(NodeBinOpType::new(Plus, 0)))
                    }
                    TokenType::Minus => {
                        if old_tok.token_type == TokenType::Plus
                            || old_tok.token_type == TokenType::Minus
                            || old_tok.token_type == TokenType::Multiply
                            || old_tok.token_type == TokenType::Divide
                        {
                            is_un_op = true;
                        }
                        Nodes::NodeBinOpType(arena.alloc(NodeBinOpType::new(Minus, 0)))
                    }
                    TokenType::Multiply => {
                        Nodes::NodeBinOpType(arena.alloc(NodeBinOpType::new(Multiply, 0)))
                    }
                    TokenType::Divide => {
                        Nodes::NodeBinOpType(arena.alloc(NodeBinOpType::new(Divide, 0)))
                    }
                    _ => Nodes::Null,
                });
                old_tok = tok;
            }
        });

        Self { out_stream }
    }
}

#[derive(Debug, PartialEq)]
pub struct NodeProcessedUnOp<'a> {
    op: OpType,
    rhs: NodeNumber<'a>,
}

impl<'a> NodeProcessedUnOp<'a> {
    pub fn new(toks: Vec<Token<'a>>) -> Self {
        let op: OpType;
        let rhs: NodeNumber<'a>;

        if toks[0].token_type == TokenType::Number {
            rhs = NodeNumber(toks[0]);
            op = OpType::from(toks[1].token_type);
        } else {
            rhs = NodeNumber(toks[1]);
            op = OpType::from(toks[0].token_type);
        }

        Self { op, rhs }
    }
}
