use crate::lexer::token::Token;

use super::general::ParamNode;

#[derive(PartialEq, Debug)]
pub struct TypeNode {
    name: String,
    fields: Vec<Box<ParamNode>>,
}

impl TypeNode {
    pub fn new(name: String, fields: Vec<Box<ParamNode>>) -> Self {
        Self { name, fields }
    }
}

#[derive(PartialEq, Debug)]
pub struct TypeFunctionNode {
    pub name: String,
    pub params: Vec<Box<ParamNode>>,
    pub return_type: Box<TypeNode>,
}

#[derive(PartialEq, Debug)]
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
