use either::Either;

use super::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
    Int,
    Float,
    String,
    Char,
    Bool,
    Type { name: String },
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrayVarType {
    Int { init_num: isize },
    Float { init_num: isize },
    String { init_num: isize },
    Char { init_num: isize },
    Bool { init_num: isize },
    Type { name: String, init_num: isize },
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarNode<'a> {
    pub name: String,
    pub ty: Either<VarType, ArrayVarType>,
    pub val: Vec<Node<'a>>,
    pub is_mut: bool,
}

impl<'a> VarNode<'a> {
    pub fn new(
        name: String,
        ty: Either<VarType, ArrayVarType>,
        val: Vec<Node<'a>>,
        is_mut: bool,
    ) -> Self {
        Self {
            name,
            ty,
            val,
            is_mut,
        }
    }
}
