use either::Either;

use super::{
    general::{BlockNode, ConditionNode},
    identifier::IdentifierNode,
};

#[derive(Debug, PartialEq)]
pub struct SwitchNode<'a> {
    pub condition: Box<IdentifierNode<'a>>,
    pub cases: Vec<Box<CaseNode<'a>>>,
    pub default_block: Either<Box<DefaultNode<'a>>, ()>,
}

impl<'a> SwitchNode<'a> {
    pub fn new(
        condition: Box<IdentifierNode<'a>>,
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
    pub block: Box<BlockNode<'a>>,
}

impl<'a> DefaultNode<'a> {
    pub fn new(block: Box<BlockNode<'a>>) -> Self {
        Self { block }
    }
}
