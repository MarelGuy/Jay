use super::{declarations::VarType, BlockNode, ConditionNode, Node, ParamNode};

#[derive(PartialEq, Debug)]
pub struct FunctionDeclNode {
    pub name: String,
    pub args: Vec<Box<ParamNode>>,
    pub ret_ty: VarType,
}

impl FunctionDeclNode {
    pub fn new(name: String, args: Vec<Box<ParamNode>>, ret_ty: VarType) -> Self {
        Self { name, args, ret_ty }
    }
}

#[derive(PartialEq, Debug)]
pub struct FunctionNode<'a> {
    pub func_details: Box<FunctionDeclNode>,
    pub block: Box<BlockNode<'a>>,
}

impl<'a> FunctionNode<'a> {
    pub fn new(func_details: Box<FunctionDeclNode>, block: Box<BlockNode<'a>>) -> Self {
        Self {
            func_details,
            block,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ArgNode<'a> {
    pub value: Box<Node<'a>>,
}

impl<'a> ArgNode<'a> {
    pub fn new(value: Box<Node<'a>>) -> Self {
        Self { value }
    }
}

#[derive(PartialEq, Debug)]
pub struct UseFunctionNode<'a> {
    pub name: String,
    pub args: Vec<Box<ArgNode<'a>>>,
}

impl<'a> UseFunctionNode<'a> {
    pub fn new(name: String, args: Vec<Box<ArgNode<'a>>>) -> Self {
        Self { name, args }
    }
}

#[derive(PartialEq, Debug)]
pub struct ReturnNode<'a> {
    pub value: Box<Node<'a>>,
}

impl<'a> ReturnNode<'a> {
    pub fn new(value: Box<Node<'a>>) -> Self {
        Self { value }
    }
}

#[derive(PartialEq, Debug)]
pub struct ReturnIfNode<'a> {
    pub condition: Box<ConditionNode<'a>>,
    pub value: Box<Node<'a>>,
}

impl<'a> ReturnIfNode<'a> {
    pub fn new(condition: Box<ConditionNode<'a>>, value: Box<Node<'a>>) -> Self {
        Self { condition, value }
    }
}
