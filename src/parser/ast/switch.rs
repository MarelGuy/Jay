use either::Either;

use super::general::Node;

#[derive(Debug, PartialEq)]
pub struct SwitchNode<'a> {
    pub condition: Box<Node<'a>>,
    pub cases: Vec<CaseNode<'a>>,
    pub default_block: Either<Box<Node<'a>>, ()>,
}

impl<'a> SwitchNode<'a> {
    pub fn new(
        condition: Box<Node<'a>>,
        cases: Vec<CaseNode<'a>>,
        default_block: Either<Box<Node<'a>>, ()>,
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
    pub condition: Box<Node<'a>>,
    pub block: Box<Node<'a>>,
}

impl<'a> CaseNode<'a> {
    pub fn new(condition: Box<Node<'a>>, block: Box<Node<'a>>) -> Self {
        Self { condition, block }
    }
}
