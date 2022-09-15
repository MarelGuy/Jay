use crate::lexer::token::Token;

use super::{types::NumberNode, Node};

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierNode<'a> {
    pub token: Token<'a>,
}

impl<'a> IdentifierNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayAccessNode<'a> {
    position: NumberNode<'a>,
}

impl<'a> ArrayAccessNode<'a> {
    pub fn new(position: NumberNode<'a>) -> Self {
        Self { position }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DotNotationNode<'a> {
    next_node: Box<Node<'a>>,
}

impl<'a> DotNotationNode<'a> {
    pub fn new(next_node: Box<Node<'a>>) -> Self {
        Self { next_node }
    }
}
