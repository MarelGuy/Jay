pub mod math;
use core::fmt::{self, Display, Formatter};

use self::math::{
    ast::{BinOpTypeNode, NumberNode},
    ProcessedBinOpNode,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Nodes<'a> {
    // AST
    ProcessedBinOpNode(ProcessedBinOpNode<'a>),
    NumberNode(NumberNode<'a>),
    BinOpTypeNode(BinOpTypeNode),

    // General
    NextLine,
    Null,
}

impl Display for Nodes<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self)
    }
}
