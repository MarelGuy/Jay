use crate::lexer::token::Token;

use super::types::NumberNode;

#[derive(PartialEq, Debug)]
pub struct BinOpNode<'a> {
    left_node: Box<NumberNode<'a>>,
    op_token: Token<'a>,
    right_node: Box<NumberNode<'a>>,
}

impl<'a> BinOpNode<'a> {
    pub fn new(
        left_node: Box<NumberNode<'a>>,
        op_token: Token<'a>,
        right_node: Box<NumberNode<'a>>,
    ) -> Self {
        Self {
            left_node,
            op_token,
            right_node,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct UnOpNode<'a> {
    op_token: Token<'a>,
    node: Box<NumberNode<'a>>,
}

impl<'a> UnOpNode<'a> {
    pub fn new(op_token: Token<'a>, node: Box<NumberNode<'a>>) -> Self {
        Self { op_token, node }
    }
}
