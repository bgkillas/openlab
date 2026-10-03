use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::assign::NegAssign;
use crate::traits::complex::ComplexImpl;
use core::fmt::Debug;
use core::ops::Neg;
pub trait MatrixImpl: Default + Clone + Debug + PartialEq + Neg<Output = Self> + NegAssign {
    type Entry: ComplexImpl;
    fn add(&mut self, rhs: Self) -> Option<()>;
    fn dimension(&self) -> MatrixDimension;
    fn width(&self) -> usize {
        self.dimension().width()
    }
    fn height(&self) -> usize {
        self.dimension().height()
    }
    fn size(&self) -> usize {
        self.dimension().size()
    }
    fn get(&self, index: MatrixIndex) -> Option<&Self::Entry>;
    fn get_mut(&mut self, index: MatrixIndex) -> Option<&mut Self::Entry>;
}
