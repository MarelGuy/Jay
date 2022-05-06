use crate::lexer::token::Token;

#[derive(Debug)]
pub struct Node<T> {
    children: Vec<T>,
    pub node: T,
}

impl<T> Node<T> {
    pub fn new(children: Vec<T>, node: T) -> Self {
        Self { children, node }
    }
}

#[derive(Debug)]
pub struct ConditionNode<'a> {
    left_token: Token<'a>,
    op_token: Token<'a>,
    right_token: Token<'a>,
}

impl<'a> ConditionNode<'a> {
    pub fn new(left_token: Token<'a>, op_token: Token<'a>, right_token: Token<'a>) -> Self {
        Self {
            left_token,
            op_token,
            right_token,
        }
    }
}

pub struct BlockNode {
    pub block: String,
}

impl BlockNode {
    pub fn new(block: String) -> Self {
        Self { block }
    }
}
