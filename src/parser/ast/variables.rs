use core::fmt;
use std::{
    fmt::{Display, Formatter},
    isize,
};

use either::Either;

use super::Nodes;

#[derive(Debug, PartialEq, Clone)]
pub enum VarType {
    Int,
    Float,
    String,
    Char,
    Bool,
    Type { name: String },
}

impl Display for VarType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

impl Display for ArrayVarType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ArrayVarType {
    pub fn to_var_type(&self) -> VarType {
        match self {
            ArrayVarType::Int { init_num: _ } => VarType::Int,
            ArrayVarType::Float { init_num: _ } => VarType::Float,
            ArrayVarType::String { init_num: _ } => VarType::String,
            ArrayVarType::Bool { init_num: _ } => VarType::Bool,
            ArrayVarType::Char { init_num: _ } => VarType::Char,
            ArrayVarType::Type {
                name: _,
                init_num: _,
            } => todo!(),
        }
    }

    pub fn get_init_num(&self) -> &isize {
        match self {
            ArrayVarType::Int { init_num } => init_num,
            ArrayVarType::Float { init_num } => init_num,
            ArrayVarType::String { init_num } => init_num,
            ArrayVarType::Bool { init_num } => init_num,
            ArrayVarType::Char { init_num } => init_num,
            ArrayVarType::Type {
                name: _,
                init_num: _,
            } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValueNode<'a>(
    pub Either<Box<Nodes<'a>>, Vec<ArrElem<'a>>>,
    pub Either<VarType, ArrayVarType>,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VarNode<'a>(pub String, pub ValueNode<'a>, pub bool);

#[derive(Debug, PartialEq, Clone)]
pub struct ArrElem<'a>(pub Box<Nodes<'a>>, pub isize);

#[derive(Debug, PartialEq, Clone)]
pub struct CallVarNode<'a>(pub VarNode<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct CallVarArrNode<'a>(pub CallVarNode<'a>, pub isize);

#[derive(Debug, PartialEq, Clone)]
pub struct AssignToVarNode<'a>(pub CallVarNode<'a>, pub Box<Nodes<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct AssignToVarArrNode<'a>(pub CallVarArrNode<'a>, pub isize, pub Box<Nodes<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct InitTypeNode<'a> {
    fields: Vec<ValueNode<'a>>,
}

impl<'a> InitTypeNode<'a> {
    pub fn new(fields: Vec<ValueNode<'a>>) -> Self {
        Self { fields }
    }
}
