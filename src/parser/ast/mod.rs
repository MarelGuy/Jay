pub mod math;

use core::fmt::{self, Display, Formatter};

use self::math::{Int, Operation};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Nodes<'a> {
    // AST
    Int(Int<'a>),
    Op(Operation<'a>),

    // General
    Null,
}

impl<'a> Nodes<'a> {
    pub(crate) fn _get_int(self) -> Option<Int<'a>> {
        match self {
            Nodes::Int(int) => Some(int),
            _ => None,
        }
    }

    pub(crate) fn get_op(self) -> Option<Operation<'a>> {
        match self {
            Nodes::Op(op) => Some(op),
            _ => None,
        }
    }
}

impl<'a> Display for Nodes<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self)
    }
}
