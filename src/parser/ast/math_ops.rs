use crate::lexer::token::Token;

use super::{node::Node, types::NumberNode};

pub struct BinOpNode<'a> {
    left_node: Node<NumberNode<'a>>,
    op_token: Token<'a>,
    right_node: Node<NumberNode<'a>>,
}

impl<'a> BinOpNode<'a> {
    pub fn new(
        left_node: Node<NumberNode<'a>>,
        op_token: Token<'a>,
        right_node: Node<NumberNode<'a>>,
    ) -> Self {
        Self {
            left_node,
            op_token,
            right_node,
        }
    }
}

pub struct UnOpNode<'a> {
    op_token: Token<'a>,
    node: Node<NumberNode<'a>>,
}

impl<'a> UnOpNode<'a> {
    pub fn new(op_token: Token<'a>, node: Node<NumberNode<'a>>) -> Self {
        Self { op_token, node }
    }
}
