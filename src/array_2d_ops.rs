use core::mem::ManuallyDrop;

use super::*;

#[const_trait]
pub trait Array2dOps<T, const W: usize, const H: usize>: ArrayOps<[T; W], H>
{
    /// Transposes a two-dimensional array (as if it were a matrix)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array_trait::*;
    /// 
    /// let matrix: [[u8; 5]; 3] = [
    ///     [1,   2,  3,  4,  5],
    ///     [6,   7,  8,  9, 10],
    ///     [11, 12, 13, 14, 15]
    /// ];
    /// 
    /// assert_eq!(matrix.transpose(), [
    ///     [1,  6, 11],
    ///     [2,  7, 12],
    ///     [3,  8, 13],
    ///     [4,  9, 14],
    ///     [5, 10, 15]
    /// ]);
    /// ```
    fn transpose(self) -> [[T; H]; W];
}

impl<T, const W: usize, const H: usize> const Array2dOps<T, W, H> for [[T; W]; H]
{
    #[inline]
    fn transpose(self) -> [[T; H]; W]
    {
        let this = ManuallyDrop::new(self);
        let mut this_t: [[T; H]; W] = unsafe {MaybeUninit::assume_init(MaybeUninit::uninit())};
        let mut i = 0;
        while i != H
        {
            let mut j = 0;
            while j != W
            {
                unsafe {core::ptr::copy_nonoverlapping(
                    this[i][j].borrow() as *const T,
                    &mut this_t[j][i] as *mut T,
                    1
                )};
                j += 1;
            }
            i += 1;
        }
        this_t
        /*ArrayOps::fill(const |i| ArrayOps::fill(const |j| unsafe {
            unsafe {core::ptr::read(&matrix.deref()[j][i] as *const T)}
        }))*/
    }
}