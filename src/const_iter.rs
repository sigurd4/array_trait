use crate::ConstIterator;

pub struct ConstIter<'a, T, const LENGTH: usize>
{
    data: &'a [T; LENGTH],
    i: usize
}

impl<'a, T, const N: usize> ConstIter<'a, T, N>
{   
    #[inline]
    pub const fn from(array: &'a [T; N]) -> Self
    {
        Self {
            data: array,
            i: 0
        }
    }

    #[inline]
    pub const fn next(&mut self) -> Option<&T>
    {
        if self.i < N
        {
            let out = &self.data[self.i];
            self.i += 1;
            Some(out)
        }
        else
        {
            None
        }
    }
    
    #[inline]
    pub const fn next_enumerated(&mut self) -> Option<(usize, &T)>
    {
        if self.i < N
        {
            let out = (self.i, &self.data[self.i]);
            self.i += 1;
            Some(out)
        }
        else
        {
            None
        }
    }
}
impl<'a, T, const N: usize> const From<&'a [T; N]> for ConstIter<'a, T, N>
{
    #[inline]
    fn from(value: &'a [T; N]) -> Self
    {
        Self::from(value)
    }
}
impl<T, const N: usize> const ConstIterator for ConstIter<'_, T, N>
{
    type Item<'a> = &'a T
    where
        Self: 'a;

    #[inline]
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>
    {
        Self::next(self)
    }
}