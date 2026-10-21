use crate::matrix::dense::Matrix;
use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::assign::NegAssign;
use crate::traits::matrix::MatrixImpl as _;
use crate::traits::number::NumberImpl;
use crate::traits::pow::{Pow, Root};
use core::iter;
use core::ops::Neg;
use core::ops::{Add, Div, Mul, Sub};
use std::debug_assert_matches;
#[derive(Clone, PartialEq, Debug)]
pub enum Number<C>
where
    C: NumberImpl,
{
    Complex(C),
    Matrix(Matrix<C>),
    List(Vec<Self>),
}
impl<C: NumberImpl> Default for Number<C> {
    fn default() -> Self {
        Self::Complex(C::default())
    }
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberDimension {
    #[default]
    Complex,
    Matrix(MatrixDimension),
    List(Box<[Self]>),
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
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
    pub fn list(dimension: NumberDimension, i: usize, index: NumberIndex) -> Self {
        debug_assert_matches!(dimension, NumberDimension::List(_));
        Self::new(dimension, NumberIndex::List(i, Box::new(index)))
    }
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberIndex {
    #[default]
    Complex,
    Matrix(MatrixIndex),
    List(usize, Box<Self>),
}
macro_rules! optional_operator {
    ($tr:ident, $f:ident, $fa:ident) => {
        impl<C: NumberImpl> $tr<Self> for Number<C> {
            type Output = Option<Self>;
            fn $f(mut self, rhs: Self) -> Self::Output {
                self.$fa(&rhs)?;
                Some(self)
            }
        }
        impl<C: NumberImpl> $tr<&Self> for Number<C> {
            type Output = Option<Self>;
            fn $f(mut self, rhs: &Self) -> Self::Output {
                self.$fa(rhs)?;
                Some(self)
            }
        }
    };
}
optional_operator!(Add, add, add_assign);
optional_operator!(Sub, sub, sub_assign);
optional_operator!(Mul, mul, mul_assign);
optional_operator!(Div, div, div_assign);
optional_operator!(Pow, pow, pow_assign);
optional_operator!(Root, root, root_assign);
impl<C: NumberImpl> Neg for Number<C> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.neg_assign();
        self
    }
}
impl<C: NumberImpl> NegAssign for Number<C> {
    fn neg_assign(&mut self) {
        match self {
            Self::Complex(c) => c.neg_assign(),
            Self::Matrix(m) => m.neg_assign(),
            Self::List(l) => {
                for e in l {
                    e.neg_assign();
                }
            }
        }
    }
}
impl<C: NumberImpl> Number<C> {
    pub fn complex(&self) -> Option<C> {
        if let Self::Complex(c) = self {
            Some(*c)
        } else {
            None
        }
    }
    pub fn matrix(&self) -> Option<&Matrix<C>> {
        if let Self::Matrix(m) = self {
            Some(m)
        } else {
            None
        }
    }
    pub fn matrix_owned(self) -> Option<Matrix<C>> {
        if let Self::Matrix(m) = self {
            Some(m)
        } else {
            None
        }
    }
    pub fn matrix_mut(&mut self) -> Option<&mut Matrix<C>> {
        if let Self::Matrix(m) = self {
            Some(m)
        } else {
            None
        }
    }
    pub fn component_wise<F>(mut self, rhs: &Self, f: F) -> Option<Self>
    where
        F: FnMut(C, C) -> Option<C>,
    {
        self.component_wise_mut(rhs, f)?;
        Some(self)
    }
    pub fn component_wise_mut<F>(&mut self, rhs: &Self, mut f: F) -> Option<()>
    where
        F: FnMut(C, C) -> Option<C>,
    {
        match (self.size(), rhs.size()) {
            (sizea, sizeb) if sizea == sizeb => {
                for (a, b) in self
                    .iter_mut()
                    .zip(rhs.iter())
                    .map(|((_, a), (_, b))| (a, b))
                {
                    *a = f(*a, b)?;
                }
            }
            (_, NumberDimension::Complex) => {
                let b = rhs.complex()?;
                for (_, a) in self.iter_mut() {
                    *a = f(*a, b)?;
                }
            }
            (NumberDimension::Matrix(ma), NumberDimension::Matrix(mb))
                if ma.width == mb.width && mb.height == 1 =>
            {
                let vec = rhs.matrix()?.entries();
                for row in self.matrix_mut()?.rows_mut() {
                    for (a, &b) in row.iter_mut().zip(vec) {
                        *a = f(*a, b)?;
                    }
                }
            }
            (NumberDimension::Matrix(ma), NumberDimension::Matrix(mb))
                if ma.height == mb.height && mb.width == 1 =>
            {
                let vec = rhs.matrix()?.entries();
                for (row, &b) in self.matrix_mut()?.rows_mut().zip(vec) {
                    for a in row.iter_mut() {
                        *a = f(*a, b)?;
                    }
                }
            }
            _ => return None,
        }
        Some(())
    }
    pub fn add_assign(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => a.add_assign(b)?,
            (Self::Matrix(a), Self::Matrix(b)) => a.add_assign(b)?,
            (Self::List(_), Self::List(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn sub_assign(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => a.sub_assign(b)?,
            (Self::Matrix(a), Self::Matrix(b)) => a.sub_assign(b)?,
            (Self::List(_), Self::List(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn mul_assign(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => a.mul_assign(b)?,
            (Self::Matrix(a), Self::Matrix(b)) => a.mul_assign(b)?,
            (Self::List(_), Self::List(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn div_assign(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => a.div_assign(b)?,
            (Self::Matrix(_), Self::Matrix(_)) | (Self::List(_), Self::List(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn pow_assign(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => a.pow_assign(b)?,
            (Self::Matrix(_), Self::Matrix(_)) | (Self::List(_), Self::List(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn root_assign(&mut self, rhs: &Self) -> Option<()> {
        if self.size() != rhs.size() {
            return None;
        }
        match (self, rhs) {
            (Self::Complex(a), &Self::Complex(b)) => a.root_assign(b)?,
            (Self::Matrix(_), Self::Matrix(_)) | (Self::List(_), Self::List(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn new(dimension: &NumberDimension) -> Self {
        match dimension {
            &NumberDimension::Complex => Self::Complex(C::default()),
            &NumberDimension::Matrix(dim) => Self::Matrix(Matrix::new(dim)),
            NumberDimension::List(dim) => Self::List(dim.iter().map(|l| Self::new(l)).collect()),
        }
    }
    pub fn size(&self) -> NumberDimension {
        match self {
            Self::Complex(_) => NumberDimension::Complex,
            Self::Matrix(mat) => NumberDimension::Matrix(mat.dimension),
            Self::List(l) => NumberDimension::List(l.iter().map(Self::size).collect()),
        }
    }
    pub fn get(&self, index: NumberIndexChecked) -> Option<C> {
        if index.dimension != self.size() {
            return None;
        }
        match (self, index.index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(*val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get(i),
            (Self::List(l), NumberIndex::List(i, idx)) => l[i].get_unchecked(*idx),
            _ => unreachable!(),
        }
    }
    pub fn get_unchecked(&self, index: NumberIndex) -> Option<C> {
        match (self, index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(*val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get(i),
            (Self::List(l), NumberIndex::List(i, idx)) => l[i].get_unchecked(*idx),
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
            (Self::List(l), NumberIndex::List(i, idx)) => l[i].get_unchecked_mut(*idx),
            _ => unreachable!(),
        }
    }
    pub fn get_unchecked_mut(&mut self, index: NumberIndex) -> Option<&mut C> {
        match (self, index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get_mut(i),
            (Self::List(l), NumberIndex::List(i, idx)) => l[i].get_unchecked_mut(*idx),
            _ => unreachable!(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (NumberIndexChecked, C)> {
        match self {
            &Self::Complex(c) => NumberIter::A(iter::once((NumberIndexChecked::complex(), c))),
            Self::Matrix(m) => NumberIter::B(
                m.iter()
                    .map(|(i, c)| (NumberIndexChecked::matrix(m.dimension, i), c)),
            ),
            Self::List(l) => {
                let dim = self.size();
                NumberIter::C(l.iter().enumerate().flat_map(move |(i, elem)| {
                    let value = dim.clone();
                    Box::<dyn Iterator<Item = (NumberIndexChecked, C)>>::from(Box::new(elem.iter()))
                        .map(move |(a, b)| (NumberIndexChecked::list(value.clone(), i, a.index), b))
                }))
            }
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
            Self::List(l) => {
                let dim = NumberDimension::List(l.iter().map(Self::size).collect());
                NumberIter::C(l.iter_mut().enumerate().flat_map(move |(i, elem)| {
                    let value = dim.clone();
                    Box::<dyn Iterator<Item = (NumberIndexChecked, &mut C)>>::from(Box::new(
                        elem.iter_mut(),
                    ))
                    .map(move |(a, b)| (NumberIndexChecked::list(value.clone(), i, a.index), b))
                }))
            }
        }
    }
    pub fn transpose(self) -> Option<Self> {
        match self {
            c @ Self::Complex(_) => Some(c),
            Self::Matrix(mut m) => {
                m.transpose();
                Some(Self::Matrix(m))
            }
            Self::List(_) => None,
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
