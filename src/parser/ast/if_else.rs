use either::Either;

use super::general::{BlockNode, ConditionNode, Node};

pub struct IfNode<'a> {
    condition: Node<ConditionNode<'a>>,
    if_block: Node<BlockNode>, // Storing the block as a string for now, but will probably change to a new node.,
    else_block: Either<Node<BlockNode>, ()>, // Same thing here, but for the else block.
}

impl<'a> IfNode<'a> {
    pub fn new(
        condition: Node<ConditionNode<'a>>,
        if_block: Node<BlockNode>,
        else_block: Either<Node<BlockNode>, ()>,
    ) -> Self {
        Self {
            condition,
            if_block,
            else_block,
        }
    }
}
