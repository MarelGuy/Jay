use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeNumber<'a>(pub Token<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct NodeBinOpType {
    pub op: OpType,
    pub prio: usize,
}

impl NodeBinOpType {
    pub fn new(op: OpType) -> Self {
        let prio: usize = if op == OpType::Plus || op == OpType::Minus {
            1
        } else {
            2
        };

        Self { op, prio }
    }
}
