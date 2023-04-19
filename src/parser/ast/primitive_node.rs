use crate::lexer::token::Token;

use super::functions::ArgNode;

#[derive(Debug, PartialEq, Clone)]
pub struct PrimitiveTypeNode<'a>(pub Token<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct TypeNode {
    pub name: String,
    args: Vec<ArgNode>,
}

impl TypeNode {
    pub fn new(name: String, args: Vec<ArgNode>) -> Self {
        Self { name, args }
    }
}
