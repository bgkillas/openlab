use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::assign::NegAssign;
use crate::traits::number::NumberImpl;
use core::fmt::Debug;
use core::ops::Neg;
pub trait MatrixImpl: Default + Clone + Debug + PartialEq + Neg<Output = Self> + NegAssign {
    type Entry: NumberImpl;
    fn add_assign(&mut self, rhs: &Self) -> Option<()>;
    fn sub_assign(&mut self, rhs: &Self) -> Option<()>;
    fn mul_assign(&mut self, rhs: &Self) -> Option<()>;
    fn transpose(&mut self);
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
    fn swap(&mut self, from: MatrixIndex, to: MatrixIndex) -> Option<()>;
    fn get(&self, index: MatrixIndex) -> Option<&Self::Entry>;
    fn get_mut(&mut self, index: MatrixIndex) -> Option<&mut Self::Entry>;
    fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [MatrixIndex; N],
    ) -> Option<[&mut Self::Entry; N]>;
    fn swap_row(&mut self, from: u32, to: u32) -> Option<()>;
    fn swap_col(&mut self, from: u32, to: u32) -> Option<()>;
}
