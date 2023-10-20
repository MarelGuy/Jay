use crate::lexer::token::TokenType;

use super::Nodes;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ASTTypes {
    Int,
    Float,
    Uint,
    Char,
    Str,
    String,
    Bool,
    Void,
}

impl From<TokenType> for ASTTypes {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::TypeBool => ASTTypes::Bool,
            TokenType::TypeChar => ASTTypes::Char,
            TokenType::TypeInt => ASTTypes::Int,
            TokenType::TypeUint => ASTTypes::Uint,
            TokenType::TypeFloat => ASTTypes::Float,
            TokenType::TypeString => ASTTypes::String,
            TokenType::TypeStr => ASTTypes::Str,
            TokenType::TypeVoid => ASTTypes::Void,
            _ => panic!(),
        }
    }
}

impl<'a> From<&Nodes<'a>> for ASTTypes {
    fn from(node: &Nodes<'a>) -> Self {
        match node {
            Nodes::NodeNumber(node) => {
                if node.0.token_type == TokenType::Number {
                    ASTTypes::Int
                } else {
                    ASTTypes::Float
                }
            }
            Nodes::NullValue => ASTTypes::Void,
            _ => panic!(),
        }
    }
}
