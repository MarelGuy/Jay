use crate::lexer::token::Token;

use super::functions::ArgNode;

#[derive(Debug, PartialEq, Clone)]
pub struct PrimitiveTypeNode<'a>(pub Token<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct TypeNode {
    type_name: String,
    args: Vec<ArgNode>,
}

impl TypeNode {
    pub fn new(type_name: String, args: Vec<ArgNode>) -> Self {
        Self { type_name, args }
    }
}
