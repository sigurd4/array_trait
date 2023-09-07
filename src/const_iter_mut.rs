use crate::ConstIterator;

pub struct ConstIterMut<'a, T, const LENGTH: usize>
{
    data: &'a mut [T; LENGTH],
    i: usize
}

impl<'a, T, const N: usize> ConstIterMut<'a, T, N>
{   
    pub const fn from(array: &'a mut [T; N]) -> Self
    {
        Self {
            data: array,
            i: 0
        }
    }

    pub const fn next(&mut self) -> Option<&mut T>
    {
        if self.i < N
        {
            let out = &mut self.data[self.i];
            self.i += 1;
            Some(out)
        }
        else
        {
            None
        }
    }
}
impl<'a, T, const N: usize> const From<&'a mut [T; N]> for ConstIterMut<'a, T, N>
{
    fn from(value: &'a mut [T; N]) -> Self
    {
        Self::from(value)
    }
}
impl<T, const N: usize> const ConstIterator for ConstIterMut<'_, T, N>
{
    type Item<'a> = &'a mut T
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>
    {
        Self::next(self)
    }
}