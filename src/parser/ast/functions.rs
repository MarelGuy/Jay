use crate::lexer::token::TokenType;

use super::{types::ASTTypes, Nodes};

#[derive(Debug, PartialEq)]
pub struct NodeScope<'a> {
    nodes: Vec<Nodes<'a>>,
}

impl<'a> NodeScope<'a> {
    pub fn new(nodes: Vec<Nodes<'a>>) -> Self {
        Self { nodes }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NodeArg<'a> {
    name: &'a str,
    ty: ASTTypes,
}

impl<'a> NodeArg<'a> {
    pub fn new(name: &'a str, ty: TokenType) -> Self {
        Self {
            name,
            ty: ASTTypes::from(ty),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeFunctionDecl<'a> {
    name: &'a str,
    ty: ASTTypes,
    args: Vec<NodeArg<'a>>,
}

impl<'a> NodeFunctionDecl<'a> {
    pub fn new(name: &'a str, ty: TokenType, args: Vec<NodeArg<'a>>) -> Self {
        Self {
            name,
            ty: ASTTypes::from(ty),
            args,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NodeFunction<'a> {
    decl: NodeFunctionDecl<'a>,
    scope: NodeScope<'a>,
}

impl<'a> NodeFunction<'a> {
    pub fn new(decl: NodeFunctionDecl<'a>, scope: NodeScope<'a>) -> Self {
        Self { decl, scope }
    }
}

#[derive(Debug, PartialEq)]
pub struct NodeReturn<'a> {
    ty: ASTTypes,
    val: Box<Nodes<'a>>,
}

impl<'a> NodeReturn<'a> {
    pub fn new(val: Nodes<'a>) -> Self {
        Self {
            ty: ASTTypes::from(&val),
            val: Box::new(val),
        }
    }
}
