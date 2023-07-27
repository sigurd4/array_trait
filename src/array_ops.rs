use core::ops::{Add, Sub, AddAssign};

use super::*;

trait Equals<T>
where
    T: ?Sized {}
impl<T> Equals<T> for T
where
    T: ?Sized
{
    
}

#[const_trait]
pub trait ArrayOps<T, const N: usize>: ArrayPrereq + IntoIterator<Item = T>
    + Borrow<[T; N]>
    + BorrowMut<[T; N]>
{
    type Array<I, const L: usize>: ~const ArrayOps<I, L> = [I; L];
    type PaddedItem<I, const W: usize>: ~const Borrow<I> + ~const BorrowMut<I>
        = Padded<I, W>
    where
        [(); W - 1]:;
    type PaddedArray<I, const W: usize, const L: usize>: ~const ArrayOps<Self::PaddedItem<I, W>, L>
        = Self::Array<Self::PaddedItem<I, W>, L>
    where
        [(); W - 1]:;

    fn fill<F>(fill: F) -> Self
    where
        F: ~const FnMut(usize) -> T + ~const Destruct;
    fn rfill<F>(fill: F) -> Self
    where
        F: ~const FnMut(usize) -> T + ~const Destruct;
    
    fn truncate<const M: usize>(self) -> Self::Array<T, M>
    where
        T: ~const Destruct,
        [(); N - M]:;
    fn rtruncate<const M: usize>(self) -> Self::Array<T, M>
    where
        T: ~const Destruct,
        [(); N - M]:;

    fn resize<const M: usize, F>(self, fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;
    fn rresize<const M: usize, F>(self, fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;

    fn extend<const M: usize, F>(self, fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;
    fn rextend<const M: usize, F>(self, fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;

    /// Converts an array into a const interator.
    /// 
    /// The const iterator does not implement [std::iter::Iterator](Iterator), and as such is more limited in its usage.
    /// However it can be used at compile-time.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(inline_const)]
    /// #![feature(const_trait_impl)]
    /// #![feature(const_mut_refs)]
    /// 
    /// use array_trait::*;
    /// 
    /// const A: [u8; 3] = [1, 2, 3];
    /// 
    /// const A_SUM: u8 = const {
    ///     let mut iter = A.into_const_iter();
    ///     let mut sum = 0;
    /// 
    ///     while let Some(b) = iter.next()
    ///     {
    ///         sum += b;
    ///     }
    /// 
    ///     sum
    /// };
    /// 
    /// assert_eq!(A_SUM, 1 + 2 + 3);
    /// ```
    fn into_const_iter(self) -> IntoConstIter<T, N, true>;
    fn into_const_iter_reverse(self) -> IntoConstIter<T, N, false>;

    /// Makes a const iterator over the array-slice.
    /// 
    /// The const iterator does not implement [std::iter::Iterator](Iterator), and as such is more limited in its usage.
    /// However it can be used at compile-time.
    fn const_iter(&self) -> ConstIter<'_, T, N>;
    /// Makes a mutable const iterator over the mutable array-slice.
    /// 
    /// The const iterator does not implement [std::iter::Iterator](Iterator), and as such is more limited in its usage.
    /// However it can be used at compile-time.
    fn const_iter_mut(&mut self) -> ConstIterMut<'_, T, N>;

    /// Maps all values of an array with a given function.
    /// 
    /// This method can be executed at compile-time, as opposed to the standard-library method.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(const_closures)]
    /// #![feature(const_mut_refs)]
    /// #![feature(const_trait_impl)]
    /// 
    /// use array_trait::*;
    /// 
    /// const A: [u8; 4] = [1, 2, 3, 4];
    /// const B: [i8; 4] = A.map2(const |b| -(b as i8));
    /// 
    /// assert_eq!(B, [-1, -2, -3, -4]);
    /// ```
    fn map2<M>(self, map: M) -> Self::Array<<M as FnOnce<(T,)>>::Output, N>
    where
        M: ~const FnMut<(T,)> + ~const Destruct;

    /// Combines two arrays with possibly different items into parallel, where each element lines up in the same position.
    /// 
    /// This method can be executed at compile-time, as opposed to the standard-library method.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(const_trait_impl)]
    /// 
    /// use array_trait::*;
    /// 
    /// const A: [u8; 4] = [4, 3, 2, 1];
    /// const B: [&str; 4] = ["four", "three", "two", "one"];
    /// const C: [(u8, &str); 4] = A.zip2(B);
    /// 
    /// assert_eq!(C, [(4, "four"), (3, "three"), (2, "two"), (1, "one")]);
    /// ```
    fn zip2<O, Rhs>(self, other: Rhs) -> Self::Array<(T, O), N>
    where
        Rhs: ~const Into<Self::Array<O, N>>;

    fn enumerate(self) -> Self::Array<(usize, T), N>;
    
    /// Differentiates array (discrete calculus)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use array_trait::*;
    /// 
    /// let a = [1, 2, 3];
    /// 
    /// assert_eq!(a.differentiate(), [2 - 1, 3 - 2]);
    /// ```
    fn differentiate(self) -> Self::Array<<T as Sub<T>>::Output, {N - 1}>
    where
        T: ~const Sub<T> + Copy;
    
    /// Integrates array (discrete calculus)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array_trait::*;
    /// 
    /// let a = [1, 2, 3];
    /// 
    /// assert_eq!(a.integrate(), [1, 1 + 2, 1 + 2 + 3])
    /// ```
    fn integrate(self) -> Self::Array<T, N>
    where
        T: ~const AddAssign<T> + Copy;

    fn reduce<R>(self, reduce: R) -> Option<T>
    where
        T: ~const Destruct,
        R: ~const FnMut(T, T) -> T + ~const Destruct;

    /// Chains two arrays with the same item together.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array_trait::*;
    /// 
    /// let a = ["one", "two"];
    /// let b = ["three"];
    /// 
    /// assert_eq!(a.chain(b), ["one", "two", "three"]);
    /// ```
    fn chain<const M: usize, Rhs>(self, rhs: Rhs) -> Self::Array<T, {N + M}>
    where
        Rhs: ~const Into<Self::Array<T, M>>;

    /// Chains two arrays with the same item together in reverse.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array_trait::*;
    /// 
    /// let a = ["two", "three"];
    /// let b = ["one"];
    /// 
    /// assert_eq!(a.rchain(b), ["one", "two", "three"]);
    /// ```
    fn rchain<const M: usize, Rhs>(self, rhs: Rhs) -> Self::Array<T, {N + M}>
    where
        Rhs: ~const Into<Self::Array<T, M>>;

    /// Distributes items of an array equally across a given width, then provides the rest as a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::*;
    /// 
    /// let array = ["ping 1", "pong 1", "ping 2", "pong 2", "ping 3", "pong 3", "uhh..."];
    /// 
    /// let ([ping, pong], rest) = array.array_spread::<2>();
    /// 
    /// assert_eq!(ping, ["ping 1", "ping 2", "ping 3"]);
    /// assert_eq!(pong, ["pong 1", "pong 2", "pong 3"]);
    /// assert_eq!(rest, ["uhh..."]);
    /// ```
    fn array_spread<const M: usize>(self) -> ([Self::Array<T, {N / M}>; M], Self::Array<T, {N % M}>)
    where
        [(); M - 1]:,
        T: Copy;

    /// Distributes items of an array-slice equally across a given width, then provides the rest as a separate array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    fn array_spread_ref<const M: usize>(&self) -> ([&Self::PaddedArray<T, M, {N / M}>; M], &Self::Array<T, {N % M}>)
    where
        [(); M - 1]:;
    
    /// Distributes items of a mutable array-slice equally across a given width, then provides the rest as a separate mutable array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::*;
    /// 
    /// let mut array = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20"];
    /// 
    /// let (threes, _) = array.array_spread_mut::<3>();
    /// 
    /// for fizz in threes.into_iter().last().unwrap()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let (fives, _) = array.array_spread_mut::<5>();
    /// 
    /// for buzz in fives.into_iter().last().unwrap()
    /// {
    ///     **buzz = "buzz";
    /// }
    /// 
    /// let (fifteens, _) = array.array_spread_mut::<15>();
    /// 
    /// for fizzbuzz in fifteens.into_iter().last().unwrap()
    /// {
    ///     **fizzbuzz = "fizzbuzz";
    /// }
    /// 
    /// assert_eq!(array, ["1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14", "fizzbuzz", "16", "17", "fizz", "19", "buzz"]);
    /// 
    /// ```
    fn array_spread_mut<const M: usize>(&mut self) -> ([&mut Self::PaddedArray<T, M, {N / M}>; M], &mut Self::Array<T, {N % M}>)
    where
        [(); M - 1]:;
    
    /// Distributes items of an array equally across a given width, then provides the leftmost rest as a separate array.
    fn array_rspread<const M: usize>(self) -> (Self::Array<T, {N % M}>, [Self::Array<T, {N / M}>; M])
    where
        [(); M - 1]:,
        T: Copy;

    /// Distributes items of an array-slice equally across a given width, then provides the leftmost rest as a separate array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array_trait::*;
    /// 
    /// let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    /// 
    /// let (zero, [odd, even]) = array.array_rspread_ref::<2>();
    /// 
    /// assert_eq!(*zero, [0]);
    /// assert_eq!(odd.each_ref().map(|padding| **padding), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    /// assert_eq!(even.each_ref().map(|padding| **padding), [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    /// ```
    fn array_rspread_ref<const M: usize>(&self) -> (&Self::Array<T, {N % M}>, [&Self::PaddedArray<T, M, {N / M}>; M])
    where
        [(); M - 1]:;
    /// Distributes items of a mutable array-slice equally across a given width, then provides the leftmost rest as a separate mutable array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array_trait::*;
    /// 
    /// let mut array = ["the", "beat", "goes", "1", "2", "3", "4", "5", "6", "7", "8"];
    /// 
    /// let (start, [boots, n, cats, and]) = array.array_rspread_mut::<4>();
    /// 
    /// for boots in boots
    /// {
    ///     **boots = "boots";
    /// }
    /// for n in n
    /// {
    ///     **n = "n";
    /// }
    /// for cats in cats
    /// {
    ///     **cats = "cats";
    /// }
    /// for and in and
    /// {
    ///     **and = "and";
    /// }
    /// 
    /// assert_eq!(array, ["the", "beat", "goes", "boots", "n", "cats", "and", "boots", "n", "cats", "and"]);
    /// ```
    fn array_rspread_mut<const M: usize>(&mut self) -> (&mut Self::Array<T, {N % M}>, [&mut Self::PaddedArray<T, M, {N / M}>; M])
    where
        [(); M - 1]:;
    
    /// Distributes items of an array equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::*;
    /// 
    /// let array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.array_spread_exact::<2>();
    /// 
    /// assert_eq!(lower_case, *b"abcdefghijklmnopqrstuvwxyz");
    /// assert_eq!(upper_case, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// ```
    fn array_spread_exact<const M: usize>(self) -> [Self::Array<T, {N / M}>; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:;
    
    /// Distributes items of an array-slice equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array_trait::*;
    /// 
    /// let statement = ["s", "he", "be", "lie", "ve", "d"];
    /// 
    /// let [interpretation2, interpretation1] = statement.array_spread_exact_ref::<2>();
    /// 
    /// assert_eq!(interpretation1.each_ref().map(|padding| &**padding), ["he", "lie", "d"].each_ref());
    /// assert_eq!(interpretation2.each_ref().map(|padding| &**padding), ["s", "be", "ve"].each_ref());
    /// ```
    fn array_spread_exact_ref<const M: usize>(&self) -> [&Self::PaddedArray<T, M, {N / M}>; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:;

    /// Distributes items of a mutable array-slice equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array_trait::*;
    /// 
    /// let mut array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.array_spread_exact_mut::<2>();
    /// 
    /// assert_eq!(lower_case.each_ref().map(|padding| padding.borrow()), b"abcdefghijklmnopqrstuvwxyz".each_ref());
    /// assert_eq!(upper_case.each_ref().map(|padding| padding.borrow()), b"ABCDEFGHIJKLMNOPQRSTUVWXYZ".each_ref());
    /// 
    /// for c in upper_case
    /// {
    ///     **c = b'_';
    /// }
    /// 
    /// assert_eq!(array, *b"a_b_c_d_e_f_g_h_i_j_k_l_m_n_o_p_q_r_s_t_u_v_w_x_y_z_")
    /// ```
    fn array_spread_exact_mut<const M: usize>(&mut self) -> [&mut Self::PaddedArray<T, M, {N / M}>; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:;

    /// Divides an array into chunks, then yielding the rest in a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::*;
    /// 
    /// let array = ["carrot", "potato", "beet", "tomato", "kiwi", "banana", "cherry", "peach", "strawberry", "nine volt batteries"];
    /// 
    /// let ([root_vegetables, technically_berries, stone_fruits], not_for_human_consumption) = array.array_chunks::<3>();
    /// 
    /// assert_eq!(root_vegetables, ["carrot", "potato", "beet"]);
    /// assert_eq!(technically_berries, ["tomato", "kiwi", "banana"]);
    /// assert_eq!(stone_fruits, ["cherry", "peach", "strawberry"]);
    /// assert_eq!(not_for_human_consumption, ["nine volt batteries"]);
    /// ```
    fn array_chunks<const M: usize>(self) -> ([Self::Array<T, M>; N / M], Self::Array<T, {N % M}>);
    /// Divides an array-slice into chunks, then yielding the rest in a separate array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::*;
    /// 
    /// let transistors = ["2N3904", "2N2222A", "BC107", "AC127", "OC7", "NKT275", "2SK30A", "2N5458", "J108", "2N7000", "BS170"];
    /// 
    /// let ([silicon_bjts, germanium_bjts, jfets], mosfets) = transistors.array_chunks_ref::<3>();
    /// 
    /// assert_eq!(silicon_bjts, &["2N3904", "2N2222A", "BC107"]);
    /// assert_eq!(germanium_bjts, &["AC127", "OC7", "NKT275"]);
    /// assert_eq!(jfets, &["2SK30A", "2N5458", "J108"]);
    /// assert_eq!(mosfets, &["2N7000", "BS170"]);
    /// ```
    fn array_chunks_ref<const M: usize>(&self) -> ([&Self::Array<T, M>; N / M], &Self::Array<T, {N % M}>);
    /// Divides a mutable array-slice into chunks, then yielding the rest in a separate mutable array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::*;
    /// 
    /// let mut array = [0, 1, 0, 1, 0, 1, 6];
    /// 
    /// let (pairs, last) = array.array_chunks_mut::<2>();
    /// 
    /// for (i, pair) in pairs.into_iter().enumerate()
    /// {
    ///     for number in pair
    ///     {
    ///         *number += i*2;
    ///     }
    /// }
    /// 
    /// assert_eq!(array, [0, 1, 2, 3, 4, 5, 6]);
    /// ```
    fn array_chunks_mut<const M: usize>(&mut self) -> ([&mut Self::Array<T, M>; N / M], &mut Self::Array<T, {N % M}>);
    
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate mutable array-slice.
    fn array_rchunks<const M: usize>(self) -> (Self::Array<T, {N % M}>, [Self::Array<T, M>; N / M]);
    /// Divides an array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn array_rchunks_ref<const M: usize>(&self) -> (&Self::Array<T, {N % M}>, [&Self::Array<T, M>; N / M]);
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn array_rchunks_mut<const M: usize>(&mut self) -> (&mut Self::Array<T, {N % M}>, [&mut Self::Array<T, M>; N / M]);
    
    /// Divides an array into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    fn array_chunks_exact<const M: usize>(self) -> [Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    /// Divides an array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::*;
    /// 
    /// let array = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    /// 
    /// let [lower_half, upper_half] = array.array_chunks_exact_ref::<5>();
    /// 
    /// assert_eq!(lower_half, &[0.0, 0.1, 0.2, 0.3, 0.4]);
    /// assert_eq!(upper_half, &[0.5, 0.6, 0.7, 0.8, 0.9]);
    /// ```
    fn array_chunks_exact_ref<const M: usize>(&self) -> [&Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    fn array_chunks_exact_mut<const M: usize>(&mut self) -> [&mut Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;

    /// Splits an array at a chosen index.
    fn split_array<const M: usize>(self) -> (Self::Array<T, M>, Self::Array<T, {N - M}>)
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as array-slices.
    fn split_array_ref2<const M: usize>(&self) -> (&Self::Array<T, M>, &Self::Array<T, {N - M}>)
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as mutable array-slices.
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut Self::Array<T, M>, &mut Self::Array<T, {N - M}>)
    where
        [(); N - M]:;
    
    /// Splits an array at a chosen index, where the index goes from right to left.
    fn rsplit_array<const M: usize>(self) -> (Self::Array<T, {N - M}>, Self::Array<T, M>)
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as array-slices, where the index goes from right to left.
    fn rsplit_array_ref2<const M: usize>(&self) -> (&Self::Array<T, {N - M}>, &Self::Array<T, M>)
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as mutable array-slices, where the index goes from right to left.
    fn rsplit_array_mut2<const M: usize>(&mut self) -> (&mut Self::Array<T, {N - M}>, &mut Self::Array<T, M>)
    where
        [(); N - M]:;

    fn each_ref2<B>(&self) -> Self::Array<&B, N>
    where
        T: ~const Borrow<B>;
    fn each_mut2<B>(&mut self) -> Self::Array<&mut B, N>
    where
        T: ~const BorrowMut<B>;
}

impl<T, const N: usize> const ArrayOps<T, N> for [T; N]
{
    type Array<I, const M: usize> = [I; M];

    #[inline]
    fn fill<F>(mut fill: F) -> Self
    where
        F: ~const FnMut(usize) -> T + ~const Destruct
    {
        let mut array = MaybeUninit::uninit_array();
        let mut i = 0;
        while i < Self::LENGTH
        {
            array[i] = MaybeUninit::new(fill(i));
            i += 1;
        }
        unsafe {MaybeUninit::array_assume_init(array)}
    }
    #[inline]
    fn rfill<F>(mut fill: F) -> Self
    where
        F: ~const FnMut(usize) -> T + ~const Destruct
    {
        let mut array = MaybeUninit::uninit_array();
        let mut i = N - 1;
        while let Some(i_next) = i.checked_sub(1)
        {
            array[i] = MaybeUninit::new(fill(i));
            i = i_next;
        }
        unsafe {MaybeUninit::array_assume_init(array)}
    }
    
    #[inline]
    fn truncate<const M: usize>(self) -> Self::Array<T, M>
    where
        T: ~const Destruct,
        [(); N - M]:
    {
        self.split_array().0
    }
    #[inline]
    fn rtruncate<const M: usize>(self) -> Self::Array<T, M>
    where
        T: ~const Destruct,
        [(); N - M]:
    {
        self.rsplit_array().1
    }

    #[inline]
    fn resize<const M: usize, F>(self, mut fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct
    {
        let mut iter = self.into_const_iter();
        ArrayOps::fill(const move |i| match iter.next()
        {
            Some(item) => item,
            None => fill(i)
        })
    }
    #[inline]
    fn rresize<const M: usize, F>(self, mut fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct
    {
        let mut iter = self.into_const_iter_reverse();
        ArrayOps::rfill(const move |i| match iter.next()
        {
            Some(item) => item,
            None => fill(i)
        })
    }
    
    #[inline]
    fn extend<const M: usize, F>(self, mut fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = ArrayOps::fill(const move |i| fill(i + N));
        unsafe {
            private::transmute_unchecked_size((self, filled))
        }
    }
    #[inline]
    fn rextend<const M: usize, F>(self, fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = ArrayOps::fill(fill);
        unsafe {
            private::transmute_unchecked_size((filled, self))
        }
    }

    #[inline]
    fn into_const_iter(self) -> IntoConstIter<T, N, true>
    {
        IntoConstIter::from(self)
    }
    #[inline]
    fn into_const_iter_reverse(self) -> IntoConstIter<T, N, false>
    {
        IntoConstIter::from(self)
    }
    
    #[inline]
    fn const_iter(&self) -> ConstIter<'_, T, N>
    {
        ConstIter::from(self)
    }
    #[inline]
    fn const_iter_mut(&mut self) -> ConstIterMut<'_, T, N>
    {
        ConstIterMut::from(self)
    }
    
    #[inline]
    fn map2<M>(self, mut map: M) -> Self::Array<<M as FnOnce<(T,)>>::Output, N>
    where
        M: ~const FnMut<(T,)> + ~const Destruct
    {
        let mut iter = self.into_const_iter();
        ArrayOps::fill(const |_| map(iter.next().unwrap()))
    }
    
    #[inline]
    fn zip2<O, Rhs>(self, other: Rhs) -> Self::Array<(T, O), N>
    where
        Rhs: ~const Into<Self::Array<O, N>>
    {
        let mut iter_self = self.into_const_iter();
        let mut iter_other = other.into().into_const_iter();
        ArrayOps::fill(const |_| (iter_self.next().unwrap(), iter_other.next().unwrap()))
    }
    
    #[inline]
    fn enumerate(self) -> Self::Array<(usize, T), N>
    {
        let mut iter_self = self.into_const_iter();
        ArrayOps::fill(const |i| (i, iter_self.next().unwrap()))
    }

    #[inline]
    fn differentiate(self) -> Self::Array<<T as Sub<T>>::Output, {N - 1}>
    where
        T: ~const Sub<T> + Copy + ~const Destruct
    {
        let mut iter_self = self.into_const_iter();
        if let Some(mut x0) = iter_self.next()
        {
            ArrayOps::fill(const move |_| {
                let x = iter_self.next().unwrap();
                let dx = x - x0;
                x0 = x;
                dx
            })
        }
        else
        {
            unsafe {
                private::transmute_unchecked_size::<[<T as Sub<T>>::Output; 0], _>([])
            }
        }
    }

    #[inline]
    fn integrate(self) -> Self::Array<T, N>
        where
            T: ~const AddAssign<T> + Copy + ~const Destruct
    {
        let mut iter_self = self.into_const_iter();
        if let Some(mut x_accum) = iter_self.next()
        {
            ArrayOps::fill(const move |_| {
                let xi = x_accum;
                if let Some(x) = iter_self.next()
                {
                    x_accum += x;
                }
                xi
            })
        }
        else
        {
            unsafe {
                private::transmute_unchecked_size::<[T; 0], _>([])
            }
        }
    }

    #[inline]
    fn reduce<R>(self, mut reduce: R) -> Option<T>
    where
        R: ~const FnMut(T, T) -> T + ~const Destruct,
        T: ~const Destruct
    {
        let mut iter = self.into_const_iter();
        let mut reduction = match iter.next()
        {
            Some(x) => x,
            None => return None
        };
        while let Some(x) = iter.next()
        {
            reduction = reduce(reduction, x);
        }
        Some(reduction)
    }
    
    #[inline]
    fn chain<const M: usize, Rhs>(self, rhs: Rhs) -> Self::Array<T, {N + M}>
    where
        Rhs: ~const Into<Self::Array<T, M>>
    {
        unsafe {private::transmute_unchecked_size((self, rhs))}
    }
    
    #[inline]
    fn rchain<const M: usize, Rhs>(self, rhs: Rhs) -> Self::Array<T, {N + M}>
    where
        Rhs: ~const Into<Self::Array<T, M>>
    {
        unsafe {private::transmute_unchecked_size((rhs, self))}
    }
    
    #[inline]
    fn array_spread<const M: usize>(self) -> ([Self::Array<T, {N / M}>; M], Self::Array<T, {N % M}>)
    where
        [(); M - 1]:,
        T: Copy
    {
        let (spread_t, rest): ([[T; M]; N / M], [T; N % M]) = unsafe {
            private::transmute_unchecked_size(self)
        };
        (spread_t.transpose(), rest)
    }
    #[inline]
    fn array_spread_ref<const M: usize>(&self) -> ([&Self::PaddedArray<T, M, {N / M}>; M], &Self::Array<T, {N % M}>)
    where
        [(); M - 1]:
    {
        let mut ptr = self as *const T;

        (ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            slice
        }), unsafe {core::mem::transmute(ptr)})
    }
    #[inline]
    fn array_spread_mut<const M: usize>(&mut self) -> ([&mut Self::PaddedArray<T, M, {N / M}>; M], &mut Self::Array<T, {N % M}>)
    where
        [(); M - 1]:
    {
        let mut ptr = self as *mut T;

        (ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            slice
        }), unsafe {core::mem::transmute(ptr)})
    }
    
    #[inline]
    fn array_rspread<const M: usize>(self) -> ([T; N % M], [Self::Array<T, {N / M}>; M])
    where
        [(); M - 1]:,
        T: Copy
    {
        let (start, spread_t): ([T; N % M], [[T; M]; N / M]) = unsafe {
            private::transmute_unchecked_size(self)
        };
        (start, spread_t.transpose())
    }
    #[inline]
    fn array_rspread_ref<const M: usize>(&self) -> (&[T; N % M], [&Self::PaddedArray<T, M, {N / M}>; M])
    where
        [(); M - 1]:
    {
        let start = self as *const T;
        let mut ptr = unsafe {start.add(Self::LENGTH % M)};

        (unsafe {core::mem::transmute(start)}, ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            slice
        }))
    }
    #[inline]
    fn array_rspread_mut<const M: usize>(&mut self) -> (&mut [T; N % M], [&mut Self::PaddedArray<T, M, {N / M}>; M])
    where
        [(); M - 1]:
    {
        let start = self as *mut T;
        let mut ptr = unsafe {start.add(Self::LENGTH % M)};

        (unsafe {core::mem::transmute(start)}, ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            slice
        }))
    }
    #[inline]
    fn array_spread_exact<const M: usize>(self) -> [Self::Array<T, {N / M}>; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:
    {
        let spread_t: [[T; M]; N / M] = unsafe {
            private::transmute_unchecked_size(self)
        };
        spread_t.transpose()
    }
    #[inline]
    fn array_spread_exact_ref<const M: usize>(&self) -> [&Self::PaddedArray<T, M, {N / M}>; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:
    {
        let mut ptr = self as *const T;
        
        ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            slice
        })
    }
    #[inline]
    fn array_spread_exact_mut<const M: usize>(&mut self) -> [&mut Self::PaddedArray<T, M, {N / M}>; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:
    {
        let mut ptr = self as *mut T;
        
        ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            slice
        })
    }
    
    #[inline]
    fn array_chunks<const M: usize>(self) -> ([Self::Array<T, M>; N / M], Self::Array<T, {N % M}>)
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    #[inline]
    fn array_chunks_ref<const M: usize>(&self) -> ([&Self::Array<T, M>; N / M], &Self::Array<T, {N % M}>)
    {
        let mut ptr = self as *const T;

        (ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            slice
        }), unsafe {core::mem::transmute(ptr)})
    }
    #[inline]
    fn array_chunks_mut<const M: usize>(&mut self) -> ([&mut Self::Array<T, M>; N / M], &mut Self::Array<T, {N % M}>)
    {
        let mut ptr = self as *mut T;

        (ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            slice
        }), unsafe {core::mem::transmute(ptr)})
    }

    #[inline]
    fn array_rchunks<const M: usize>(self) -> (Self::Array<T, {N % M}>, [Self::Array<T, M>; N / M])
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    #[inline]
    fn array_rchunks_ref<const M: usize>(&self) -> (&[T; N % M], [&[T; M]; N / M])
    {
        let start = self as *const T;
        let mut ptr = unsafe {start.add(N % M)};

        (unsafe {core::mem::transmute(start)}, ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            slice
        }))
    }
    #[inline]
    fn array_rchunks_mut<const M: usize>(&mut self) -> (&mut Self::Array<T, {N % M}>, [&mut Self::Array<T, M>; N / M])
    {
        let start = self as *mut T;
        let mut ptr = unsafe {start.add(N % M)};

        (unsafe {core::mem::transmute(start)}, ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            slice
        }))
    }
    
    #[inline]
    fn array_chunks_exact<const M: usize>(self) -> [Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    #[inline]
    fn array_chunks_exact_ref<const M: usize>(&self) -> [&Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        let mut ptr = self as *const T;

        ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            slice
        })
    }
    #[inline]
    fn array_chunks_exact_mut<const M: usize>(&mut self) -> [&mut Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        let mut ptr = self as *mut T;

        ArrayOps::fill(const |_| {
            let slice = unsafe {core::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            slice
        })
    }
    
    #[inline]
    fn split_array<const M: usize>(self) -> (Self::Array<T, M>, Self::Array<T, {N - M}>)
    where
        [(); N - M]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    #[inline]
    fn split_array_ref2<const M: usize>(&self) -> (&Self::Array<T, M>, &Self::Array<T, {N - M}>)
    where
        [(); N - M]:
    {
        let ptr = self as *const T;
        unsafe {(core::mem::transmute(ptr), core::mem::transmute(ptr.add(M)))}
    }
    #[inline]
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut Self::Array<T, M>, &mut Self::Array<T, {N - M}>)
    where
        [(); N - M]:
    {
        let ptr = self as *mut T;
        unsafe {(core::mem::transmute(ptr), core::mem::transmute(ptr.add(M)))}
    }
    
    #[inline]
    fn rsplit_array<const M: usize>(self) -> (Self::Array<T, {N - M}>, Self::Array<T, M>)
    where
        [(); N - M]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    #[inline]
    fn rsplit_array_mut2<const M: usize>(&mut self) -> (&mut Self::Array<T, {N - M}>, &mut Self::Array<T, M>)
    where
        [(); N - M]:
    {
        let ptr = self as *mut T;
        unsafe {(core::mem::transmute(ptr), core::mem::transmute(ptr.add(Self::LENGTH - M)))}
    }
    #[inline]
    fn rsplit_array_ref2<const M: usize>(&self) -> (&Self::Array<T, {N - M}>, &Self::Array<T, M>)
    where
        [(); N - M]:
    {
        let ptr = self as *const T;
        unsafe {(core::mem::transmute(ptr), core::mem::transmute(ptr.add(Self::LENGTH - M)))}
    }

    #[inline]
    fn each_ref2<B>(&self) -> Self::Array<&B, N>
    where
        T: ~const Borrow<B>
    {
        let mut ptr = self as *const T;
        ArrayOps::fill(const |_| {
            let y = unsafe {core::mem::transmute::<_, &T>(ptr)}.borrow();
            ptr = unsafe {ptr.add(1)};
            y
        })
    }
    #[inline]
    fn each_mut2<B>(&mut self) -> Self::Array<&mut B, N>
    where
        T: ~const BorrowMut<B>
    {
        let mut ptr = self as *mut T;
        ArrayOps::fill(const |_| {
            let y = unsafe {core::mem::transmute::<_, &mut T>(ptr)}.borrow_mut();
            ptr = unsafe {ptr.add(1)};
            y
        })
    }
}