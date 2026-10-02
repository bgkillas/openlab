use crate::complex::Complex;
use core::alloc::{Allocator as _, Layout};
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::slice;
use std::alloc::Global;
pub struct Matrix<C>
where
    C: Complex,
{
    entries: Option<NonNull<C>>,
    pub width: u32,
    pub height: u32,
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
impl<C: Complex> Default for Matrix<C> {
    fn default() -> Self {
        Self {
            entries: None,
            width: 0,
            height: 0,
        }
    }
}
impl<C: Complex> Matrix<C> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut ret = Self::default();
        let layout = Layout::array::<C>(width * height).unwrap();
        let ptr = Global.allocate(layout).unwrap();
        ret.entries = Some(ptr.cast());
        ret.width = width.strict_cast();
        ret.height = height.strict_cast();
        for entry in ret.entries_uninit_mut() {
            entry.write(C::default());
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
        self.width.strict_cast()
    }
    pub fn height(&self) -> usize {
        self.height.strict_cast()
    }
    pub fn size(&self) -> usize {
        self.width() * self.height()
    }
}
