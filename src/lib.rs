#![feature(generic_const_exprs)]
#![feature(iter_array_chunks)]
#![feature(split_array)]
#![feature(iter_next_chunk)]

use std::{borrow::{Borrow, BorrowMut}, hash::Hash, slice::{Iter, IterMut}, ops::{Index, Range, RangeInclusive, RangeFrom, RangeTo, RangeToInclusive, IndexMut, RangeFull, Deref}};

mod private
{
    pub trait Array {}
    impl<Item, const LENGTH: usize> Array for [Item; LENGTH] {}
}

pub trait Array: private::Array
    + Sized
    + IntoIterator//<IntoIter = std::array::IntoIter<<Self as IntoIterator>::Item, {Self::_LENGTH}>>
    + AsRef<[Self::Item]>
    + AsMut<[Self::Item]>
    + Borrow<[Self::Item]>
    + BorrowMut<[Self::Item]>
    + Index<usize, Output = <[Self::Item] as Index<usize>>::Output>
    + Index<Range<usize>, Output = <[Self::Item] as Index<Range<usize>>>::Output>
    + Index<RangeInclusive<usize>, Output = <[Self::Item] as Index<RangeInclusive<usize>>>::Output>
    + Index<RangeFrom<usize>, Output = <[Self::Item] as Index<RangeFrom<usize>>>::Output>
    + Index<RangeTo<usize>, Output = <[Self::Item] as Index<RangeTo<usize>>>::Output>
    + Index<RangeToInclusive<usize>, Output = <[Self::Item] as Index<RangeToInclusive<usize>>>::Output>
    + Index<RangeFull, Output = <[Self::Item] as Index<RangeFull>>::Output>
    + IndexMut<usize>
    + IndexMut<Range<usize>>
    + IndexMut<RangeInclusive<usize>>
    + IndexMut<RangeFrom<usize>>
    + IndexMut<RangeTo<usize>>
    + IndexMut<RangeToInclusive<usize>>
    + IndexMut<RangeFull>
where
    for<'a> &'a Self: TryFrom<&'a [Self::Item]>
        + IntoIterator<Item = &'a Self::Item, IntoIter = Iter<'a, Self::Item>>,
    for<'a> &'a mut Self: TryFrom<&'a mut [Self::Item]> + IntoIterator<Item = &'a mut Self::Item, IntoIter = IterMut<'a, Self::Item>>
{
    const LENGTH: usize;

    fn into_array(self) -> [Self::Item; Self::LENGTH];
    fn as_array(&self) -> &[Self::Item; Self::LENGTH];
    fn as_array_mut(&mut self) -> &mut [Self::Item; Self::LENGTH];

    fn split_array<const M: usize>(self) -> ([Self::Item; M], [Self::Item; Self::LENGTH - M])
    {
        let mut iter = self.into_iter();
        unsafe {(iter.next_chunk().unwrap_unchecked(), iter.next_chunk().unwrap_unchecked())}
    }
    
    #[inline]
    fn split_array_ref2<const M: usize>(&self) -> (&[Self::Item; M], &[Self::Item; Self::LENGTH - M])
    {
        let (a, b) = (&self[..]).split_array_ref::<M>();
        (a, b.split_array_ref::<{Self::LENGTH - M}>().0)
    }
    
    #[inline]
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut [Self::Item; M], &mut [Self::Item; Self::LENGTH - M])
    {
        let (a, b) = (&mut self[..]).split_array_mut::<M>();
        (a, b.split_array_mut::<{Self::LENGTH - M}>().0)
    }
    
    #[inline]
    fn rsplit_array_ref2<const M: usize>(&self) -> (&[Self::Item; Self::LENGTH - M], &[Self::Item; M])
    {
        let (a, b) = (&self[..]).rsplit_array_ref::<M>();
        (a.split_array_ref::<{Self::LENGTH - M}>().0, b)
    }
    
    #[inline]
    fn rsplit_array_mut2<const M: usize>(&mut self) -> (&mut [Self::Item; Self::LENGTH - M], &mut [Self::Item; M])
    {
        let (a, b) = (&mut self[..]).rsplit_array_mut::<M>();
        (a.split_array_mut::<{Self::LENGTH - M}>().0, b)
    }
}
impl<Item, const LENGTH: usize> Array for [Item; LENGTH]
{
    const LENGTH: usize = LENGTH;

    #[inline]
    fn into_array(self) -> [Item; Self::LENGTH]
    {
        self.into_iter().array_chunks().next().unwrap()
    }
    #[inline]
    fn as_array(&self) -> &[Item; Self::LENGTH]
    {
        unsafe {std::mem::transmute(self)}
    }
    #[inline]
    fn as_array_mut(&mut self) -> &mut [Item; Self::LENGTH]
    {
        unsafe {std::mem::transmute(self)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        let a = [1.0, 2.0, 3.0];
        {
            let a_ref: &[f32; 3] = a.as_array();
            assert_eq!(a, *a_ref);
        }
        let mut a: [f32; 3] = a.into_array();
        {
            let _a_mut: &[f32; 3] = a.as_array_mut();
        }
    }
}
