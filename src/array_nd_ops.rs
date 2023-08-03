use super::*;

/// A trait for N-dimensional arrays
#[const_trait]
pub trait ArrayNdOps<const D: usize, T, const L: usize>: ArrayPrereq
{
    type Mapped<M>: ~const ArrayNdOps<D, M, L>;

    /// Fills an N-dimensional array. Indices passed to fill-function are sorted from outermost to innermost.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(const_trait_impl)]
    /// #![feature(const_closures)]
    /// #![feature(const_mut_refs)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// type T = u8;
    /// 
    /// const ND: [[T; 3]; 3] = ArrayNdOps::fill_nd(const |[i, j]| 1 + 3*i as T + j as T);
    /// 
    /// assert_eq!(ND, [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ]);
    /// ```
    fn fill_nd<F>(fill: F) -> Self
    where
        F: ~const FnMut([usize; D]) -> T + ~const Destruct;

    /// Maps each element in the N-dimensional array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(const_trait_impl)]
    /// #![feature(const_closures)]
    /// #![feature(const_mut_refs)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// const ND: [[u8; 3]; 3] = [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ];
    /// 
    /// const ND_MAPPED: [[i8; 3]; 3] = ND.map_nd(const |x: u8| -(x as i8));
    /// 
    /// assert_eq!(ND_MAPPED, [
    ///     [-1, -2, -3],
    ///     [-4, -5, -6],
    ///     [-7, -8, -9]
    /// ]);
    /// ```
    fn map_nd<M>(self, map: M) -> Self::Mapped<<M as FnOnce<(T,)>>::Output>
    where
        M: ~const FnMut<(T,)> + ~const Destruct;

    /// Enumerates each element of an N-dimensional array. Indices are sorted from outermost to innermost.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(const_trait_impl)]
    /// #![feature(const_closures)]
    /// #![feature(const_mut_refs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// type T = u8;
    /// 
    /// const ND: [[T; 3]; 3] = [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ];
    /// 
    /// // For now, the compiler cannot infer the type, so type annotations are needed.
    /// const ND_ENUM: [[([usize; 2], T); 3]; 3] = <[[T; 3]; 3] as ArrayNdOps<2, _, _>>::enumerate_nd(ND);
    /// 
    /// assert_eq!(ND_ENUM, [
    ///     [([0, 0], 1), ([0, 1], 2), ([0, 2], 3)],
    ///     [([1, 0], 4), ([1, 1], 5), ([1, 2], 6)],
    ///     [([2, 0], 7), ([2, 1], 8), ([2, 2], 9)]
    /// ]);
    /// ```
    fn enumerate_nd(self) -> Self::Mapped<([usize; D], T)>;

    /// Flattens one or multiple dimensions of an N-dimensional array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(const_trait_impl)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// type T = u8;
    /// 
    /// const ND: [[T; 3]; 3] = [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ];
    /// const FLAT: [T; 9] = ND.flatten_nd_array();
    /// assert_eq!(FLAT, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    fn flatten_nd_array(self) -> [T; L]
    where
        [(); L]:;

    /// Flattens one or multiple dimensions of an N-dimensional array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(const_trait_impl)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// type T = u8;
    /// 
    /// const ND: [[T; 3]; 3] = [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ];
    /// const FLAT: &[T; 9] = ND.flatten_nd_array_ref();
    /// assert_eq!(FLAT, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    fn flatten_nd_array_ref(&self) -> &[T; L]
    where
        [(); L]:;
    
    /// Flattens one or multiple dimensions of an N-dimensional array-slice
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// type T = u8;
    /// 
    /// let mut nd: [[T; 3]; 3] = [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ];
    /// let flat: &mut [T; 9] = nd.flatten_nd_array_mut();
    /// 
    /// for x in flat.into_iter()
    /// {
    ///     *x = 10 - *x;
    /// }
    /// 
    /// assert_eq!(nd, [
    ///     [9, 8, 7],
    ///     [6, 5, 4],
    ///     [3, 2, 1]
    /// ]);
    /// ```
    fn flatten_nd_array_mut(&mut self) -> &mut [T; L]
    where
        [(); L]:;

    fn each_ref_nd<B>(&self) -> Self::Mapped<&B>
    where
        T: ~const Borrow<B>;
    fn each_mut_nd<B>(&mut self) -> Self::Mapped<&mut B>
    where
        T: ~const BorrowMut<B>;

    fn reduce_nd<R>(self, reduce: R) -> Option<T>
    where
        R: ~const FnMut(T, T) -> T + ~const Destruct,
        T: ~const Destruct;
        
    /// Retrieves the inner item using an array of indices, sorted from outermost to innermost, as a reference
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(const_closures)]
    /// #![feature(const_option)]
    /// #![feature(const_trait_impl)]
    /// #![feature(generic_const_exprs)]
    /// #![feature(const_mut_refs)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// const A: [[u8; 2]; 3] = [
    ///     [1, 2],
    ///     [3, 4],
    ///     [5, 6]
    /// ];
    /// let b: [[u8; 2]; 3] = ArrayNdOps::fill_nd(|[i, j]| {
    ///     let item = *A.get_nd([i, j]).unwrap();
    ///     assert_eq!(item, A[i][j]);
    ///     item
    /// });
    /// 
    /// assert_eq!(A, b);
    /// ```
    fn get_nd(&self, i: [usize; D]) -> Option<&T>;
    
    /// Retrieves the inner item using an array of indices, sorted from outermost to innermost, as a mutable reference
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(const_closures)]
    /// #![feature(const_option)]
    /// #![feature(const_trait_impl)]
    /// #![feature(generic_const_exprs)]
    /// #![feature(const_mut_refs)]
    /// 
    /// use array_trait::ArrayNdOps;
    /// 
    /// const A: [[u8; 2]; 3] = [
    ///     [1, 2],
    ///     [3, 4],
    ///     [5, 6]
    /// ];
    /// let mut b: [[u8; 2]; 3] = [[0; 2]; 3];
    /// 
    /// let mut i = 0;
    /// while i < 3
    /// {
    ///     let mut j = 0;
    ///     while j < 2
    ///     {
    ///         let item = *A.get_nd([i, j]).unwrap();
    ///         assert_eq!(item, A[i][j]);
    ///         *b.get_nd_mut([i, j]).unwrap() = item;
    ///         j += 1;
    ///     }
    ///     i += 1;
    /// }
    /// 
    /// assert_eq!(A, b);
    /// ```
    fn get_nd_mut(&mut self, i: [usize; D]) -> Option<&mut T>;
}