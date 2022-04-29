/*
 Jay AST
 Copyright (C) 2022 Loris Cuntreri
*/

use crate::lexer::token::Token;

pub struct Node<T> {
    children: Vec<T>,
    node: T,
}

impl<T> Node<T> {
    pub fn new(children: Vec<T>, node: T) -> Self {
        Self { children, node }
    }
}

pub struct NumberNode<'a> {
    token: Token<'a>,
}

impl<'a> NumberNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

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
