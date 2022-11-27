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

    // Utils
    EOL,
    NullNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a>(pub Nodes<'a>);
