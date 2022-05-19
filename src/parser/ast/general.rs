use crate::lexer::token::Token;

use super::{
    declarations::{ConstDeclNode, TypeNode, VarDeclNode, VarType},
    functions::FunctionNode,
    if_else::IfNode,
    loops::{ForNode, LoopNode, WhileNode},
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
pub struct ParamNode {
    pub name: String,
    pub ty: VarType,
}

impl<'a> ParamNode {
    pub fn new(name: String, ty: VarType) -> Self {
        Self { name, ty }
    }
}

#[derive(PartialEq, Debug)]
pub enum Nodes<'a> {
    // Declarations
    VarDeclNode(VarDeclNode),
    ConstDeclNode(ConstDeclNode),

    // General
    ConditionNode(ConditionNode<'a>),
    BlockNode(BlockNode<'a>),

    // If-else
    IfNode(IfNode<'a>),

    // Ops
    BinOpNode(BinOpNode<'a>),
    UnOpNode(UnOpNode<'a>),

    // Types
    NumberNode(NumberNode<'a>),
    TypeNode(TypeNode<'a>),

    // Loops
    WhileNode(WhileNode<'a>),
    ForNode(ForNode<'a>),
    LoopNode(LoopNode<'a>),

    // Functions
    FunctionNode(FunctionNode<'a>),
    ParamNode(ParamNode),

    // Misc
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
