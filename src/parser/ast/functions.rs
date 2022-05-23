use either::Either;

use super::{
    declarations::{TypeName, VarType},
    general::Node,
};

#[derive(PartialEq, Debug)]
pub struct FunctionNode<'a> {
    pub name: String,
    pub args: Vec<Box<Node<'a>>>,
    pub ret_ty: Either<VarType, TypeName>,
    pub block: Box<Node<'a>>,
}

impl<'a> FunctionNode<'a> {
    pub fn new(
        name: String,
        args: Vec<Box<Node<'a>>>,
        ret_ty: Either<VarType, TypeName>,
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
