use either::Either;

use super::general::Node;

#[derive(PartialEq, Debug)]
pub struct TypeName {
    name: String,
}

impl TypeName {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
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
    Type(TypeName),
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
pub struct VarDeclNode<'a> {
    name: String,
    ty: Either<VarType, TypeName>,
    assign_op: AssignType,
    mutable: bool,
    value: Vec<Box<Node<'a>>>,
}

impl<'a> VarDeclNode<'a> {
    pub fn new(
        name: String,
        ty: Either<VarType, TypeName>,
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
    ty: Either<VarType, TypeName>,
    assign_op: AssignType,
    value: Vec<Box<Node<'a>>>,
}

impl<'a> ConstDeclNode<'a> {
    pub fn new(
        name: String,
        ty: Either<VarType, TypeName>,
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
