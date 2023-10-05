use crate::lexer::token::TokenType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Operation<'a> {
    pub(crate) lhs: Int<'a>,
    pub(crate) rhs: Int<'a>,
    pub(crate) op: Operator,
}

impl<'a> Operation<'a> {
    pub fn new(lhs: Int<'a>, rhs: Int<'a>, op: Operator) -> Self {
        Self { lhs, rhs, op }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Int<'a> {
    pub(crate) val: &'a str,
}

impl<'a> Int<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

// TODO: Add Floating point numbers

// TODO: Add Mul and Div parsing
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

impl Operator {
    pub fn get_op(tok: TokenType) -> Operator {
        match tok {
            TokenType::Plus => Operator::Plus,
            TokenType::Minus => Operator::Minus,
            TokenType::Multiply => Operator::Multiply,
            TokenType::Divide => Operator::Divide,
            TokenType::Modulo => Operator::Modulo,
            _ => panic!("This operator doesn't exist!"),
        }
    }
}
