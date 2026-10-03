use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl as _;
use core::alloc::{Allocator as _, Layout};
use core::fmt::{Debug, Formatter};
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::ptr::drop_in_place;
use core::slice;
use std::alloc::Global;
#[derive(Default)]
pub struct Matrix<C>
where
    C: ComplexImpl,
{
    entries: Option<NonNull<C>>,
    pub dimension: MatrixDimension,
}
impl<C: ComplexImpl> PartialEq for Matrix<C> {
    fn eq(&self, other: &Self) -> bool {
        self.dimension == other.dimension && self.entries() == other.entries()
    }
}
impl<C: ComplexImpl> Debug for Matrix<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Matrix")
            .field("dimension", &self.dimension)
            .field("entries", &self.entries())
            .finish()
    }
}
impl<C: ComplexImpl> Clone for Matrix<C> {
    fn clone(&self) -> Self {
        Self::new_with(self.dimension, |i, _| self.entries()[i])
    }
}
impl<C: ComplexImpl> Drop for Matrix<C> {
    fn drop(&mut self) {
        self.drop_entries(MatrixDimension::default());
    }
}
impl<C: ComplexImpl> Matrix<C> {
    pub fn new(dim: MatrixDimension) -> Self {
        Self::new_with(dim, |_, _| C::default())
    }
    pub fn new_with<F>(dim: MatrixDimension, mut fun: F) -> Self
    where
        F: FnMut(usize, MatrixIndex) -> C,
    {
        let mut ret = Self::default();
        if dim.size() == 0 {
            return ret;
        }
        let layout = Layout::array::<C>(dim.size()).unwrap();
        let ptr = Global.allocate(layout).unwrap();
        ret.entries = Some(ptr.cast());
        ret.dimension = dim;
        for (i, entry) in ret.entries_uninit_mut().iter_mut().enumerate() {
            let index = MatrixIndex::from(dim, i).unwrap();
            entry.write(fun(i, index));
        }
        ret
    }
    pub fn new_dimension(&mut self, dim: MatrixDimension) {
        self.new_dimension_with(dim, |_, _| C::default());
    }
    pub fn new_dimension_with<F>(&mut self, dim: MatrixDimension, mut fun: F)
    where
        F: FnMut(usize, MatrixIndex) -> C,
    {
        if self.dimension == dim {
            return;
        }
        let new = Self::new_with(dim, |i, index| {
            if let Some(ptr) = self.get_ptr(index) {
                unsafe { ptr.read() }
            } else {
                fun(i, index)
            }
        });
        self.drop_entries(dim);
        *self = new;
    }
    fn drop_entries(&mut self, dim: MatrixDimension) {
        for (i, entry) in self.iter_enumerate_mut() {
            if dim.index(i).is_some() {
                continue;
            }
            unsafe {
                drop_in_place(entry);
            }
        }
        if let Some(ptr) = self.entries {
            let layout = Layout::array::<C>(self.size()).unwrap();
            unsafe {
                Global.deallocate(ptr.cast(), layout);
            }
            self.entries = None;
            self.dimension = MatrixDimension::default();
        }
    }
    fn get_ptr(&self, index: MatrixIndex) -> Option<NonNull<C>> {
        if let Some(ptr) = self.entries {
            let i = self.dimension.index(index)?;
            Some(unsafe { ptr.add(i) })
        } else {
            None
        }
    }
    pub fn entries(&self) -> &[C] {
        if let Some(ptr) = self.entries {
            unsafe { slice::from_raw_parts(ptr.as_ptr(), self.size()) }
        } else {
            &[]
        }
    }
    pub fn entries_mut(&mut self) -> &mut [C] {
        if let Some(ptr) = self.entries {
            unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), self.size()) }
        } else {
            &mut []
        }
    }
    pub fn iter_enumerate_mut(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut C)> {
        let dim = self.dimension;
        self.entries_mut()
            .iter_mut()
            .enumerate()
            .map(move |(i, val)| (MatrixIndex::from(dim, i).unwrap(), val))
    }
    fn entries_uninit_mut(&mut self) -> &mut [MaybeUninit<C>] {
        if let Some(ptr) = self.entries {
            unsafe { slice::from_raw_parts_mut(ptr.cast_uninit().as_ptr(), self.size()) }
        } else {
            &mut []
        }
    }
}
