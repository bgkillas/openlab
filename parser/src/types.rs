use crate::parser::Number;
use number::matrix::dense::Matrix;
use number::matrix::size::{MatrixDimension, MatrixIndex};
#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Number(Number),
    Matrix(Matrix<Number>),
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
    pub fn number(self) -> Option<Number> {
        match self {
            Self::Number(a) => Some(a),
            _ => None,
        }
    }
    pub fn matrix(self) -> Option<Matrix<Number>> {
        match self {
            Self::Matrix(a) => Some(a),
            _ => None,
        }
    }
    pub fn list(self) -> Option<Vec<Self>> {
        match self {
            Self::List(a) => Some(a),
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
