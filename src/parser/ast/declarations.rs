use super::{general::Node, identifier::IdentifierNode};

#[derive(PartialEq, Debug)]
pub struct VarType {
    name: String,
    pub is_array: bool,
}

impl VarType {
    pub fn new(name: String, is_array: bool) -> Self {
        Self { name, is_array }
    }
}

#[derive(PartialEq, Debug)]
pub enum AssignType {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    Error,
}

#[derive(PartialEq, Debug)]
pub struct VarDeclNode<'a> {
    name: String,
    ty: VarType,
    assign_op: AssignType,
    mutable: bool,
    value: Vec<Box<Node<'a>>>,
}

impl<'a> VarDeclNode<'a> {
    pub fn new(
        name: String,
        ty: VarType,
        assign_op: AssignType,
        mutable: bool,
        value: Vec<Box<Node<'a>>>,
    ) -> Self {
        Self {
            name,
            ty,
            assign_op,
            mutable,
            value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ConstDeclNode<'a> {
    name: String,
    ty: VarType,
    assign_op: AssignType,
    value: Vec<Box<Node<'a>>>,
}

impl<'a> ConstDeclNode<'a> {
    pub fn new(
        name: String,
        ty: VarType,
        assign_op: AssignType,
        value: Vec<Box<Node<'a>>>,
    ) -> Self {
        Self {
            name,
            ty,
            assign_op,
            value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct AssignNode<'a> {
    var: Box<IdentifierNode<'a>>,
    assign_token: AssignType,
    val: Box<Node<'a>>,
}

impl<'a> AssignNode<'a> {
    pub fn new(var: Box<IdentifierNode<'a>>, assign_token: AssignType, val: Box<Node<'a>>) -> Self {
        Self {
            var,
            assign_token,
            val,
        }
    }
}
