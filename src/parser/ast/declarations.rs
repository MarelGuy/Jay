use super::general::{Node, ParamNode};

#[derive(PartialEq, Debug)]
pub struct TypeNode<'a> {
    name: String,
    fields: Vec<Box<Node<'a>>>,
}

impl<'a> TypeNode<'a> {
    pub fn new(name: String, fields: Vec<Box<Node<'a>>>) -> Self {
        Self { name, fields }
    }
}

#[derive(PartialEq, Debug)]
pub enum VarType {
    Int,
    Float,
    Bool,
    String,
    Char,
    Void,
    Type,
    Error,
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
pub struct VarDeclNode {
    name: String,
    ty: VarType,
    assign_op: AssignType,
    mutable: bool,
    value: String, // Values are stored in string just for debugging purposes, i'll add a prover value handling when the parser is done
}

impl<'a> VarDeclNode {
    pub fn new(
        name: String,
        ty: VarType,
        assign_op: AssignType,
        mutable: bool,
        value: String,
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
pub struct ConstDeclNode {
    name: String,
    ty: VarType,
    assign_op: AssignType,
    value: String, // Values are stored in string just for debugging purposes, i'll add a prover value handling when the parser is done
}

impl<'a> ConstDeclNode {
    pub fn new(name: String, ty: VarType, assign_op: AssignType, value: String) -> Self {
        Self {
            name,
            ty,
            assign_op,
            value,
        }
    }
}
