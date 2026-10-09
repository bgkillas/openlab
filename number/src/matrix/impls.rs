use crate::assign_neg;
use crate::matrix::dense::Matrix;
use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::assign::NegAssign;
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl;
use core::mem;
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
                let index = MatrixIndex::new(j.strict_cast(), i.strict_cast());
                let entry = row.iter().zip(vec.iter()).map(|(a, b)| *a * **b).sum();
                new.get_uninit(index).unwrap().write(entry);
            }
            vec.clear();
        }
        *self = new;
        Some(())
    }
    fn transpose(&mut self) {
        if self.height() == self.width() {
            for i in 0..self.dimension.height {
                for j in i + 1..=self.dimension.width {
                    let from = MatrixIndex::new(j, i);
                    let to = MatrixIndex::new(i, j);
                    self.swap(from, to);
                }
            }
        } else {
            let transposed = self.dimension.transpose();
            let mut is_set: Vec<bool> = vec![false; self.size()];
            is_set[0] = true;
            is_set[self.size() - 1] = true;
            let mut i = 1;
            let mut cycle_start;
            let mut next;
            let mut t;
            while i < self.size() - 1 {
                cycle_start = i;
                t = self.entries()[i];
                loop {
                    next = (i * self.height()).rem_euclid(self.size() - 1);
                    mem::swap(&mut t, &mut self.entries_mut()[next]);
                    is_set[i] = true;
                    i = next;
                    if i == cycle_start {
                        break;
                    }
                }
                while is_set.get(i).is_some_and(|b| *b) {
                    i += 1;
                }
            }
            self.dimension = transposed;
        }
    }
    fn dimension(&self) -> MatrixDimension {
        self.dimension
    }
    fn swap(&mut self, from: MatrixIndex, to: MatrixIndex) -> Option<()> {
        let i = self.dimension.index(from)?;
        let j = self.dimension.index(to)?;
        self.entries_mut().swap(i, j);
        Some(())
    }
    fn get(&self, index: MatrixIndex) -> Option<&C> {
        let i = self.dimension.index(index)?;
        Some(&self.entries()[i])
    }
    fn get_mut(&mut self, index: MatrixIndex) -> Option<&mut C> {
        let i = self.dimension.index(index)?;
        Some(&mut self.entries_mut()[i])
    }
    fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [MatrixIndex; N],
    ) -> Option<[&mut Self::Entry; N]> {
        let i = indices
            .map(|index| self.dimension.index(index))
            .transpose()?;
        self.entries_mut().get_disjoint_mut(i).ok()
    }
    fn swap_row(&mut self, from: u32, to: u32) -> Option<()> {
        if from == to {
            return Some(());
        }
        let [a, b] = self.row_disjoint_mut([from, to])?;
        a.swap_with_slice(b);
        Some(())
    }
    fn swap_col(&mut self, from: u32, to: u32) -> Option<()> {
        if from == to {
            return Some(());
        }
        for r in 0..self.height() {
            let i1 = MatrixIndex::new(r.strict_cast(), from);
            let i2 = MatrixIndex::new(r.strict_cast(), to);
            self.swap(i1, i2);
        }
        Some(())
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
