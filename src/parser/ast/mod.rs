pub mod functions;
pub mod math;
pub mod types;

use core::fmt::{self, Display, Formatter};

use self::{
    functions::{NodeFunction, NodeReturn},
    math::{
        ast::{NodeBinOpType, NodeNumber},
        NodeProcessedBinOp, NodeProcessedUnOp,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum Nodes<'a> {
    // AST
    NodeProcessedBinOp(NodeProcessedBinOp<'a>),
    NodeProcessedUnOp(NodeProcessedUnOp<'a>),

    // Math
    NodeNumber(NodeNumber<'a>),
    NodeBinOpType(NodeBinOpType),

    // Functions
    NodeFunction(NodeFunction<'a>),
    NodeReturn(NodeReturn<'a>),

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
