use either::Either;

use crate::lexer::token::Token;

use super::functions::{ArgNode, FunctionNode};

#[derive(Debug, PartialEq, Clone)]
pub struct PrimitiveTypeNode<'a>(pub Token<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct TypeNode<'a> {
    pub name: String,
    pub args: Vec<Either<ArgNode, FunctionNode<'a>>>,
}

impl<'a> TypeNode<'a> {
    pub fn new(name: String, args: Vec<Either<ArgNode, FunctionNode<'a>>>) -> Self {
        Self { name, args }
    }
}
