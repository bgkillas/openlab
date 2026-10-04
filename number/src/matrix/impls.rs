use crate::assign_neg;
use crate::matrix::dense::Matrix;
use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::assign::NegAssign;
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl;
use core::ops::Neg;
impl<C: ComplexImpl> MatrixImpl for Matrix<C> {
    type Entry = C;
    fn add(&mut self, rhs: &Self) -> Option<()> {
        if self.dimension != rhs.dimension {
            return None;
        }
        for (a, &b) in self.entries_mut().iter_mut().zip(rhs.entries()) {
            *a += b;
        }
        Some(())
    }
    fn sub(&mut self, rhs: &Self) -> Option<()> {
        if self.dimension != rhs.dimension {
            return None;
        }
        for (a, &b) in self.entries_mut().iter_mut().zip(rhs.entries()) {
            *a -= b;
        }
        Some(())
    }
    fn mul(&mut self, rhs: &Self) -> Option<()> {
        if self.width() != rhs.height() {
            return None;
        }
        let dim = MatrixDimension {
            width: rhs.dimension.width,
            height: self.dimension.height,
        };
        let mut new = Self::new_uninit(dim);
        let mut vec: Vec<&C> = Vec::with_capacity(rhs.height());
        for (i, col) in rhs.cols().into_iter().enumerate() {
            vec.extend(col);
            for (j, row) in self.rows().enumerate() {
                let index = MatrixIndex {
                    col: i.strict_cast(),
                    row: j.strict_cast(),
                };
                let entry = row.iter().zip(vec.iter()).map(|(a, b)| *a * **b).sum();
                new.get_uninit(index).unwrap().write(entry);
            }
            vec.clear();
        }
        *self = new;
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
