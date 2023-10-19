pub mod functions;
pub mod math;
pub mod types;

use core::fmt::{self, Display, Formatter};

use self::{
    functions::NodeFunctionDecl,
    math::{
        ast::{NodeBinOpType, NodeNumber},
        NodeProcessedBinOp,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum Nodes<'a> {
    // AST
    ProcessedBinOpNode(NodeProcessedBinOp<'a>),
    NodeNumber(NodeNumber<'a>),
    NodeBinOpType(NodeBinOpType),
    NodeFunctionDecl(NodeFunctionDecl<'a>),

    // General
    NextLine,
    Null,
}

impl Display for Nodes<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self)
    }
}
