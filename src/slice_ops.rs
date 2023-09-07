use super::*;

#[const_trait]
pub trait SliceOps<T>: SlicePrereq<T>
{
    fn rsplit_at(&self, mid: usize) -> (&[T], &[T]);
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);

    /// Does the exact same as the method in the standard library in an identical way,
    /// but can be done at compile-time.
    /// 
    /// Divides one slice into an array and a remainder slice at an index.
    ///
    /// The array will contain all indices from `[0, N)` (excluding
    /// the index `N` itself) and the slice will contain all
    /// indices from `[N, len)` (excluding the index `len` itself).
    ///
    /// # Panics
    ///
    /// Panics if `N > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(split_array)]
    /// #![feature(const_trait_impl)]
    /// #![feature(const_slice_index)]
    /// 
    /// use array_trait::SliceOps;
    ///
    /// const V: &[u8] = &[1, 2, 3, 4, 5, 6][..];
    ///
    /// {
    ///     const SPLIT: (&[u8; 0], &[u8]) = V.split_array_ref2();
    ///     assert_eq!(SPLIT.0, &[]);
    ///     assert_eq!(SPLIT.1, [1, 2, 3, 4, 5, 6]);
    ///     assert_eq!(SPLIT, V.split_array_ref::<0>());
    /// }
    ///
    /// {
    ///     const SPLIT: (&[u8; 2], &[u8]) = V.split_array_ref2();
    ///     assert_eq!(SPLIT.0, &[1, 2]);
    ///     assert_eq!(SPLIT.1, [3, 4, 5, 6]);
    ///     assert_eq!(SPLIT, V.split_array_ref::<2>());
    /// }
    ///
    /// {
    ///     const SPLIT: (&[u8; 6], &[u8]) = V.split_array_ref2();
    ///     assert_eq!(SPLIT.0, &[1, 2, 3, 4, 5, 6]);
    ///     assert_eq!(SPLIT.1, []);
    ///     assert_eq!(SPLIT, V.split_array_ref::<6>());
    /// }
    /// ```
    fn split_array_ref2<const N: usize>(&self) -> (&[T; N], &[T]);
    fn split_array_mut2<const N: usize>(&mut self) -> (&mut [T; N], &mut [T]);

    fn rsplit_array_ref2<const N: usize>(&self) -> (&[T], &[T; N]);
    fn rsplit_array_mut2<const N: usize>(&mut self) -> (&mut [T], &mut [T; N]);
}

impl<T> const SliceOps<T> for [T]
{
    fn rsplit_at(&self, mid: usize) -> (&[T], &[T])
    {
        assert!(mid <= self.len());
        self.split_at(self.len() - mid)
    }
    fn rsplit_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])
    {
        assert!(mid <= self.len());
        self.split_at_mut(self.len() - mid)
    }

    fn split_array_ref2<const N: usize>(&self) -> (&[T; N], &[T])
    {
        let (left, right) = self.split_at(N);
        unsafe {(&*(left.as_ptr() as *const [T; N]), right)}
    }
    fn split_array_mut2<const N: usize>(&mut self) -> (&mut [T; N], &mut [T])
    {
        let (left, right) = self.split_at_mut(N);
        unsafe {(&mut *(left.as_mut_ptr() as *mut [T; N]), right)}
    }

    fn rsplit_array_ref2<const N: usize>(&self) -> (&[T], &[T; N])
    {
        let (left, right) = self.rsplit_at(N);
        unsafe {(left, &*(right.as_ptr() as *const [T; N]))}
    }
    fn rsplit_array_mut2<const N: usize>(&mut self) -> (&mut [T], &mut [T; N])
    {
        let (left, right) = self.rsplit_at_mut(N);
        unsafe {(left, &mut *(right.as_mut_ptr() as *mut [T; N]))}
    }
}

#[cfg(test)]
#[test]
fn test()
{
    let a = [1, 2];

    let ar: &[u8] = &a;

    let split = ar.rsplit_array_ref2::<2>();
}