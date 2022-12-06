use crate::lexer::token::Token;

use self::{
    primitive_node::PrimitiveTypeNode,
    variables::{AssignToVarNode, CallVarArrNode, CallVarNode, VarNode},
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

    // Variables
    VarNode(VarNode<'a>),
    CallVarNode(CallVarNode<'a>),
    CallVarArrNode(CallVarArrNode<'a>),
    AssignToVarNode(AssignToVarNode<'a>),

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

    pub fn get_call_var_node(&self) -> Option<CallVarNode<'a>> {
        match self {
            Nodes::CallVarNode(node) => Some(node.to_owned()),
            _ => None,
        }
    }

    pub fn get_call_var_arr_node(&self) -> Option<CallVarArrNode<'a>> {
        match self {
            Nodes::CallVarArrNode(node) => Some(node.to_owned()),
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
