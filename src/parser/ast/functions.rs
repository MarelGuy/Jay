use either::Either;

use super::{
    declarations::{TypeName, VarType},
    general::{BlockNode, Node, ParamNode},
};

#[derive(PartialEq, Debug)]
pub struct FunctionDeclNode {
    pub name: String,
    pub args: Vec<Box<ParamNode>>,
    pub ret_ty: Either<VarType, TypeName>,
}

impl<'a> FunctionDeclNode {
    pub fn new(name: String, args: Vec<Box<ParamNode>>, ret_ty: Either<VarType, TypeName>) -> Self {
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