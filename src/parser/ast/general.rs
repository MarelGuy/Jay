use crate::lexer::token::Token;

use super::{
    declarations::{ConstDeclNode, VarDeclNode},
    if_else::IfNode,
    math_ops::{BinOpNode, UnOpNode},
    types::NumberNode,
};

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct BlockNode<'a> {
    pub block: Box<Node<'a>>,
}

impl<'a> BlockNode<'a> {
    pub fn new(block: Box<Node<'a>>) -> Self {
        Self { block }
    }
}

#[derive(PartialEq, Debug)]
pub enum Nodes<'a> {
    VarDeclNode(VarDeclNode),
    ConstDeclNode(ConstDeclNode),
    ConditionNode(ConditionNode<'a>),
    BlockNode(BlockNode<'a>),
    IfNode(IfNode<'a>),
    BinOpNode(BinOpNode<'a>),
    UnOpNode(UnOpNode<'a>),
    NumberNode(NumberNode<'a>),
    NullNode,
}

#[derive(PartialEq, Debug)]
pub struct Node<'a> {
    pub children: Vec<Box<Node<'a>>>,
    pub node: Box<Nodes<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(children: Vec<Box<Node<'a>>>, node: Box<Nodes<'a>>) -> Self {
        Self { children, node }
    }
}
