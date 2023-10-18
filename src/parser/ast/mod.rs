pub mod math;
use core::fmt::{self, Display, Formatter};

use self::math::{Int, ProcessedMathNode};

#[derive(Debug, PartialEq, Clone)]
pub enum Nodes<'a> {
    // AST
    Int(Int<'a>),
    ProcessedMathNode(ProcessedMathNode<'a>),

    // General
    NextLine,
    Null,
}

impl<'a> Nodes<'a> {
    // pub(crate) fn get_int(self) -> Option<Int<'a>> {
    //     match self {
    //         Nodes::Int(int) => Some(int),
    //         _ => None,
    //     }
    // }

    // pub(crate) fn get_math_node(&self) -> Option<MathNode<'a>> {
    //     match self {
    //         Nodes::MathNode(op) => Some(op.clone()),
    //         _ => None,
    //     }
    // }
}

impl<'a> Display for Nodes<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self)
    }
}
