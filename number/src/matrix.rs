use crate::complex::ComplexImpl;
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
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MatrixDimension {
    pub width: u32,
    pub height: u32,
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MatrixIndex {
    pub col: u32,
    pub row: u32,
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
        Self::new_with(self.dimension, |i, _| self.entries()[i].clone())
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
        for (i, entry) in self.entries_enumerate_mut() {
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
    pub fn get(&self, index: MatrixIndex) -> Option<&C> {
        let i = self.dimension.index(index)?;
        Some(&self.entries()[i])
    }
    pub fn get_mut(&mut self, index: MatrixIndex) -> Option<&mut C> {
        let i = self.dimension.index(index)?;
        Some(&mut self.entries_mut()[i])
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
    pub fn entries_enumerate_mut(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut C)> {
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
    pub fn width(&self) -> usize {
        self.dimension.width()
    }
    pub fn height(&self) -> usize {
        self.dimension.height()
    }
    pub fn size(&self) -> usize {
        self.dimension.size()
    }
}
impl MatrixDimension {
    pub fn width(self) -> usize {
        self.width.strict_cast()
    }
    pub fn height(self) -> usize {
        self.height.strict_cast()
    }
    pub fn size(self) -> usize {
        self.width() * self.height()
    }
    pub fn index(self, index: MatrixIndex) -> Option<usize> {
        let n = index.row() * self.width() + index.col();
        (index.col < self.width && index.row < self.height).then_some(n)
    }
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl MatrixIndex {
    pub fn row(self) -> usize {
        self.row.strict_cast()
    }
    pub fn col(self) -> usize {
        self.col.strict_cast()
    }
    pub fn from(dim: MatrixDimension, index: usize) -> Option<Self> {
        let row = (index / dim.width()).strict_cast();
        let col = index.rem_euclid(dim.width()).strict_cast();
        (row < dim.height).then_some(Self { col, row })
    }
}
