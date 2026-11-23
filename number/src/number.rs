use crate::float::complex::Complex;
use crate::matrix::dense::Matrix;
use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::rational::impls::Rational;
use crate::traits::assign::NegAssign;
use crate::traits::matrix::MatrixImpl as _;
use crate::traits::number::NumberImpl as _;
use crate::traits::pow::{Pow, Root};
use crate::units::NumberUnits;
use core::iter;
use core::ops::Neg;
use core::ops::{Add, Div, Mul, Sub};
use malachite::float::BigExponentFloat;
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ComplexType {
    Arbitrary,
    F128,
    #[default]
    F64,
    F32,
}
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum RationalType {
    Arbitrary,
    U128,
    I128,
    U64,
    #[default]
    I64,
    U32,
    I32,
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberType {
    Complex(ComplexType),
    Rational(RationalType),
}
#[derive(Clone, Debug)]
pub enum NumberElement {
    Float(FloatElement),
    Rational(RationalElement),
}
#[derive(Clone, Debug)]
pub enum FloatElement {
    Arbitrary(BigExponentFloat),
    F128(!),
    F64(NumberUnits<Complex<f64>>),
    F32(NumberUnits<Complex<f32>>),
}
#[derive(Clone, Debug)]
pub enum RationalElement {
    Arbitrary(malachite::Rational),
    U128(Rational<u128>),
    I128(Rational<i128>),
    U64(Rational<u64>),
    I64(Rational<i64>),
    U32(Rational<u32>),
    I32(Rational<i32>),
}
#[derive(Clone, Debug)]
pub enum FloatElementRef<'a> {
    Arbitrary(&'a BigExponentFloat),
    F128(!),
    F64(NumberUnits<Complex<f64>>),
    F32(NumberUnits<Complex<f32>>),
}
#[derive(Clone, Debug)]
pub enum RationalElementRef<'a> {
    Arbitrary(&'a malachite::Rational),
    U128(Rational<u128>),
    I128(Rational<i128>),
    U64(Rational<u64>),
    I64(Rational<i64>),
    U32(Rational<u32>),
    I32(Rational<i32>),
}
#[derive(Clone, PartialEq, Debug)]
pub enum Number {
    Complex(NumberUnits<Complex>),
    Matrix(Matrix<NumberUnits<Complex>>),
}
#[derive(Clone, PartialEq, Debug)]
pub enum NumberRef {
    Complex(NumberUnits<Complex>),
    Matrix(Matrix<NumberUnits<Complex>>),
}
impl Default for Number {
    fn default() -> Self {
        Self::Complex(NumberUnits::default())
    }
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberDimension {
    #[default]
    Complex,
    Matrix(MatrixDimension),
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
}
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberIndex {
    #[default]
    Complex,
    Matrix(MatrixIndex),
}
macro_rules! optional_operator {
    ($tr:ident, $f:ident, $fa:ident) => {
        impl $tr<Self> for Number {
            type Output = Option<Self>;
            fn $f(mut self, rhs: Self) -> Self::Output {
                self.$fa(&rhs)?;
                Some(self)
            }
        }
        impl $tr<&Self> for Number {
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
impl Neg for Number {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.neg_assign();
        self
    }
}
impl NegAssign for Number {
    fn neg_assign(&mut self) {
        match self {
            Self::Complex(c) => c.neg_assign(),
            Self::Matrix(m) => m.neg_assign(),
        }
    }
}
impl Number {
    pub fn complex(&self) -> Option<NumberUnits<Complex>> {
        if let Self::Complex(c) = self {
            Some(*c)
        } else {
            None
        }
    }
    pub fn matrix(self) -> Option<Matrix<NumberUnits<Complex>>> {
        if let Self::Matrix(m) = self {
            Some(m)
        } else {
            None
        }
    }
    pub fn matrix_ref(&self) -> Option<&Matrix<NumberUnits<Complex>>> {
        if let Self::Matrix(m) = self {
            Some(m)
        } else {
            None
        }
    }
    pub fn matrix_mut(&mut self) -> Option<&mut Matrix<NumberUnits<Complex>>> {
        if let Self::Matrix(m) = self {
            Some(m)
        } else {
            None
        }
    }
    pub fn component_wise<F>(mut self, rhs: &Self, f: F) -> Option<Self>
    where
        F: FnMut(NumberUnits<Complex>, NumberUnits<Complex>) -> Option<NumberUnits<Complex>>,
    {
        self.component_wise_mut(rhs, f)?;
        Some(self)
    }
    pub fn component_wise_mut<F>(&mut self, rhs: &Self, mut f: F) -> Option<()>
    where
        F: FnMut(NumberUnits<Complex>, NumberUnits<Complex>) -> Option<NumberUnits<Complex>>,
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
                let vec = rhs.matrix_ref()?.entries();
                for row in self.matrix_mut()?.rows_mut() {
                    for (a, &b) in row.iter_mut().zip(vec) {
                        *a = f(*a, b)?;
                    }
                }
            }
            (NumberDimension::Matrix(ma), NumberDimension::Matrix(mb))
                if ma.height == mb.height && mb.width == 1 =>
            {
                let vec = rhs.matrix_ref()?.entries();
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
            (Self::Matrix(_), Self::Matrix(_)) => return None,
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
            (Self::Matrix(_), Self::Matrix(_)) => return None,
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
            (Self::Matrix(_), Self::Matrix(_)) => return None,
            _ => unreachable!(),
        }
        Some(())
    }
    pub fn new(dimension: &NumberDimension) -> Self {
        match *dimension {
            NumberDimension::Complex => Self::Complex(NumberUnits::default()),
            NumberDimension::Matrix(dim) => Self::Matrix(Matrix::new(dim)),
        }
    }
    pub fn size(&self) -> NumberDimension {
        match self {
            Self::Complex(_) => NumberDimension::Complex,
            Self::Matrix(mat) => NumberDimension::Matrix(mat.dimension),
        }
    }
    pub fn get(&self, index: NumberIndexChecked) -> Option<NumberUnits<Complex>> {
        if index.dimension != self.size() {
            return None;
        }
        match (self, index.index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(*val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get(i),
            _ => unreachable!(),
        }
    }
    pub fn get_unchecked(&self, index: NumberIndex) -> Option<NumberUnits<Complex>> {
        match (self, index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(*val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get(i),
            _ => unreachable!(),
        }
    }
    pub fn get_mut(&mut self, index: NumberIndexChecked) -> Option<&mut NumberUnits<Complex>> {
        if index.dimension != self.size() {
            return None;
        }
        match (self, index.index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get_mut(i),
            _ => unreachable!(),
        }
    }
    pub fn get_unchecked_mut(&mut self, index: NumberIndex) -> Option<&mut NumberUnits<Complex>> {
        match (self, index) {
            (Self::Complex(val), NumberIndex::Complex) => Some(val),
            (Self::Matrix(mat), NumberIndex::Matrix(i)) => mat.get_mut(i),
            _ => unreachable!(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (NumberIndexChecked, NumberUnits<Complex>)> {
        match self {
            &Self::Complex(c) => NumberIter::A(iter::once((NumberIndexChecked::complex(), c))),
            Self::Matrix(m) => NumberIter::B(
                m.iter()
                    .map(|(i, c)| (NumberIndexChecked::matrix(m.dimension, i), c)),
            ),
        }
    }
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (NumberIndexChecked, &mut NumberUnits<Complex>)> {
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
    pub fn transpose(self) -> Option<Self> {
        match self {
            c @ Self::Complex(_) => Some(c),
            Self::Matrix(mut m) => {
                m.transpose();
                Some(Self::Matrix(m))
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
