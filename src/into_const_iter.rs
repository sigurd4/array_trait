use core::{mem::MaybeUninit};

use crate::{private, ConstIterator};

pub struct IntoConstIter<T, const LENGTH: usize, const DIR: bool>
{
    data: [MaybeUninit<T>; LENGTH],
    i: Option<usize>
}

impl<T, const N: usize, const DIR: bool> IntoConstIter<T, N, DIR>
{   
    #[inline]
    pub const fn from(array: [T; N]) -> Self
    {
        Self {
            data: unsafe {private::transmute_unchecked_size(array)},
            i: Some(if DIR {0} else {N - 1})
        }
    }

    #[inline]
    pub const fn next(&mut self) -> Option<T>
    {
        if let Some(i) = self.i.as_mut()
        {
            if *i < N
            {
                let mut out = MaybeUninit::uninit();
                core::mem::swap(&mut out, &mut self.data[*i]);
                if DIR
                {
                    self.i = i.checked_add(1);
                }
                else
                {
                    self.i = i.checked_sub(1);
                }
                return Some(unsafe {MaybeUninit::assume_init(out)})
            }
        }
        None
    }
}
impl<T, const N: usize, const DIR: bool> const From<[T; N]> for IntoConstIter<T, N, DIR>
{
    #[inline]
    fn from(value: [T; N]) -> Self
    {
        Self::from(value)
    }
}
impl<T, const N: usize, const DIR: bool> const ConstIterator for IntoConstIter<T, N, DIR>
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