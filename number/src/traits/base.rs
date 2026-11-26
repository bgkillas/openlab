use crate::traits::assign::NegAssign;
use crate::traits::number::NumberImpl;
use core::fmt::Debug;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
pub trait BaseImpl:
    Default
    + Copy
    + Clone
    + Debug
    + PartialEq
    + NumberImpl
    + Operations
    + NegAssign
    + Sum<Self>
    + Product<Self>
{
}
pub trait Operations:
    Sized
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
//for<'a> &'a Self: Add<Self, Output = Self>
//    + Sub<Self, Output = Self>
//    + Mul<Self, Output = Self>
//    + Div<Self, Output = Self>,
{
}
impl<
    T: Sized
        + Add<Self, Output = Self>
        + AddAssign<Self>
        + Sub<Self, Output = Self>
        + SubAssign<Self>
        + Mul<Self, Output = Self>
        + MulAssign<Self>
        + Div<Self, Output = Self>
        + DivAssign<Self>
        + Neg<Output = Self>
        + for<'a> Add<&'a Self, Output = Self>
        + for<'a> Sub<&'a Self, Output = Self>
        + for<'a> Mul<&'a Self, Output = Self>
        + for<'a> Div<&'a Self, Output = Self>,
> Operations for T
{
}
