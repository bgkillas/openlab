use crate::assign_neg;
use crate::matrix::dense::Matrix;
use crate::traits::assign::NegAssign;
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl;
use core::ops::Neg;
impl<C: ComplexImpl> MatrixImpl for Matrix<C> {}
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
