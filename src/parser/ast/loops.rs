use super::general::Node;

#[derive(Debug, PartialEq)]
pub struct WhileNode<'a> {
    condition: Box<Node<'a>>,
    while_block: Box<Node<'a>>,
}

impl<'a> WhileNode<'a> {
    pub fn new(condition: Box<Node<'a>>, while_block: Box<Node<'a>>) -> Self {
        Self {
            condition,
            while_block,
        }
    }
}
