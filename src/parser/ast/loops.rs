use either::Either;

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

#[derive(PartialEq, Debug)]
pub struct ForNode<'a> {
    condition: Box<Node<'a>>,
    next: Either<Box<Node<'a>>, ()>,
    for_block: Box<Node<'a>>,
}

impl<'a> ForNode<'a> {
    pub fn new(
        condition: Box<Node<'a>>,
        next: Either<Box<Node<'a>>, ()>,
        for_block: Box<Node<'a>>,
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
    loop_block: Box<Node<'a>>,
}

impl<'a> LoopNode<'a> {
    pub fn new(loop_block: Box<Node<'a>>) -> Self {
        Self { loop_block }
    }
}
