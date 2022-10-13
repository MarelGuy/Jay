use super::Node;

#[derive(Debug, PartialEq)]
pub struct VarNode<'a>(String, Box<Node<'a>>, bool);
