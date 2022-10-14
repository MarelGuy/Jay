use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct PrimitiveTypeNode<'a>(pub Token<'a>);
