use crate::complex::Complex;
use core::alloc::{Allocator as _, Layout};
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::slice;
use std::alloc::Global;
#[derive(Default)]
pub struct Matrix<C>
where
    C: Complex,
{
    entries: Option<NonNull<C>>,
    pub dimension: MatrixDimension,
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MatrixDimension {
    pub width: u32,
    pub height: u32,
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MatrixIndex {
    pub row: u32,
    pub col: u32,
}
impl<C: Complex> Clone for Matrix<C> {
    fn clone(&self) -> Self {
        Self::with(self.dimension, |i, _| self.entries()[i].clone())
    }
}
impl<C: Complex> Drop for Matrix<C> {
    fn drop(&mut self) {
        if let Some(ptr) = self.entries {
            let layout = Layout::array::<C>(self.size()).unwrap();
            unsafe {
                Global.deallocate(ptr.cast(), layout);
            }
        }
    }
}
impl<C: Complex> Matrix<C> {
    pub fn new(dim: MatrixDimension) -> Self {
        Self::with(dim, |_, _| C::default())
    }
    pub fn with<F>(dim: MatrixDimension, mut fun: F) -> Self
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
            let index = MatrixIndex::from(dim, i);
            entry.write(fun(i, index));
        }
        ret
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
    pub fn entries_uninit_mut(&mut self) -> &mut [MaybeUninit<C>] {
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
    pub fn index(self, index: MatrixIndex) -> usize {
        index.row() * self.width() + index.col()
    }
}
impl MatrixIndex {
    pub fn row(self) -> usize {
        self.row.strict_cast()
    }
    pub fn col(self) -> usize {
        self.col.strict_cast()
    }
    pub fn from(dim: MatrixDimension, index: usize) -> Self {
        let row = (index / dim.width()).strict_cast();
        let col = index.div_euclid(dim.width()).strict_cast();
        Self { row, col }
    }
}
