use self::primitive_nodes::{IdentifierNode, NumberNode};

pub mod primitive_nodes;

#[derive(Debug, PartialEq)]
pub enum Nodes<'a> {
    NumberNode(NumberNode<'a>),
    IdentifierNode(IdentifierNode<'a>),

    NullNode,
}

#[derive(Debug, PartialEq)]
pub struct Node<'a> {
    pub node: Nodes<'a>,
}

impl<'a> Node<'a> {
    pub fn new(node: Nodes<'a>) -> Self {
        Self { node }
    }
}
