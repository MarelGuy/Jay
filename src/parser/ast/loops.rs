use either::Either;

use super::{math_ops::UnOpNode, BlockNode, ConditionNode};

#[derive(Debug, PartialEq)]
pub struct WhileNode<'a> {
    condition: ConditionNode<'a>,
    while_block: BlockNode<'a>,
}

impl<'a> WhileNode<'a> {
    pub fn new(condition: ConditionNode<'a>, while_block: BlockNode<'a>) -> Self {
        Self {
            condition,
            while_block,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ForNode<'a> {
    condition: ConditionNode<'a>,
    next: Either<UnOpNode<'a>, ()>,
    for_block: BlockNode<'a>,
}

impl<'a> ForNode<'a> {
    pub fn new(
        condition: ConditionNode<'a>,
        next: Either<UnOpNode<'a>, ()>,
        for_block: BlockNode<'a>,
    ) -> Self {
        Self {
            condition,
            next,
            for_block,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LoopNode<'a> {
    loop_block: BlockNode<'a>,
}

impl<'a> LoopNode<'a> {
    pub fn new(loop_block: BlockNode<'a>) -> Self {
        Self { loop_block }
    }
}
