use crate::lexer::token::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Plus,
    Minus,
    Multiply,
    Divide,
    UnPlus,
    UnMinus,
}

impl From<TokenType> for OpType {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::UnMinus => OpType::UnMinus,
            TokenType::UnPlus => OpType::UnPlus,
            TokenType::Minus => OpType::Minus,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeNumber<'a>(pub Token<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct NodeBinOpType {
    pub op: OpType,
    pub prio: usize,
}

impl NodeBinOpType {
    pub fn new(op: OpType, add: usize) -> Self {
        let prio: usize = if op == OpType::Plus || op == OpType::Minus {
            1 + add
        } else {
            2 + add
        };

        Self { op, prio }
    }
}
