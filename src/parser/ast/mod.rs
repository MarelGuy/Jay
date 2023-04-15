use core::fmt;
use std::fmt::{Display, Formatter};

use crate::lexer::token::Token;

use self::{
    functions::{CallFuncNode, FunctionNode, /*ReturnIfNode,*/ ReturnNode},
    primitive_node::{PrimitiveTypeNode, TypeNode},
    variables::{AssignToVarArrNode, AssignToVarNode, CallVarArrNode, CallVarNode, VarNode},
};

use super::math::{
    ast::{MathIdNode, MathNumberNode, MathOpTypeNode},
    ProcessedMathNode,
};

pub mod functions;
pub mod primitive_node;
pub mod variables;

#[derive(Debug, PartialEq, Clone)]
pub enum Nodes<'a> {
    // AST
    PrimitiveTypeNode(PrimitiveTypeNode<'a>),
    TypeNode(TypeNode),

    // Variables
    VarNode(VarNode<'a>),
    CallVarNode(CallVarNode<'a>),
    CallVarArrNode(CallVarArrNode<'a>),
    AssignToVarNode(AssignToVarNode<'a>),
    AssignToVarArrNode(AssignToVarArrNode<'a>),

    // Functions
    FunctionNode(FunctionNode<'a>),
    CallFuncNode(CallFuncNode<'a>),
    ReturnNode(ReturnNode<'a>),
    // ReturnIfNode(ReturnIfNode<'a>),

    // External Math AST
    ProcessedMathNode(ProcessedMathNode<'a>),
    MathOpTypeNode(MathOpTypeNode),
    MathNumberNode(MathNumberNode<'a>),
    MathIdNode(MathIdNode<'a>),

    // General
    Eol,
    NullNode,
}

impl<'a> Nodes<'a> {
    pub fn get_primitive(&self) -> Option<Token<'a>> {
        match self {
            Nodes::PrimitiveTypeNode(token) => Some(token.0),
            _ => None,
        }
    }

    pub fn get_call_var_node(&self) -> Option<CallVarNode<'a>> {
        match self {
            Nodes::CallVarNode(node) => Some(node.to_owned()),
            _ => None,
        }
    }

    pub fn get_call_var_arr_node(&self) -> Option<CallVarArrNode<'a>> {
        match self {
            Nodes::CallVarArrNode(node) => Some(node.to_owned()),
            _ => None,
        }
    }
}

impl<'a> Display for Nodes<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self)
    }
}
