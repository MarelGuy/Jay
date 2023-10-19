pub mod ast;

use crate::lexer::token::{Token, TokenType};

use self::ast::{
    BinOpTypeNode, NumberNode,
    OpType::{Divide, Minus, Multiply, Plus},
};

use super::Nodes;

#[derive(Debug, PartialEq, Clone)]
pub struct ProcessedBinOpNode<'a> {
    out_stream: Vec<Nodes<'a>>,
}

impl<'a> ProcessedBinOpNode<'a> {
    pub fn new(out_stream: Vec<Nodes<'a>>) -> Self {
        Self { out_stream }
    }
}

pub fn process_math_node(tok_stream: Vec<Token<'_>>) -> ProcessedBinOpNode {
    let mut out_stream: Vec<Nodes> = vec![];

    tok_stream.into_iter().for_each(|tok| {
        out_stream.push(match tok.token_type {
            TokenType::Number => Nodes::NumberNode(NumberNode(tok)),
            TokenType::Plus => Nodes::BinOpTypeNode(BinOpTypeNode::new(Plus)),
            TokenType::Minus => Nodes::BinOpTypeNode(BinOpTypeNode::new(Minus)),
            TokenType::Multiply => Nodes::BinOpTypeNode(BinOpTypeNode::new(Multiply)),
            TokenType::Divide => Nodes::BinOpTypeNode(BinOpTypeNode::new(Divide)),
            _ => Nodes::Null,
        })
    });

    ProcessedBinOpNode::new(out_stream)
}
