use crate::lexer::token::Token;

use super::general::Node;

#[derive(PartialEq, Debug)]
pub struct TypeNode<'a> {
    name: String,
    fields: Vec<Box<Node<'a>>>,
}

impl<'a> TypeNode<'a> {
    pub fn new(name: String, fields: Vec<Box<Node<'a>>>) -> Self {
        Self { name, fields }
    }
}

#[derive(PartialEq, Debug)]
pub struct BlockTypeNode<'a> {
    pub name: String,
    pub fields: Vec<Box<Node<'a>>>,
}

impl<'a> BlockTypeNode<'a> {
    pub fn new(name: String, fields: Vec<Box<Node<'a>>>) -> Self {
        Self { name, fields }
    }
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
pub struct StringNode<'a> {
    token: Token<'a>,
    length: usize,
}

impl<'a> StringNode<'a> {
    pub fn new(token: Token<'a>, length: usize) -> Self {
        Self { token, length }
    }
}
