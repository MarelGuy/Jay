use crate::lexer::token::Token;

use super::types::NumberNode;

#[derive(PartialEq, Debug)]
pub struct BinOpNode<'a> {
    pub left_node: NumberNode<'a>,
    op_token: Token<'a>,
    pub right_node: NumberNode<'a>,
}

impl<'a> BinOpNode<'a> {
    pub fn new(left_node: NumberNode<'a>, op_token: Token<'a>, right_node: NumberNode<'a>) -> Self {
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
    pub node: NumberNode<'a>,
}

impl<'a> UnOpNode<'a> {
    pub fn new(op_token: Token<'a>, node: NumberNode<'a>) -> Self {
        Self { op_token, node }
    }
}
