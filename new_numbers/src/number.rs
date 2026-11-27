use crate::float::arbitrary::BigFloat;
use crate::units::Units;
use fraction::GenericFraction;
use malachite::Rational;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ComplexType {
    Arbitrary(u64),
    F128,
    F64,
    F32,
}
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum RationalType {
    #[default]
    Arbitrary,
    U128,
    U64,
    U32,
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NumberType {
    Complex(ComplexType),
    Rational(RationalType),
}
#[derive(Clone, PartialEq, Debug)]
pub enum NumberElement {
    Complex(ComplexElement),
    Rational(RationalElement),
}
#[derive(Clone, PartialEq, Debug)]
pub enum ComplexElement {
    Arbitrary(Units<BigFloat>),
    F128(!),
    F64(Units<f64>),
    F32(Units<f32>),
}
#[derive(Clone, PartialEq, Debug)]
pub enum RationalElement {
    Arbitrary(Units<Rational>),
    U128(Units<GenericFraction<u128>>),
    U64(Units<GenericFraction<u64>>),
    U32(Units<GenericFraction<u32>>),
}
#[derive(Clone, PartialEq, Debug)]
pub enum NumberElementMatrix {
    Complex(ComplexElementMatrix),
    Rational(RationalElementMatrix),
}
#[derive(Clone, PartialEq, Debug)]
pub enum ComplexElementMatrix {
    Arbitrary(!),
    F128(!),
    F64(!),
    F32(!),
}
#[derive(Clone, PartialEq, Debug)]
pub enum RationalElementMatrix {
    Arbitrary(!),
    U128(!),
    U64(!),
    U32(!),
}
#[derive(Clone, PartialEq, Debug)]
pub enum Number {
    Complex(NumberElement),
    Matrix(NumberElementMatrix),
}
impl Default for ComplexType {
    fn default() -> Self {
        Self::Arbitrary(512)
    }
}
