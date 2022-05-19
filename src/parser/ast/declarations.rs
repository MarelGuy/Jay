use super::general::ParamNode;

#[derive(PartialEq, Debug)]
pub struct TypeNode {
    name: String,
    fields: Vec<ParamNode>,
}

impl TypeNode {
    pub fn new(name: String, fields: Vec<ParamNode>) -> Self {
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
    Type(TypeNode),
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

impl VarDeclNode {
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

impl ConstDeclNode {
    pub fn new(name: String, ty: VarType, assign_op: AssignType, value: String) -> Self {
        Self {
            name,
            ty,
            assign_op,
            value,
        }
    }
}
