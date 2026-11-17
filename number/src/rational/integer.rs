use core::fmt::Debug;
use core::ops::{Add, Div, Mul, Sub};
pub trait IntegerTrait:
    Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Sized
    + Clone
    + Debug
    + Copy
{
}
