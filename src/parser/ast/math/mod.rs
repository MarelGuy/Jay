pub mod ast;

use crate::lexer::token::{Token, TokenType};

use self::ast::{
    NodeBinOpType, NodeNumber,
    OpType::{Divide, Minus, Multiply, Plus},
};

use super::Nodes;

#[derive(Debug, PartialEq, Clone)]
pub struct NodeProcessedBinOp<'a> {
    out_stream: Vec<Nodes<'a>>,
}

impl<'a> NodeProcessedBinOp<'a> {
    pub fn new(out_stream: Vec<Nodes<'a>>) -> Self {
        Self { out_stream }
    }
}

pub fn process_math_node(tok_stream: Vec<Token<'_>>) -> NodeProcessedBinOp {
    let mut out_stream: Vec<Nodes> = vec![];

    tok_stream.into_iter().for_each(|tok| {
        out_stream.push(match tok.token_type {
            TokenType::Number => Nodes::NodeNumber(NodeNumber(tok)),
            TokenType::Plus => Nodes::NodeBinOpType(NodeBinOpType::new(Plus)),
            TokenType::Minus => Nodes::NodeBinOpType(NodeBinOpType::new(Minus)),
            TokenType::Multiply => Nodes::NodeBinOpType(NodeBinOpType::new(Multiply)),
            TokenType::Divide => Nodes::NodeBinOpType(NodeBinOpType::new(Divide)),
            _ => Nodes::Null,
        })
    });

    NodeProcessedBinOp::new(out_stream)
}
