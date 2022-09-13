pub(crate) mod declarations;
pub(crate) mod functions;
pub(crate) mod identifier;
pub(crate) mod if_else;
pub(crate) mod import_export;
pub(crate) mod loops;
pub(crate) mod math_ops;
pub(crate) mod switch;
pub(crate) mod types;

use core::fmt::{self, Display};

use either::Either;

use crate::lexer::token::Token;

use self::{
    declarations::{ArrNode, AssignNode, ConstDeclNode, VarDeclNode, VarType},
    functions::{FunctionDeclNode, FunctionNode, ReturnIfNode, ReturnNode, UseFunctionNode},
    identifier::{ArrayAccessNode, DotNotationNode, IdentifierNode},
    if_else::IfNode,
    import_export::{ExportNode, ImportNode},
    loops::{ForNode, LoopNode, WhileNode},
    math_ops::OpNode,
    switch::SwitchNode,
    types::{BoolNode, CharNode, FloatNode, NewTypeValueNode, NumberNode, StringNode, TypeNode},
};

#[derive(PartialEq, Debug)]
pub struct ConditionNode<'a> {
    pub left_node: Node<'a>,
    op_token: Token<'a>,
    pub right_node: Node<'a>,
}

impl<'a> ConditionNode<'a> {
    pub fn new(left_node: Node<'a>, op_token: Token<'a>, right_node: Node<'a>) -> Self {
        Self {
            left_node,
            op_token,
            right_node,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct BlockNode<'a> {
    pub block: Vec<Node<'a>>,
}

impl<'a> BlockNode<'a> {
    pub fn new(block: Vec<Node<'a>>) -> Self {
        Self { block }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParamNode {
    pub name: String,
    pub ty: Either<FunctionDeclNode, VarType>,
}

impl ParamNode {
    pub fn new(name: String, ty: Either<FunctionDeclNode, VarType>) -> Self {
        Self { name, ty }
    }
}

impl Display for ParamNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(PartialEq, Debug)]
pub enum Nodes<'a> {
    // Declarations
    VarDeclNode(VarDeclNode<'a>),
    ConstDeclNode(ConstDeclNode<'a>),
    AssignNode(AssignNode<'a>),

    // Identifiers
    IdentifierNode(IdentifierNode<'a>),
    ArrayAccessNode(ArrayAccessNode<'a>),
    DotNotationNode(DotNotationNode<'a>),
    NewTypeValueNode(NewTypeValueNode<'a>),
    ArrayNode(ArrNode<'a>),

    // If-else
    IfNode(IfNode<'a>),
    SwitchNode(SwitchNode<'a>),

    // Ops
    OpNode(OpNode<'a>),

    // Types
    NumberNode(NumberNode<'a>),
    FloatNode(FloatNode<'a>),
    StringNode(StringNode<'a>),
    CharNode(CharNode<'a>),
    BoolNode(BoolNode<'a>),
    TypeNode(TypeNode),

    // Loops
    WhileNode(WhileNode<'a>),
    ForNode(ForNode<'a>),
    LoopNode(LoopNode<'a>),

    // Functions
    FunctionNode(FunctionNode<'a>),
    UseFunctionNode(UseFunctionNode<'a>),
    ReturnNode(ReturnNode<'a>),
    ReturnIfNode(ReturnIfNode<'a>),

    // Import & Export
    ImportNode(ImportNode<'a>),
    ExportNode(ExportNode<'a>),

    // Misc
    NullNode,
}

#[derive(PartialEq, Debug)]
pub struct Node<'a> {
    pub node: Nodes<'a>,
}

impl<'a> Node<'a> {
    pub fn new(node: Nodes<'a>) -> Self {
        Self { node }
    }
}
