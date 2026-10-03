use crate::traits::assign::NegAssign;
use core::fmt::Debug;
use core::ops::Neg;
pub trait MatrixImpl: Default + Clone + Debug + PartialEq + Neg<Output = Self> + NegAssign {}
