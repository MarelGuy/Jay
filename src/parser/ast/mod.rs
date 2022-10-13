use self::{primitive_node::PrimitiveTypeNode, variables::VarNode};

use super::math::{
    ast::{MathIdNode, MathNumberNode, MathOpTypeNode},
    ProcessedMathNode,
};

pub mod primitive_node;
pub mod variables;

#[derive(Debug, PartialEq)]
pub enum Nodes<'a> {
    // AST
    PrimitiveTypeNode(PrimitiveTypeNode<'a>),
    VarNode(VarNode<'a>),

    // External Math AST
    ProcessedMathNode(ProcessedMathNode<'a>),
    MathOpTypeNode(MathOpTypeNode),
    MathNumberNode(MathNumberNode<'a>),
    MathIdNode(MathIdNode<'a>),

    // Utils
    EOL,
    NullNode,
}

#[derive(Debug, PartialEq)]
pub struct Node<'a>(pub Nodes<'a>);
