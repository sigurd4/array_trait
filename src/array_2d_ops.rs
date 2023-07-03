use super::*;

#[const_trait]
pub trait Array2DOps<T, const W: usize, const H: usize>: ArrayOps<[T; W], H> + Array
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

impl<T, const W: usize, const H: usize> const Array2DOps<T, W, H> for [[T; W]; H]
{
    #[inline]
    fn transpose(self) -> [[T; H]; W]
    {
        let mut matrix: [[MaybeUninit<T>; W]; H] = unsafe {
            private::transmute_unchecked_size(self)
        };
        let mut matrix_t: [[MaybeUninit<T>; H]; W] = unsafe {
            private::transmute_unchecked_size(MaybeUninit::<[T; H]>::uninit_array::<W>())
        };
        let mut i = 0;
        while i < H
        {
            let mut j = 0;
            while j < W
            {
                core::mem::swap(&mut matrix_t[j][i], &mut matrix[i][j]);
                j += 1;
            }
            i += 1;
        }
        unsafe {
            private::transmute_unchecked_size(matrix_t)
        }
    }
}