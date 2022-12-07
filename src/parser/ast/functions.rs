use either::Either;

use crate::parser::Parser;

use super::variables::{ArrayVarType, VarNode, VarType};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode<'a> {
    name: String,
    args: ArgNode,
    scope: ScopeNode<'a>,
    ret_ty: Either<VarType, ArrayVarType>,
}

impl<'a> FunctionNode<'a> {
    pub fn new(
        name: String,
        args: ArgNode,
        ret_ty: Either<VarType, ArrayVarType>,
        scope: ScopeNode<'a>,
    ) -> Self {
        Self {
            name,
            args,
            ret_ty,
            scope,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScopeNode<'a> {
    scope: Parser<'a>,
    var_vec: Vec<VarNode<'a>>,
}

impl<'a> ScopeNode<'a> {
    pub fn new(scope: Parser<'a>, var_vec: Vec<VarNode<'a>>) -> Self {
        Self { scope, var_vec }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArgNode {
    name: String,
    ty: VarType,
}

impl ArgNode {
    pub fn new(name: String, ty: VarType) -> Self {
        Self { name, ty }
    }
}
