use core::fmt::Debug;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
pub trait BaseImpl:
    Default
    + Clone
    + Debug
    + PartialEq
    + Add<Output = Self>
    + AddAssign<Self>
    + Sub<Output = Self>
    + SubAssign<Self>
    + Mul<Output = Self>
    + MulAssign<Self>
    + Div<Output = Self>
    + DivAssign<Self>
    + Neg<Output = Self>
{
}
