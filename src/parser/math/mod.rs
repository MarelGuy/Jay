pub mod ast;

use crate::{
    error_handler::Error,
    lexer::token::{Token, TokenType},
};

use self::ast::{
    MathIdNode, MathNumberNode, MathOpTypeNode,
    OpType::{Divide, Minus, Multiply, Plus},
};

use super::ast::{Node, Nodes};

#[derive(Debug, PartialEq)]
pub struct ProcessedMathNode<'a> {
    out_stream: Vec<Node<'a>>,
}

impl<'a> ProcessedMathNode<'a> {
    pub fn new(out_stream: Vec<Node<'a>>) -> Self {
        Self { out_stream }
    }
}

pub fn process_math_node<'a>(
    tok_stream: Vec<Token<'a>>,
    line_string: String,
    file_name: String,
) -> ProcessedMathNode<'a> {
    let mut out_stream: Vec<Node<'a>> = vec![];

    for tok in tok_stream {
        out_stream.push(match tok.token_type {
            TokenType::Number => Node(Nodes::MathNumberNode(MathNumberNode(tok))),
            TokenType::Plus => Node(Nodes::MathOpTypeNode(MathOpTypeNode::new(Plus))),
            TokenType::Minus => Node(Nodes::MathOpTypeNode(MathOpTypeNode::new(Minus))),
            TokenType::Multiply => Node(Nodes::MathOpTypeNode(MathOpTypeNode::new(Multiply))),
            TokenType::Divide => Node(Nodes::MathOpTypeNode(MathOpTypeNode::new(Divide))),
            TokenType::Identifier => Node(Nodes::MathIdNode(MathIdNode::new(tok))),
            _ => {
                Error::new(tok, line_string.clone(), file_name.clone())
                    .throw_unkown_token_in_math_expr();
                Node(Nodes::NullNode)
            }
        })
    }

    ProcessedMathNode::new(out_stream)
}
