use either::Either;

use super::general::{BlockNode, ConditionNode};

#[derive(PartialEq, Debug)]
pub struct IfNode<'a> {
    condition: Box<ConditionNode<'a>>,
    if_block: Box<BlockNode<'a>>,
    else_block: Either<Box<BlockNode<'a>>, ()>,
}

impl<'a> IfNode<'a> {
    pub fn new(
        condition: Box<ConditionNode<'a>>,
        if_block: Box<BlockNode<'a>>,
        else_block: Either<Box<BlockNode<'a>>, ()>,
    ) -> Self {
        Self {
            condition,
            if_block,
            else_block,
        }
    }
}
