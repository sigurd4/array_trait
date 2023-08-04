use core::{mem::ManuallyDrop, marker::Destruct, ops::DerefMut};

use crate::{private, ConstIterator};

pub struct IntoConstIter<T, const LENGTH: usize, const DIR: bool, const ENUMERATE: bool = false>
{
    data: [ManuallyDrop<T>; LENGTH],
    i: usize
}

impl<T, const N: usize, const DIR: bool, const ENUMERATE: bool> IntoConstIter<T, N, DIR, ENUMERATE>
{
    #[inline]
    pub const fn from(array: [T; N]) -> Self
    {
        Self {
            data: unsafe {private::transmute_unchecked_size(array)},
            i: if DIR {0} else {N}
        }
    }
    
    pub const fn drop_copyable(self)
    where
        T: Copy
    {
        let mut this = ManuallyDrop::new(self);
        let this_mut = this.deref_mut();

        while this_mut.i != if DIR {N} else {0}
        {
            if !DIR
            {
                this_mut.i -= 1;
            }
            let _: T = unsafe {ManuallyDrop::into_inner(core::ptr::read(core::mem::transmute(&mut this_mut.data[this_mut.i])))};
            if DIR
            {
                this_mut.i += 1;
            }
        }
    }

    pub const fn drop(self)
    where
        T: ~const Destruct
    {
        let mut this = ManuallyDrop::new(self);
        let this_mut = this.deref_mut();

        while this_mut.i != if DIR {N} else {0}
        {
            if !DIR
            {
                this_mut.i -= 1;
            }
            let _: T = unsafe {ManuallyDrop::into_inner(core::ptr::read(core::mem::transmute(&mut this_mut.data[this_mut.i])))};
            if DIR
            {
                this_mut.i += 1;
            }
        }
    }
}

impl<T, const N: usize, const DIR: bool> IntoConstIter<T, N, DIR, false>
{
    #[inline]
    pub const fn next(&mut self) -> Option<T>
    {
        if self.i != if DIR {N} else {0}
        {
            if !DIR
            {
                self.i -= 1;
            }
            let out = unsafe {core::ptr::read(core::mem::transmute(&self.data[self.i]))};
            if DIR
            {
                self.i += 1;
            }
            return Some(out)
        }
        None
    }

    pub const fn enumerate(self) -> IntoConstIter<T, N, DIR, true>
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
}

impl<T, const N: usize, const DIR: bool> IntoConstIter<T, N, DIR, true>
{
    #[inline]
    pub const fn next(&mut self) -> Option<(usize, T)>
    {
        if self.i != if DIR {N} else {0}
        {
            if !DIR
            {
                self.i -= 1;
            }
            let i = self.i;
            let out = unsafe {core::ptr::read(core::mem::transmute(&self.data[self.i]))};
            if DIR
            {
                self.i += 1;
            }
            return Some((i, out))
        }
        None
    }
}

impl<T, const N: usize, const DIR: bool, const ENUMERATE: bool> const From<[T; N]> for IntoConstIter<T, N, DIR, ENUMERATE>
{
    #[inline]
    fn from(value: [T; N]) -> Self
    {
        Self::from(value)
    }
}
impl<T, const N: usize, const DIR: bool> const ConstIterator for IntoConstIter<T, N, DIR, false>
{
    type Item<'a> = T
    where
        Self: 'a;

    #[inline]
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>
    {
        Self::next(self)
    }
}
impl<T, const N: usize, const DIR: bool> const ConstIterator for IntoConstIter<T, N, DIR, true>
{
    type Item<'a> = (usize, T)
    where
        Self: 'a;

    #[inline]
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>
    {
        Self::next(self)
    }
}
impl<T, const N: usize, const DIR: bool, const ENUMERATE: bool> Drop for IntoConstIter<T, N, DIR, ENUMERATE>
{
    fn drop(&mut self)
    {
        while self.i != if DIR {N} else {0}
        {
            if !DIR
            {
                self.i -= 1;
            }
            unsafe {ManuallyDrop::drop(&mut self.data[self.i])};
            if DIR
            {
                self.i += 1;
            }
        }
    }
}