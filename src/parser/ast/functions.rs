use either::Either;

use super::{
    declarations::{TypeName, VarType},
    general::{BlockNode, ParamNode},
};

#[derive(PartialEq, Debug)]
pub struct FunctionNode<'a> {
    pub name: String,
    pub args: Vec<Box<ParamNode>>,
    pub ret_ty: Either<VarType, TypeName>,
    pub block: Box<BlockNode<'a>>,
}

impl<'a> FunctionNode<'a> {
    pub fn new(
        name: String,
        args: Vec<Box<ParamNode>>,
        ret_ty: Either<VarType, TypeName>,
        block: Box<BlockNode<'a>>,
    ) -> Self {
        Self {
            name,
            args,
            ret_ty,
            block,
        }
    }
}
