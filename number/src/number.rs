use crate::matrix::dense::Matrix;
use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl as _;
use core::iter;
#[derive(Clone, Default, PartialEq)]
pub enum Number<C>
where
    C: ComplexImpl,
{
    #[default]
    Null,
    Complex(C),
    Matrix(Matrix<C>),
    //List(Box<Vec<Number<C>>>)
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberDimension {
    #[default]
    Null,
    Complex,
    Matrix(MatrixDimension),
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct NumberIndexChecked {
    pub dimension: NumberDimension,
    pub index: NumberIndex,
}
impl NumberIndexChecked {
    pub fn new(dimension: NumberDimension, index: NumberIndex) -> Self {
        Self { dimension, index }
    }
    pub fn complex() -> Self {
        Self::new(NumberDimension::Complex, NumberIndex::Complex)
    }
    pub fn matrix(dimension: MatrixDimension, index: MatrixIndex) -> Self {
        Self::new(
            NumberDimension::Matrix(dimension),
            NumberIndex::Matrix(index),
        )
    }
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberIndex {
    #[default]
    Null,
    Complex,
    Matrix(MatrixIndex),
}
impl<C: ComplexImpl> Number<C> {
    pub fn new(dimension: NumberDimension) -> Self {
        match dimension {
            NumberDimension::Null => Self::Null,
            NumberDimension::Complex => Self::Complex(C::default()),
            NumberDimension::Matrix(dim) => Self::Matrix(Matrix::new(dim)),
        }
    }
    pub fn size(&self) -> NumberDimension {
        match self {
            Self::Null => NumberDimension::Null,
            Self::Complex(_) => NumberDimension::Complex,
            Self::Matrix(mat) => NumberDimension::Matrix(mat.dimension),
        }
    }
    pub fn get(&self, index: NumberIndexChecked) -> Option<&C> {
        if index.dimension != self.size() {
            return None;
        }
        match (self, index.index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get(i),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, index: NumberIndexChecked) -> Option<&mut C> {
        if index.dimension != self.size() {
            return None;
        }
        match (self, index.index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get_mut(i),
            _ => None,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (NumberIndexChecked, &C)> {
        match self {
            Number::Null => NumberIter::A(iter::empty()),
            Number::Complex(c) => NumberIter::B(iter::once((NumberIndexChecked::complex(), c))),
            Number::Matrix(m) => NumberIter::C(
                m.iter_enumerate()
                    .map(|(i, c)| (NumberIndexChecked::matrix(m.dimension, i), c)),
            ),
        }
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NumberIndexChecked, &mut C)> {
        match self {
            Number::Null => NumberIter::A(iter::empty()),
            Number::Complex(c) => NumberIter::B(iter::once((NumberIndexChecked::complex(), c))),
            Number::Matrix(m) => {
                let dim = m.dimension;
                NumberIter::C(
                    m.iter_enumerate_mut()
                        .map(move |(i, c)| (NumberIndexChecked::matrix(dim, i), c)),
                )
            }
        }
    }
}
pub enum NumberIter<A, B, C> {
    A(A),
    B(B),
    C(C),
}
impl<T, A, B, C> Iterator for NumberIter<A, B, C>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
    C: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            Self::A(i) => i.next(),
            Self::B(i) => i.next(),
            Self::C(i) => i.next(),
        }
    }
}
