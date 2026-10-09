use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl as _;
use crate::traits::number::NumberImpl;
use alloc::alloc::Global;
use core::alloc::{Allocator as _, Layout};
use core::fmt::{Debug, Formatter};
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::ptr::drop_in_place;
use core::slice;
use itertools::{IntoChunks, Itertools as _};
#[derive(Default)]
pub struct Matrix<C>
where
    C: NumberImpl,
{
    pub(crate) entries: Option<NonNull<C>>,
    pub dimension: MatrixDimension,
}
impl<C: NumberImpl> PartialEq for Matrix<C> {
    fn eq(&self, other: &Self) -> bool {
        self.dimension == other.dimension && self.entries() == other.entries()
    }
}
impl<C: NumberImpl> Debug for Matrix<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Matrix")
            .field("dimension", &self.dimension)
            .field_with("entries", |fmt| {
                let mut list = fmt.debug_list();
                for row in self.rows() {
                    list.entry(&row);
                }
                list.finish()
            })
            .finish()
    }
}
impl<C: NumberImpl> Clone for Matrix<C> {
    fn clone(&self) -> Self {
        Self::new_with(self.dimension, |i, _| self.entries()[i])
    }
}
impl<C: NumberImpl> Drop for Matrix<C> {
    fn drop(&mut self) {
        self.drop_entries(MatrixDimension::default());
    }
}
impl<C: NumberImpl> Matrix<C> {
    pub fn new(dim: MatrixDimension) -> Self {
        Self::new_with(dim, |_, _| C::default())
    }
    pub(crate) fn new_uninit(dim: MatrixDimension) -> Self {
        let mut ret = Self::default();
        if dim.size() == 0 {
            return ret;
        }
        let layout = Layout::array::<C>(dim.size()).unwrap();
        let ptr = Global.allocate(layout).unwrap();
        ret.entries = Some(ptr.cast());
        ret.dimension = dim;
        ret
    }
    pub fn new_with<F>(dim: MatrixDimension, mut fun: F) -> Self
    where
        F: FnMut(usize, MatrixIndex) -> C,
    {
        let mut ret = Self::new_uninit(dim);
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
    pub(crate) fn drop_entries(&mut self, dim: MatrixDimension) {
        for (i, entry) in self.iter_mut() {
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
    pub(crate) fn get_ptr(&self, index: MatrixIndex) -> Option<NonNull<C>> {
        if let Some(ptr) = self.entries {
            let i = self.dimension.index(index)?;
            Some(unsafe { ptr.add(i) })
        } else {
            None
        }
    }
    pub(crate) fn get_uninit(&mut self, index: MatrixIndex) -> Option<&mut MaybeUninit<C>> {
        if let Some(ptr) = self.entries {
            let i = self.dimension.index(index)?;
            let entry = unsafe { ptr.add(i) };
            Some(unsafe { entry.cast_uninit().as_mut() })
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
    pub fn row(&self, n: u32) -> &[C] {
        let u = n.strict_cast::<usize>();
        &self.entries()[self.width() * u..self.width() * (u + 1)]
    }
    pub fn row_mut(&mut self, n: u32) -> &mut [C] {
        let u = n.strict_cast::<usize>();
        let width = self.width();
        &mut self.entries_mut()[width * u..width * (u + 1)]
    }
    pub fn row_disjoint_mut<const N: usize>(&mut self, n: [u32; N]) -> Option<[&mut [C]; N]> {
        if n.iter().duplicates().count() != 0 || self.entries.is_none() {
            return None;
        }
        let u = n.map(|i| i.strict_cast::<usize>());
        let rows = u.map(|i| {
            let ptr = unsafe { self.entries.unwrap().add(self.width() * i) };
            unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), self.width()) }
        });
        Some(rows)
    }
    pub fn rows(&self) -> impl Iterator<Item = &[C]> {
        self.entries().chunks_exact(self.width())
    }
    pub fn cols(&self) -> IntoChunks<impl Iterator<Item = &C>> {
        (0..self.width())
            .flat_map(|c| (c..self.size()).step_by(self.width()))
            .map(|i| &self.entries()[i])
            .chunks(self.height())
    }
    pub fn entries_mut(&mut self) -> &mut [C] {
        if let Some(ptr) = self.entries {
            unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), self.size()) }
        } else {
            &mut []
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (MatrixIndex, &C)> {
        let dim = self.dimension;
        self.entries()
            .iter()
            .enumerate()
            .map(move |(i, val)| (MatrixIndex::from(dim, i).unwrap(), val))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut C)> {
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
impl<C: NumberImpl + ComplexImpl, D: Into<C::Real>, const N: usize, const M: usize>
    From<[[D; N]; M]> for Matrix<C>
{
    fn from(value: [[D; N]; M]) -> Self {
        let dim = MatrixDimension::new(N.strict_cast(), M.strict_cast());
        let mut vals = value.into_iter().flatten();
        Self::new_with(dim, |_, _| C::new_real_from(vals.next().unwrap()))
    }
}
