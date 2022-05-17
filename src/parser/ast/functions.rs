use super::{declarations::VarType, general::Node};

#[derive(PartialEq, Debug)]
pub struct FunctionNode<'a> {
    pub name: String,
    pub args: Vec<Box<Node<'a>>>,
    pub ret_ty: VarType,
    pub block: Box<Node<'a>>,
}

impl<'a> FunctionNode<'a> {
    pub fn new(
        name: String,
        args: Vec<Box<Node<'a>>>,
        ret_ty: VarType,
        block: Box<Node<'a>>,
    ) -> Self {
        Self {
            name,
            args,
            ret_ty,
            block,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ParamNode {
    pub name: String,
    pub ty: VarType,
}

impl ParamNode {
    pub fn new(name: String, ty: VarType) -> Self {
        Self { name, ty }
    }
}
