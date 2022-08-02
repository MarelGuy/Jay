use super::general::Node;

#[derive(PartialEq, Debug)]
pub struct VarType {
    name: String,
}

impl VarType {
    pub fn new(name: String) -> Self {
        Self { name }
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
