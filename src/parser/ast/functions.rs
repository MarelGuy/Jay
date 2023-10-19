use super::{types::ASTTypes, Nodes};

#[derive(Debug, PartialEq, Clone)]
pub struct NodeScope<'a> {
    nodes: Vec<Nodes<'a>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NodeArg<'a> {
    name: &'a str,
    ty: ASTTypes,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeFunctionDecl<'a> {
    name: &'a str,
    ty: ASTTypes,
    scope: NodeScope<'a>,
    args: Vec<NodeArg<'a>>,
}
