use std::{mem::MaybeUninit, borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

pub struct Padded<T, const WIDTH: usize>([MaybeUninit<T>; WIDTH])
where
    [(); WIDTH - 1]:;
impl<T, const WIDTH: usize> std::fmt::Debug for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: std::fmt::Debug
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.borrow().fmt(f)
    }
}
impl<T, const WIDTH: usize> std::fmt::Display for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: std::fmt::Display
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.borrow().fmt(f)
    }
}

impl<T, const WIDTH: usize> Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    #[inline]
    pub const fn new(value: T) -> Self
    {
        let mut array = MaybeUninit::uninit_array();
        array[0] = MaybeUninit::new(value);
        Self(array)
    }
    #[inline]
    pub fn into_inner(self) -> T
    {
        unsafe {MaybeUninit::assume_init(self.0.into_iter().next().unwrap())}
    }
    #[inline]
    pub const fn borrow(&self) -> &T
    {
        unsafe {self.0[0].assume_init_ref()}
    }
    #[inline]
    pub const fn borrow_mut(&mut self) -> &mut T
    {
        unsafe {self.0[0].assume_init_mut()}
    }
}
impl<T, const WIDTH: usize> const Borrow<T> for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    #[inline]
    fn borrow(&self) -> &T
    {
        self.borrow()
    }
}
impl<T, const WIDTH: usize> const BorrowMut<T> for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut T
    {
        self.borrow_mut()
    }
}
impl<T, const WIDTH: usize> const Deref for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        self.borrow()
    }
}
impl<T, const WIDTH: usize> const DerefMut for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.borrow_mut()
    }
}