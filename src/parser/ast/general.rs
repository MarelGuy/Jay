use either::Either;

use crate::lexer::token::Token;

use super::{
    declarations::{ConstDeclNode, VarDeclNode, VarType},
    functions::{FunctionDeclNode, FunctionNode, ReturnIfNode, ReturnNode, UseFunctionNode},
    identifier::IdentifierNode,
    if_else::IfNode,
    import_export::{ExportNode, ImportNode},
    loops::{ForNode, LoopNode, WhileNode},
    math_ops::{BinOpNode, UnOpNode},
    switch::SwitchNode,
    types::{BoolNode, CharNode, NumberNode, StringNode, TypeNode},
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
    pub block: Vec<Box<Node<'a>>>,
}

impl<'a> BlockNode<'a> {
    pub fn new(block: Vec<Box<Node<'a>>>) -> Self {
        Self { block }
    }
}

#[derive(PartialEq, Debug)]
pub struct ParamNode {
    pub name: String,
    pub ty: Either<FunctionDeclNode, VarType>,
}

impl ParamNode {
    pub fn new(name: String, ty: Either<FunctionDeclNode, VarType>) -> Self {
        Self { name, ty }
    }
}

#[derive(PartialEq, Debug)]
pub enum Nodes<'a> {
    // Declarations
    VarDeclNode(VarDeclNode<'a>),
    ConstDeclNode(ConstDeclNode<'a>),

    // Identifiers
    IdentifierNode(IdentifierNode<'a>),

    // If-else
    IfNode(IfNode<'a>),

    // Switch
    SwitchNode(SwitchNode<'a>),

    // Ops
    BinOpNode(BinOpNode<'a>),
    UnOpNode(UnOpNode<'a>),

    // Types
    NumberNode(NumberNode<'a>),
    StringNode(StringNode<'a>),
    CharNode(CharNode<'a>),
    BoolNode(BoolNode<'a>),
    TypeNode(TypeNode),

    // Loops
    WhileNode(WhileNode<'a>),
    ForNode(ForNode<'a>),
    LoopNode(LoopNode<'a>),

    // Functions
    FunctionNode(FunctionNode<'a>),
    UseFunctionNode(UseFunctionNode<'a>),
    ReturnNode(ReturnNode<'a>),
    ReturnIfNode(ReturnIfNode<'a>),

    // Import & Export
    ImportNode(ImportNode<'a>),
    ExportNode(ExportNode<'a>),

    // Misc
    NullNode,
}

#[derive(PartialEq, Debug)]
pub struct Node<'a> {
    pub node: Box<Nodes<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(node: Box<Nodes<'a>>) -> Self {
        Self { node }
    }
}
