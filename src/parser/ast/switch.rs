use either::Either;

use super::general::{BlockNode, ConditionNode, Node};

#[derive(Debug, PartialEq)]
pub struct SwitchNode<'a> {
    pub condition: Box<Node<'a>>,
    pub cases: Vec<Box<CaseNode<'a>>>,
    pub default_block: Either<Box<DefaultNode<'a>>, ()>,
}

impl<'a> SwitchNode<'a> {
    pub fn new(
        condition: Box<Node<'a>>,
        cases: Vec<Box<CaseNode<'a>>>,
        default_block: Either<Box<DefaultNode<'a>>, ()>,
    ) -> Self {
        Self {
            condition,
            cases,
            default_block,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CaseNode<'a> {
    pub condition: Box<ConditionNode<'a>>,
    pub block: Box<BlockNode<'a>>,
}

impl<'a> CaseNode<'a> {
    pub fn new(condition: Box<ConditionNode<'a>>, block: Box<BlockNode<'a>>) -> Self {
        Self { condition, block }
    }
}

#[derive(Debug, PartialEq)]
pub struct DefaultNode<'a> {
    pub block: Box<Node<'a>>,
}

impl<'a> DefaultNode<'a> {
    pub fn new(block: Box<Node<'a>>) -> Self {
        Self { block }
    }
}
