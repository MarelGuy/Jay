use either::Either;

use super::{
    declarations::{TypeName, VarType},
    general::{BlockNode, ParamNode},
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
