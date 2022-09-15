use either::Either;

use super::{BlockNode, ConditionNode};

#[derive(PartialEq, Debug, Clone)]
pub struct IfNode<'a> {
    condition: ConditionNode<'a>,
    if_block: BlockNode<'a>,
    else_block: Either<BlockNode<'a>, ()>,
}

impl<'a> IfNode<'a> {
    pub fn new(
        condition: ConditionNode<'a>,
        if_block: BlockNode<'a>,
        else_block: Either<BlockNode<'a>, ()>,
    ) -> Self {
        Self {
            condition,
            if_block,
            else_block,
        }
    }
}
