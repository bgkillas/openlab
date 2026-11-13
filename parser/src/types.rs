use crate::parser::NumberInner;
use number::matrix::size::{MatrixDimension, MatrixIndex};
use number::number::Number;
#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Number(Number<NumberInner>),
    List(Vec<Self>),
    Bool(bool),
    Function(u32),
    Integer(i64),
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberDimension {
    #[default]
    Complex,
    Bool,
    Function,
    Integer,
    Matrix(MatrixDimension),
    List(Box<[Self]>),
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct NumberIndexChecked {
    pub dimension: number::number::NumberDimension,
    pub index: NumberIndex,
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberIndex {
    #[default]
    Complex,
    Bool,
    Function,
    Integer,
    Matrix(MatrixIndex),
    List(usize, Box<Self>),
}
impl Type {
    pub fn number(self) -> Option<Number<NumberInner>> {
        match self {
            Self::Number(a) => Some(a),
            _ => None,
        }
    }
    pub fn bool(self) -> Option<bool> {
        match self {
            Self::Bool(a) => Some(a),
            _ => None,
        }
    }
    pub fn function(self) -> Option<u32> {
        match self {
            Self::Function(a) => Some(a),
            _ => None,
        }
    }
    pub fn integer(self) -> Option<i64> {
        match self {
            Self::Integer(a) => Some(a),
            _ => None,
        }
    }
}
