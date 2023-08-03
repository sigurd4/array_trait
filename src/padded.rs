use core::{mem::{MaybeUninit, ManuallyDrop}, borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

#[repr(C)]
pub struct Padded<T, const WIDTH: usize>
where
    [(); WIDTH - 1]:
{
    value: T,
    _pad: ManuallyDrop<MaybeUninit<[T; WIDTH - 1]>>
}

impl<T, const WIDTH: usize> core::fmt::Debug for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: core::fmt::Debug
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        self.borrow().fmt(f)
    }
}
impl<T, const WIDTH: usize> core::fmt::Display for Padded<T, WIDTH>
where
    [(); WIDTH - 1]:,
    T: core::fmt::Display
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
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
        Self
        {
            value,
            _pad: ManuallyDrop::new(MaybeUninit::uninit())
        }
    }
    #[inline]
    pub fn into_inner(self) -> T
    {
        self.value
    }
    #[inline]
    pub const fn borrow(&self) -> &T
    {
        &self.value
    }
    #[inline]
    pub const fn borrow_mut(&mut self) -> &mut T
    {
        &mut self.value
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