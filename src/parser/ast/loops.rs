use either::Either;

use super::general::{BlockNode, ConditionNode, Node};

#[derive(Debug, PartialEq)]
pub struct WhileNode<'a> {
    condition: Box<ConditionNode<'a>>,
    while_block: Box<BlockNode<'a>>,
}

impl<'a> WhileNode<'a> {
    pub fn new(condition: Box<ConditionNode<'a>>, while_block: Box<BlockNode<'a>>) -> Self {
        Self {
            condition,
            while_block,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ForNode<'a> {
    condition: Box<ConditionNode<'a>>,
    next: Either<Box<Node<'a>>, ()>,
    for_block: Box<BlockNode<'a>>,
}

impl<'a> ForNode<'a> {
    pub fn new(
        condition: Box<ConditionNode<'a>>,
        next: Either<Box<Node<'a>>, ()>,
        for_block: Box<BlockNode<'a>>,
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
    loop_block: Box<BlockNode<'a>>,
}

impl<'a> LoopNode<'a> {
    pub fn new(loop_block: Box<BlockNode<'a>>) -> Self {
        Self { loop_block }
    }
}
