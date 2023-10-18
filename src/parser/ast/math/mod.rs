use crate::lexer::token::{Token, TokenType};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Int<'a> {
    pub(crate) val: &'a str,
}

impl<'a> Int<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }

    pub fn neg_to_minus(&self) -> Vec<Token> {
        // let base: Vec<char> = self.val.chars();
        let ret_value: Vec<Token> = vec![];

        // ret_value.push(Token::new(
        //     0,
        //     0,
        //     Ok(TokenType::Minus),
        //     EitherCharOrStr::Char(self.val.chars().nth(0).unwrap()),
        //     Span { start: 0, end: 0 },
        // ));
        // ret_value.push(Token::new(
        //     0,
        //     0,
        //     Ok(TokenType::Number),
        //     EitherCharOrStr::Char(self.val.chars().nth(1).unwrap()),
        //     Span { start: 0, end: 0 },
        // ));

        ret_value
    }
}

// TODO: Add Floating point numbers

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    pub fn get_op(tok: TokenType) -> Operator {
        match tok {
            TokenType::Plus => Operator::Plus,
            TokenType::Minus => Operator::Minus,
            TokenType::Multiply => Operator::Multiply,
            TokenType::Divide => Operator::Divide,
            _ => panic!("the operator: {:?} doesn't exist!", tok),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EitherIntOrMathNode<'a> {
    MathNode(MathNode<'a>),
    Int(Int<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MathNode<'a> {
    pub(crate) lhs: Int<'a>,
    pub(crate) op: Operator,
    pub(crate) rhs: Box<EitherIntOrMathNode<'a>>,
}

impl<'a> MathNode<'a> {
    pub fn new(lhs: Int<'a>, op: Operator, rhs: EitherIntOrMathNode<'a>) -> Self {
        Self {
            lhs,
            op,
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OpNode<'a> {
    pub(crate) math_node: MathNode<'a>,
    pub(crate) priority: usize,
}

impl<'a> OpNode<'a> {
    pub fn new(priority: usize, math_node: MathNode<'a>) -> Self {
        Self {
            math_node,
            priority,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProcessedMathNode<'a> {
    pub(crate) op_node: Vec<OpNode<'a>>,
}

impl<'a> ProcessedMathNode<'a> {
    pub fn new(op_node: Vec<OpNode<'a>>) -> Self {
        Self { op_node }
    }
}
