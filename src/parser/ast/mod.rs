pub mod functions;
pub mod math;
pub mod types;

use core::fmt;
use std::fmt::{Display, Formatter};

use self::{
    functions::{NodeFunction, NodeReturn},
    math::{
        ast::{NodeBinOpType, NodeNumber},
        NodeProcessedBinOp, NodeProcessedUnOp,
    },
};

#[derive(Debug, PartialEq)]
pub enum Nodes<'a> {
    // AST
    NodeProcessedBinOp(&'a mut NodeProcessedBinOp<'a>),
    NodeProcessedUnOp(&'a mut NodeProcessedUnOp<'a>),

    // Math
    NodeNumber(&'a mut NodeNumber<'a>),
    NodeBinOpType(&'a mut NodeBinOpType),

    // Functions
    NodeFunction(&'a mut NodeFunction<'a>),
    NodeReturn(&'a mut NodeReturn<'a>),

    // General
    NextLine,
    NullValue,
    Null,
}

impl Display for Nodes<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self)
    }
}
