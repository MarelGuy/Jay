use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MathNumberNode<'a>(pub Token<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct MathIdNode<'a> {
    id_name: String,
    id_value: Token<'a>,
}

impl<'a> MathIdNode<'a> {
    pub fn new(tok: Token<'a>) -> Self {
        Self {
            id_name: tok.slice.into(),
            id_value: tok,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MathOpTypeNode {
    op: OpType,
    prio: i8,
}

impl MathOpTypeNode {
    pub fn new(op: OpType) -> Self {
        let prio: i8;

        if op == OpType::Plus || op == OpType::Minus {
            prio = 1
        } else {
            prio = 2
        }

        Self { op, prio }
    }
}
