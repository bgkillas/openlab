use crate::matrix::dense::Matrix;
use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl as _;
use core::iter;
#[derive(Clone, PartialEq, Debug)]
pub enum Number<C>
where
    C: ComplexImpl,
{
    Complex(C),
    Matrix(Matrix<C>),
    //List(Box<Vec<Number<C>>>)
}
impl<C: ComplexImpl> Default for Number<C> {
    fn default() -> Self {
        Self::Complex(C::default())
    }
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberDimension {
    #[default]
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
    Complex,
    Matrix(MatrixIndex),
}
impl<C: ComplexImpl> Number<C> {
    pub fn add(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => *a += b,
            (Self::Matrix(a), Self::Matrix(b)) => a.add(b)?,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn sub(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => *a -= b,
            (Self::Matrix(a), Self::Matrix(b)) => a.sub(b)?,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn mul(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => *a *= b,
            (Self::Matrix(a), Self::Matrix(b)) => a.mul(b)?,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn div(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => *a += b,
            (Self::Matrix(_), Self::Matrix(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn new(dimension: NumberDimension) -> Self {
        match dimension {
            NumberDimension::Complex => Self::Complex(C::default()),
            NumberDimension::Matrix(dim) => Self::Matrix(Matrix::new(dim)),
        }
    }
    pub fn size(&self) -> NumberDimension {
        match self {
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
            _ => unreachable!(),
        }
    }
    pub fn get_mut(&mut self, index: NumberIndexChecked) -> Option<&mut C> {
        if index.dimension != self.size() {
            return None;
        }
        match (self, index.index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get_mut(i),
            _ => unreachable!(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (NumberIndexChecked, &C)> {
        match self {
            Self::Complex(c) => NumberIter::A(iter::once((NumberIndexChecked::complex(), c))),
            Self::Matrix(m) => NumberIter::B(
                m.iter()
                    .map(|(i, c)| (NumberIndexChecked::matrix(m.dimension, i), c)),
            ),
        }
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NumberIndexChecked, &mut C)> {
        match self {
            Self::Complex(c) => NumberIter::A(iter::once((NumberIndexChecked::complex(), c))),
            Self::Matrix(m) => {
                let dim = m.dimension;
                NumberIter::B(
                    m.iter_mut()
                        .map(move |(i, c)| (NumberIndexChecked::matrix(dim, i), c)),
                )
            }
        }
    }
}
pub enum NumberIter<A, B> {
    A(A),
    B(B),
}
impl<T, A, B> Iterator for NumberIter<A, B>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            Self::A(i) => i.next(),
            Self::B(i) => i.next(),
        }
    }
}
