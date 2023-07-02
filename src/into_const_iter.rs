use std::{mem::MaybeUninit};

use crate::{private, ConstIterator};

pub struct IntoConstIter<T, const LENGTH: usize>
{
    data: [MaybeUninit<T>; LENGTH],
    i: usize
}

impl<T, const N: usize> IntoConstIter<T, N>
{   
    #[inline]
    pub const fn from(array: [T; N]) -> Self
    {
        Self {
            data: unsafe {private::transmute_unchecked_size(array)},
            i: 0
        }
    }

    #[inline]
    pub const fn next(&mut self) -> Option<T>
    {
        if self.i < N
        {
            let mut out = MaybeUninit::uninit();
            std::mem::swap(&mut out, &mut self.data[self.i]);
            self.i += 1;
            Some(unsafe {MaybeUninit::assume_init(out)})
        }
        else
        {
            None
        }
    }
}
impl<T, const N: usize> const From<[T; N]> for IntoConstIter<T, N>
{
    #[inline]
    fn from(value: [T; N]) -> Self
    {
        Self::from(value)
    }
}
impl<T, const N: usize> const ConstIterator for IntoConstIter<T, N>
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