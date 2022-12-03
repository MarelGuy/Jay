use crate::lexer::token::Token;

use self::{
    primitive_node::PrimitiveTypeNode,
    variables::{CallVarArrNode, CallVarNode, VarNode},
};

use super::math::{
    ast::{MathIdNode, MathNumberNode, MathOpTypeNode},
    ProcessedMathNode,
};

pub mod primitive_node;
pub mod variables;

#[derive(Debug, PartialEq, Clone)]
pub enum Nodes<'a> {
    // AST
    PrimitiveTypeNode(PrimitiveTypeNode<'a>),
    VarNode(VarNode<'a>),
    CallVarNode(CallVarNode<'a>),
    CallVarArrNode(CallVarArrNode<'a>),

    // External Math AST
    ProcessedMathNode(ProcessedMathNode<'a>),
    MathOpTypeNode(MathOpTypeNode),
    MathNumberNode(MathNumberNode<'a>),
    MathIdNode(MathIdNode<'a>),

    // General
    EOL,
    NullNode,
}

impl<'a> Nodes<'a> {
    pub fn get_primitive(&self) -> Option<Token<'a>> {
        match self {
            Nodes::PrimitiveTypeNode(token) => Some(token.0),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a>(pub Nodes<'a>);

impl<'a> Node<'a> {
    pub fn to_string(&self) -> String {
        format!("{:#?} \n", self)
    }
}
