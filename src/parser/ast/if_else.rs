use either::Either;

use super::general::Node;

#[derive(PartialEq, Debug)]
pub struct IfNode<'a> {
    condition: Box<Node<'a>>,
    if_block: Box<Node<'a>>,
    else_block: Either<Box<Node<'a>>, ()>,
}

impl<'a> IfNode<'a> {
    pub fn new(
        condition: Box<Node<'a>>,
        if_block: Box<Node<'a>>,
        else_block: Either<Box<Node<'a>>, ()>,
    ) -> Self {
        Self {
            condition,
            if_block,
            else_block,
        }
    }
}
