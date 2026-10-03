use crate::assign_neg;
use crate::matrix::dense::Matrix;
use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::assign::NegAssign;
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl;
use core::ops::Neg;
impl<C: ComplexImpl> MatrixImpl for Matrix<C> {
    type Entry = C;
    fn add(&mut self, rhs: Self) -> Option<()> {
        if self.dimension != rhs.dimension {
            return None;
        }
        for (a, &b) in self.entries_mut().iter_mut().zip(rhs.entries()) {
            *a += b;
        }
        Some(())
    }
    fn dimension(&self) -> MatrixDimension {
        self.dimension
    }
    fn get(&self, index: MatrixIndex) -> Option<&C> {
        let i = self.dimension.index(index)?;
        Some(&self.entries()[i])
    }
    fn get_mut(&mut self, index: MatrixIndex) -> Option<&mut C> {
        let i = self.dimension.index(index)?;
        Some(&mut self.entries_mut()[i])
    }
}
impl<C: ComplexImpl> Neg for Matrix<C> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for entry in self.entries_mut() {
            entry.neg_assign();
        }
        self
    }
}
assign_neg!(Matrix<C>, C: ComplexImpl);
