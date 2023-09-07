use core::{mem::ManuallyDrop, ops::{Mul, AddAssign}};

use super::*;

#[const_trait]
pub trait Array2dOps<T, const M: usize, const N: usize>: ArrayOps<[T; N], M>
{
    type Array2d<I, const H: usize, const W: usize>: Array2dOps<I, H, W>;
    
    type Resized2d<const H: usize, const W: usize>: Array2dOps<T, H, W> = Self::Array2d<T, H, W>;
    
    type Transposed: Array2dOps<T, N, M> = Self::Resized2d<N, M>;

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
    fn transpose(self) -> Self::Transposed;
    
    fn mul_matrix<Rhs, const P: usize>(&self, rhs: &Self::Array2d<Rhs, N, P>) -> Self::Array2d<<T as Mul<Rhs>>::Output, M, P>
    where
        T: ~const Mul<Rhs, Output: ~const AddAssign + ~const Default> + Copy,
        Rhs: Copy;
}

impl<T, const M: usize, const N: usize> const Array2dOps<T, M, N> for [[T; N]; M]
{
    type Array2d<I, const H: usize, const W: usize> = [[I; W]; H];
    
    #[inline]
    fn transpose(self) -> Self::Transposed
    {
        // Alternative 1: (dirtier)
        let mut this_t: [[T; M]; N] = unsafe {private::uninit()};
        let mut i = 0;
        while i != M
        {
            let mut j = 0;
            while j != N
            {
                unsafe {core::ptr::copy_nonoverlapping(
                    self[i][j].borrow() as *const T,
                    &mut this_t[j][i] as *mut T,
                    1
                )};
                j += 1;
            }
            i += 1;
        }

        core::mem::forget(self);

        this_t

        // Alternative 2: (cleaner)
        /*ArrayOps::fill(const |i| ArrayOps::fill(const |j| unsafe {
            core::ptr::read(&this.borrow()[j][i] as *const T)
        }))*/
    }
    
    fn mul_matrix<Rhs, const P: usize>(&self, rhs: &Self::Array2d<Rhs, N, P>) -> Self::Array2d<<T as Mul<Rhs>>::Output, M, P>
    where
        T: ~const Mul<Rhs, Output: ~const AddAssign + ~const Default> + Copy,
        Rhs: Copy
    {
        let mut prod: [[<T as Mul<Rhs>>::Output; P]; M] = unsafe {private::uninit()};
        let mut m = 0;
        while m != M
        {
            let mut p = 0;
            while p != P
            {
                let mut n = 0;
                while n != N
                {
                    prod[m][p] += self[m][n]*rhs[n][p];
                    n += 1;
                }
                p += 1;
            }
            m += 1;
        }

        core::mem::forget((self, rhs));

        prod
    }
}