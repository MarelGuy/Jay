use crate::lexer::token::Token;

use super::general::Node;

#[derive(PartialEq, Debug)]
pub struct BinOpNode<'a> {
    left_node: Box<Node<'a>>,
    op_token: Token<'a>,
    right_node: Box<Node<'a>>,
}

impl<'a> BinOpNode<'a> {
    pub fn new(left_node: Box<Node<'a>>, op_token: Token<'a>, right_node: Box<Node<'a>>) -> Self {
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
    node: Box<Node<'a>>,
}

impl<'a> UnOpNode<'a> {
    pub fn new(op_token: Token<'a>, node: Box<Node<'a>>) -> Self {
        Self { op_token, node }
    }
}
