use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub struct PrimitiveTypeNode<'a>(pub Token<'a>);
