use either::Either;

use super::{BlockNode, ConditionNode};

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchNode<'a> {
    pub cases: Vec<CaseNode<'a>>,
    pub default_block: Either<DefaultNode<'a>, ()>,
}

impl<'a> SwitchNode<'a> {
    pub fn new(cases: Vec<CaseNode<'a>>, default_block: Either<DefaultNode<'a>, ()>) -> Self {
        Self {
            cases,
            default_block,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseNode<'a> {
    pub condition: ConditionNode<'a>,
    pub block: BlockNode<'a>,
}

impl<'a> CaseNode<'a> {
    pub fn new(condition: ConditionNode<'a>, block: BlockNode<'a>) -> Self {
        Self { condition, block }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultNode<'a> {
    pub block: BlockNode<'a>,
}

impl<'a> DefaultNode<'a> {
    pub fn new(block: BlockNode<'a>) -> Self {
        Self { block }
    }
}
