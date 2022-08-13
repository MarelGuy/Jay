use crate::lexer::token::Token;

use super::{Node, ParamNode};

#[derive(PartialEq, Debug)]
pub struct TypeNode {
    name: String,
    fields: Vec<ParamNode>,
}

impl TypeNode {
    pub fn new(name: String, fields: Vec<ParamNode>) -> Self {
        Self { name, fields }
    }
}

#[derive(PartialEq, Debug)]
pub struct NewTypeValueNode<'a> {
    params: Vec<Node<'a>>,
}

impl<'a> NewTypeValueNode<'a> {
    pub fn new(params: Vec<Node<'a>>) -> Self {
        Self { params }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct NumberNode<'a> {
    token: Token<'a>,
}

impl<'a> NumberNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

#[derive(PartialEq, Debug)]
pub struct CharNode<'a> {
    token: Token<'a>,
}

impl<'a> CharNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

#[derive(PartialEq, Debug)]
pub struct BoolNode<'a> {
    token: Token<'a>,
}

impl<'a> BoolNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

#[derive(PartialEq, Debug)]
pub struct StringNode<'a> {
    token: Token<'a>,
    length: usize,
}

impl<'a> StringNode<'a> {
    pub fn new(token: Token<'a>, length: usize) -> Self {
        Self { token, length }
    }
}
