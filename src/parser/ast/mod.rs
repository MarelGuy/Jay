use self::primitive_node::PrimitiveTypeNode;

pub mod primitive_node;

#[derive(Debug, PartialEq)]
pub enum Nodes<'a> {
    PrimitiveTypeNode(PrimitiveTypeNode<'a>),

    // Utils
    EOL,
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
