use either::Either;

use crate::lexer::token::Token;

use super::{
    functions::FunctionNode,
    variables::{ArrayVarType, VarType},
};

#[derive(Debug, PartialEq, Clone)]
pub struct PrimitiveTypeNode<'a>(pub Token<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct TypeArgNode<'a> {
    pub name: String,
    pub val: Either<Either<VarType, ArrayVarType>, FunctionNode<'a>>,
    pub is_priv: bool,
}

impl<'a> TypeArgNode<'a> {
    pub fn new(
        name: String,
        val: Either<Either<VarType, ArrayVarType>, FunctionNode<'a>>,
        is_priv: bool,
    ) -> Self {
        Self { name, val, is_priv }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeNode<'a> {
    pub name: String,
    pub args: Vec<TypeArgNode<'a>>,
}

impl<'a> TypeNode<'a> {
    pub fn new(name: String, args: Vec<TypeArgNode<'a>>) -> Self {
        Self { name, args }
    }
}
