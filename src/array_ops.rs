use core::{ops::{Sub, AddAssign, DerefMut, Mul, Div, Add}, mem::ManuallyDrop};

use core::ops::Deref;

use super::*;

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
    
    type Resized<const M: usize>: ~const ArrayOps<T, M> = Self::Array<T, M>;
    type Chained<const M: usize>: ~const ArrayOps<T, {N + M}> = Self::Array<T, {N + M}>
    where
        [(); N + M]:;
    type RChained<const M: usize>: ~const ArrayOps<T, {M + N}> = Self::Array<T, {M + N}>
    where
        [(); M + N]:;

    type MappedTo<M>: ~const ArrayOps<M, N> = Self::Array<M, N>;
    type Mapped<M>: ~const ArrayOps<M::Output, N> = Self::MappedTo<M::Output>
    where
        M: FnOnce<(T,)>;
    type Zipped<Z>: ~const ArrayOps<(T, Z), N> = Self::MappedTo<(T, Z)>;
    type Enumerated: ~const ArrayOps<(usize, T), N> = <Self::MappedTo<usize> as ArrayOps<usize, N>>::Zipped<T>;

    type Differentiated: ~const ArrayOps<<T as Sub<T>>::Output, {N.saturating_sub(1)}> = Self::Array<<T as Sub<T>>::Output, {N.saturating_sub(1)}>
    where
        [(); N.saturating_sub(1)]:,
        T: Sub<T>;

    type Split<const M: usize>: ~const ArrayOps<T, M> = Self::Array<T, M>
    where
        [(); N - M]:;
    type RSplit<const M: usize>: ~const ArrayOps<T, {N - M}> = Self::Array<T, {N - M}>
    where
        [(); N - M]:;

    fn split_len<const WRAPPING: bool>(n: usize) -> (usize, usize);
    fn rsplit_len<const WRAPPING: bool>(n: usize) -> (usize, usize);
        
    fn split_ptr<const WRAPPING: bool>(&self, n: usize) -> (*const T, *const T);
    fn split_mut_ptr<const WRAPPING: bool>(&mut self, n: usize) -> (*mut T, *mut T);

    fn rsplit_ptr<const WRAPPING: bool>(&self, n: usize) -> (*const T, *const T);
    fn rsplit_mut_ptr<const WRAPPING: bool>(&mut self, n: usize) -> (*mut T, *mut T);

    fn fill<F>(fill: F) -> Self
    where
        F: ~const FnMut(usize) -> T + ~const Destruct;
    fn rfill<F>(fill: F) -> Self
    where
        F: ~const FnMut(usize) -> T + ~const Destruct;
    
    fn truncate<const M: usize>(self) -> Self::Resized<M>
    where
        T: ~const Destruct,
        [(); N - M]:;
    fn rtruncate<const M: usize>(self) -> Self::Resized<M>
    where
        T: ~const Destruct,
        [(); N - M]:;

    fn resize<const M: usize, F>(self, fill: F) -> Self::Resized<M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;
    fn rresize<const M: usize, F>(self, fill: F) -> Self::Resized<M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;

    fn extend<const M: usize, F>(self, fill: F) -> Self::Resized<M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;
    fn rextend<const M: usize, F>(self, fill: F) -> Self::Resized<M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;

    fn reformulate_length<const M: usize>(self) -> Self::Resized<M>
    where
        [(); M - N]:,
        [(); N - M]:;
    
    fn reformulate_length_ref<const M: usize>(&self) -> &Self::Resized<M>
    where
        [(); M - N]:,
        [(); N - M]:;
        
    fn reformulate_length_mut<const M: usize>(&mut self) -> &mut Self::Resized<M>
    where
        [(); M - N]:,
        [(); N - M]:;
        
    fn try_reformulate_length<const M: usize>(self) -> Result<Self::Resized<M>, Self>;
    
    fn try_reformulate_length_ref<const M: usize>(&self) -> Option<&Self::Resized<M>>;
        
    fn try_reformulate_length_mut<const M: usize>(&mut self) -> Option<&mut Self::Resized<M>>;

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
    /// #![feature(const_deref)]
    /// 
    /// use core::{mem::ManuallyDrop, ops::DerefMut};
    /// use array_trait::*;
    /// 
    /// const A: [u8; 3] = [1, 2, 3];
    /// 
    /// const A_SUM: u8 = const {
    ///     let mut iter = ManuallyDrop::new(A.into_const_iter());
    ///     let mut sum = 0;
    /// 
    ///     while let Some(b) = iter.deref_mut().next()
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
    fn map2<M>(self, map: M) -> Self::Mapped<M>
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
    fn zip2<Z>(self, other: Self::Array<Z, N>) -> Self::Zipped<Z>;

    fn enumerate(self) -> Self::Enumerated;
    
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
    fn differentiate(self) -> Self::Differentiated
    where
        [(); N.saturating_sub(1)]:,
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
    fn integrate(self) -> Self
    where
        T: ~const AddAssign<T> + Copy;

    fn integrate_from<const M: usize>(self, x0: T) -> Self::Resized<M>
    where
        T: ~const AddAssign<T> + Copy;

    /// Reduces elements in array into one element, using a given operand
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use array_trait::ArrayOps;
    /// 
    /// const A: [u8; 3] = [1, 2, 3];
    /// 
    /// let r: u8 = A.reduce(|a, b| a + b).unwrap();
    /// 
    /// assert_eq!(r, 6);
    /// ```
    fn reduce<R>(self, reduce: R) -> Option<T>
    where
        R: ~const FnMut(T, T) -> T + ~const Destruct;
    
    fn sum(self) -> T
    where
        T: ~const Default + ~const AddAssign;
        
    fn sum_from(self, from: T) -> T
    where
        T: ~const AddAssign;

    fn max(self) -> Option<T>
    where
        T: ~const Ord;
        
    fn min(self) -> Option<T>
    where
        T: ~const Ord;
        
    fn all(&self) -> bool
    where
        T: ~const Into<bool> + Copy;
    fn any(&self) -> bool
    where
        T: ~const Into<bool> + Copy;

    fn eq2<Rhs>(&self, rhs: &[Rhs]) -> bool
    where
        T: ~const PartialEq<Rhs>;

    fn add_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Add<Rhs>>::Output>
    where
        T: ~const Add<Rhs>,
        Rhs: Copy;
    fn sub_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Sub<Rhs>>::Output>
    where
        T: ~const Sub<Rhs>,
        Rhs: Copy;
    fn mul_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Mul<Rhs>>::Output>
    where
        T: ~const Mul<Rhs>,
        Rhs: Copy;
    fn div_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Div<Rhs>>::Output>
    where
        T: ~const Div<Rhs>,
        Rhs: Copy;
    
    fn add_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Add<Rhs>>::Output>
    where
        T: ~const Add<Rhs>;
    fn sub_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Sub<Rhs>>::Output>
    where
        T: ~const Sub<Rhs>;
    fn mul_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Mul<Rhs>>::Output>
    where
        T: ~const Mul<Rhs>;
    fn div_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Div<Rhs>>::Output>
    where
        T: ~const Div<Rhs>;

    fn mul_dot<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> <T as Mul<Rhs>>::Output
    where
        T: ~const Mul<Rhs, Output: ~const AddAssign + ~const Default>;

    fn mul_dot_bias<Rhs>(self, rhs: Self::MappedTo<Rhs>, bias: <T as Mul<Rhs>>::Output) -> <T as Mul<Rhs>>::Output
    where
        T: ~const Mul<Rhs, Output: ~const AddAssign>;

    fn mul_outer<Rhs, const M: usize>(&self, rhs: &Self::Array<Rhs, M>) -> Self::MappedTo<Self::Array<<T as Mul<Rhs>>::Output, M>>
    where
        T: ~const Mul<Rhs> + Copy,
        Rhs: Copy;

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
    fn chain<const M: usize>(self, rhs: Self::Array<T, M>) -> Self::Resized<{N + M}>;

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
    fn rchain<const M: usize>(self, rhs: Self::Array<T, M>) -> Self::Resized<{N + M}>;
    
    fn into_rotate_left(self, n: usize) -> Self;

    fn into_rotate_right(self, n: usize) -> Self;

    fn into_shift_many_left<const M: usize>(self, items: [T; M]) -> ([T; M], Self);
        
    fn into_shift_many_right<const M: usize>(self, items: [T; M]) -> (Self, [T; M]);

    fn into_shift_left(self, item: T) -> (T, Self);
        
    fn into_shift_right(self, item: T) -> (Self, T);

    fn rotate_left2(&mut self, n: usize);

    fn rotate_right2(&mut self, n: usize);

    fn shift_many_left<const M: usize>(&mut self, items: [T; M]) -> [T; M];
    
    fn shift_many_right<const M: usize>(&mut self, items: [T; M]) -> [T; M];
    
    fn shift_left(&mut self, item: T) -> T;

    fn shift_right(&mut self, item: T) -> T;

    fn from_item(item: T) -> Self::Array<T, 1>;
    
    fn from_item_ref(item: &T) -> &Self::Array<T, 1>;

    fn from_item_mut(item: &mut T) -> &mut Self::Array<T, 1>;

    fn into_single_item(self) -> T
    where
        [(); 1 - N]:,
        [(); N - 1]:;
        
    fn try_into_single_item(self) -> Result<T, Self>;

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
        [(); M - 1]:;

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
    fn array_chunks_ref<const M: usize>(&self) -> (&[Self::Array<T, M>; N / M], &Self::Array<T, {N % M}>);
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
    fn array_chunks_mut<const M: usize>(&mut self) -> (&mut [Self::Array<T, M>; N / M], &mut Self::Array<T, {N % M}>);
    
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate mutable array-slice.
    fn array_rchunks<const M: usize>(self) -> (Self::Array<T, {N % M}>, [Self::Array<T, M>; N / M]);
    /// Divides an array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn array_rchunks_ref<const M: usize>(&self) -> (&Self::Array<T, {N % M}>, &[Self::Array<T, M>; N / M]);
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn array_rchunks_mut<const M: usize>(&mut self) -> (&mut Self::Array<T, {N % M}>, &mut [Self::Array<T, M>; N / M]);
    
    /// Divides an array into chunks, with no rest.
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
    /// let [lower_half, upper_half] = array.array_chunks_exact::<5>();
    /// 
    /// assert_eq!(lower_half, [0.0, 0.1, 0.2, 0.3, 0.4]);
    /// assert_eq!(upper_half, [0.5, 0.6, 0.7, 0.8, 0.9]);
    /// ```
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
    fn array_chunks_exact_ref<const M: usize>(&self) -> &[Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    fn array_chunks_exact_mut<const M: usize>(&mut self) -> &mut [Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;

    fn split_array_from<const M: usize>(array: Self::Array<T, {N + M}>) -> (Self::Array<T, M>, Self::Array<T, N>);
    
    fn split_array_from_ref<const M: usize>(array: &Self::Array<T, {N + M}>) -> (&Self::Array<T, M>, &Self::Array<T, N>);
    
    fn split_array_from_mut<const M: usize>(array: &mut Self::Array<T, {N + M}>) -> (&mut Self::Array<T, M>, &mut Self::Array<T, N>);
    
    fn rsplit_array_from<const M: usize>(array: Self::Array<T, {N + M}>) -> (Self::Array<T, N>, Self::Array<T, M>);
    
    fn rsplit_array_from_ref<const M: usize>(array: &Self::Array<T, {N + M}>) -> (&Self::Array<T, N>, &Self::Array<T, M>);
    
    fn rsplit_array_from_mut<const M: usize>(array: &mut Self::Array<T, {N + M}>) -> (&mut Self::Array<T, N>, &mut Self::Array<T, M>);

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
    fn split_len<const WRAPPING: bool>(n: usize) -> (usize, usize)
    {
        let left = if WRAPPING
        {
            n % N
        }
        else
        {
            assert!(n <= N);
            n
        };
        let right = N - left;
        (left, right)
    }
    #[inline]
    fn rsplit_len<const WRAPPING: bool>(n: usize) -> (usize, usize)
    {
        let (right, left) = Self::split_len::<WRAPPING>(n);
        (left, right)
    }
    
    #[inline]
    fn split_ptr<const WRAPPING: bool>(&self, n: usize) -> (*const T, *const T)
    {
        let ptr = self.as_ptr();
        (ptr, unsafe {ptr.add(Self::split_len::<WRAPPING>(n).0)})
    }
    #[inline]
    fn split_mut_ptr<const WRAPPING: bool>(&mut self, n: usize) -> (*mut T, *mut T)
    {
        let ptr = self.as_mut_ptr();
        (ptr, unsafe {ptr.add(Self::split_len::<WRAPPING>(n).0)})
    }

    #[inline]
    fn rsplit_ptr<const WRAPPING: bool>(&self, n: usize) -> (*const T, *const T)
    {
        let ptr = self.as_ptr();
        (ptr, unsafe {ptr.add(Self::rsplit_len::<WRAPPING>(n).0)})
    }
    #[inline]
    fn rsplit_mut_ptr<const WRAPPING: bool>(&mut self, n: usize) -> (*mut T, *mut T)
    {
        let ptr = self.as_mut_ptr();
        (ptr, unsafe {ptr.add(Self::rsplit_len::<WRAPPING>(n).0)})
    }

    #[inline]
    fn fill<F>(mut fill: F) -> Self
    where
        F: ~const FnMut(usize) -> T + ~const Destruct
    {
        let mut array = MaybeUninit::uninit_array();
        let mut i = 0;
        while i != N
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
        if N != 0
        {
            let mut i = N - 1;
            loop
            {
                array[i] = MaybeUninit::new(fill(i));
                if i == 0
                {
                    break
                }
                i -= 1;
            }
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
        let mut array = unsafe {MaybeUninit::array_assume_init(MaybeUninit::uninit_array())};
        let mut ptr = &mut array as *mut T;

        unsafe {core::ptr::copy_nonoverlapping(&self as *const T, ptr, N.min(M))};

        let mut i = N;
        ptr = unsafe {ptr.add(N)};
        while i < M
        {
            unsafe {core::ptr::write(ptr, fill(i))};
            i += 1;
            ptr = unsafe {ptr.add(1)};
        }
        array
    }
    #[inline]
    fn rresize<const M: usize, F>(self, mut fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct
    {
        let mut array = unsafe {MaybeUninit::array_assume_init(MaybeUninit::uninit_array())};
        let mut ptr = unsafe {(&mut array as *mut T).add(M.saturating_sub(N))};
        
        unsafe {core::ptr::copy_nonoverlapping(&self as *const T, ptr, N.min(M))};

        let mut i = M.saturating_sub(N);
        while i > 0
        {
            i -= 1;
            ptr = unsafe {ptr.sub(1)};
            unsafe {core::ptr::write(ptr, fill(i))};
        }

        array
    }

    #[inline]
    fn into_rotate_left(self, n: usize) -> Self
    {
        let mut rotated = MaybeUninit::<Self>::uninit();

        let (left, right) = Self::split_len::<true>(n);
        let (src_left, src_right) = self.split_ptr::<true>(n);

        unsafe {
            let (dst_left, dst_right) = rotated.assume_init_mut().rsplit_mut_ptr::<true>(n);

            core::ptr::copy_nonoverlapping(src_right, dst_left, right);
            core::ptr::copy_nonoverlapping(src_left, dst_right, left);
        }

        core::mem::forget(self);

        unsafe {
            MaybeUninit::assume_init(rotated)
        }
    }
    
    #[inline]
    fn into_rotate_right(self, n: usize) -> Self
    {
        let mut rotated = MaybeUninit::<Self>::uninit();

        let (left, right) = Self::rsplit_len::<true>(n);
        let (src_left, src_right) = self.rsplit_ptr::<true>(n);

        unsafe {
            let (dst_left, dst_right) = rotated.assume_init_mut().split_mut_ptr::<true>(n);

            core::ptr::copy_nonoverlapping(src_right, dst_left, right);
            core::ptr::copy_nonoverlapping(src_left, dst_right, left);
        }

        core::mem::forget(self);

        unsafe {
            MaybeUninit::assume_init(rotated)
        }
    }

    #[inline]
    fn into_shift_many_left<const M: usize>(self, items: [T; M]) -> ([T; M], Self)
    {
        unsafe {
            private::overlap_swap_transmute(self, items)
        }
    }

    #[inline]
    fn into_shift_many_right<const M: usize>(self, items: [T; M]) -> (Self, [T; M])
    {
        unsafe {
            private::overlap_swap_transmute(items, self)
        }
    }

    #[inline]
    fn into_shift_left(self, item: T) -> (T, Self)
    {
        unsafe {
            private::overlap_swap_transmute(self, item)
        }
    }

    #[inline]
    fn into_shift_right(self, item: T) -> (Self, T)
    {
        unsafe {
            private::overlap_swap_transmute(item, self)
        }
    }

    #[inline]
    fn rotate_left2(&mut self, n: usize)
    {
        unsafe {
            let mut buffer: Self = private::uninit();

            let (left, right) = Self::split_len::<true>(n);
            let (src_left, src_right) = buffer.split_mut_ptr::<true>(n);
            let (dst_left, dst_right) = self.rsplit_mut_ptr::<true>(n);

            core::ptr::copy_nonoverlapping(
                dst_left,
                src_left,
                N
            );
            core::ptr::copy_nonoverlapping(
                src_right,
                dst_left,
                right
            );
            core::ptr::copy_nonoverlapping(
                src_left,
                dst_right,
                left
            );
            core::mem::forget(buffer);
        }
    }

    #[inline]
    fn rotate_right2(&mut self, n: usize)
    {
        unsafe {
            let mut buffer: Self = private::uninit();

            let (left, right) = Self::rsplit_len::<true>(n);
            let (src_left, src_right) = buffer.rsplit_mut_ptr::<true>(n);
            let (dst_left, dst_right) = self.split_mut_ptr::<true>(n);

            core::ptr::copy_nonoverlapping(
                dst_left,
                src_left,
                N
            );
            core::ptr::copy_nonoverlapping(
                src_right,
                dst_left,
                right
            );
            core::ptr::copy_nonoverlapping(
                src_left,
                dst_right,
                left
            );
            core::mem::forget(buffer);
        }
    }

    #[inline]
    fn shift_many_left<const M: usize>(&mut self, items: [T; M]) -> [T; M]
    {
        unsafe {
            let mut buffer: private::Pair<[T; M], Self> = private::Pair::new(items, private::uninit());
            let buf_left = buffer.left.as_mut_ptr();
            let buf_right = buf_left.add(N);

            core::ptr::copy_nonoverlapping(buffer.left.as_ptr(), buf_right, M);
            core::ptr::copy_nonoverlapping(self.as_ptr(), buf_left, N);

            let (overflow, shifted) = buffer.unpack_mandrop();

            core::ptr::copy_nonoverlapping(shifted.as_ptr(), self.as_mut_ptr(), N);
            core::mem::forget(shifted);

            ManuallyDrop::into_inner(overflow)
        }
    }

    #[inline]
    fn shift_many_right<const M: usize>(&mut self, items: [T; M]) -> [T; M]
    {
        unsafe {
            let mut buffer: private::Pair<Self, [T; M]> = private::Pair::new(private::uninit(), items);
            let buf_left = buffer.left.as_mut_ptr();
            let buf_right = buf_left.add(M);

            core::ptr::copy_nonoverlapping(buffer.right.as_ptr(), buf_left, M);
            core::ptr::copy_nonoverlapping(self.as_ptr(), buf_right, N);

            let (shifted, overflow) = buffer.unpack_mandrop();

            core::ptr::copy_nonoverlapping(shifted.as_ptr(), self.as_mut_ptr(), N);
            core::mem::forget(shifted);

            ManuallyDrop::into_inner(overflow)
        }
    }
    
    #[inline]
    fn shift_left(&mut self, item: T) -> T
    {
        unsafe {
            let mut buffer: private::Pair<T, Self> = private::Pair::new(item, private::uninit());
            let buf_left = &mut buffer.left as *mut T;
            let buf_right = buf_left.add(N);

            core::ptr::copy_nonoverlapping(&buffer.left as *const T, buf_right, 1);
            core::ptr::copy_nonoverlapping(self.as_ptr(), buf_left, N);

            let (overflow, shifted) = buffer.unpack_mandrop();

            core::ptr::copy_nonoverlapping(shifted.as_ptr(), self.as_mut_ptr(), N);
            core::mem::forget(shifted);

            ManuallyDrop::into_inner(overflow)
        }
    }

    #[inline]
    fn shift_right(&mut self, item: T) -> T
    {
        unsafe {
            let mut buffer: private::Pair<Self, T> = private::Pair::new(private::uninit(), item);
            let buf_left = buffer.left.as_mut_ptr();
            let buf_right = buf_left.add(1);

            core::ptr::copy_nonoverlapping(&buffer.right as *const T, buf_left, 1);
            core::ptr::copy_nonoverlapping(self.as_ptr(), buf_right, N);

            let (shifted, overflow) = buffer.unpack_mandrop();

            core::ptr::copy_nonoverlapping(shifted.as_ptr(), self.as_mut_ptr(), N);
            core::mem::forget(shifted);

            ManuallyDrop::into_inner(overflow)
        }
    }
    
    #[inline]
    fn from_item(item: T) -> Self::Array<T, 1>
    {
        [item]
    }
    
    #[inline]
    fn from_item_ref(item: &T) -> &Self::Array<T, 1>
    {
        unsafe {core::mem::transmute(item)}
    }

    #[inline]
    fn from_item_mut(item: &mut T) -> &mut Self::Array<T, 1>
    {
        unsafe {core::mem::transmute(item)}
    }
    
    #[inline]
    fn into_single_item(self) -> T
    where
        [(); 1 - N]:,
        [(); N - 1]:
    {
        let item = unsafe {core::ptr::read(self.as_ptr())};
        core::mem::forget(self);
        item
    }
    
    #[inline]
    fn try_into_single_item(self) -> Result<T, Self>
    {
        if N == 1
        {
            let item = unsafe {core::ptr::read(self.as_ptr())};
            core::mem::forget(self);
            Ok(item)
        }
        else
        {
            Err(self)
        }
    }
    
    #[inline]
    fn extend<const M: usize, F>(self, mut fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = ArrayOps::fill(const move |i| fill(i + N));
        unsafe {private::merge_transmute(self, filled)}
    }
    #[inline]
    fn rextend<const M: usize, F>(self, fill: F) -> Self::Array<T, M>
    where
        F: ~const FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = ArrayOps::fill(fill);
        unsafe {private::merge_transmute(filled, self)}
    }
    
    #[inline]
    fn reformulate_length<const M: usize>(self) -> Self::Array<T, M>
    where
        [(); M - N]:,
        [(); N - M]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    
    #[inline]
    fn reformulate_length_ref<const M: usize>(&self) -> &Self::Array<T, M>
    where
        [(); M - N]:,
        [(); N - M]:
    {
        unsafe {core::mem::transmute(self)}
    }
        
    #[inline]
    fn reformulate_length_mut<const M: usize>(&mut self) -> &mut Self::Array<T, M>
    where
        [(); M - N]:,
        [(); N - M]:
    {
        unsafe {core::mem::transmute(self)}
    }
    
    #[inline]
    fn try_reformulate_length<const M: usize>(self) -> Result<Self::Array<T, M>, Self>
    {
        if N == M
        {
            Ok(unsafe {private::transmute_unchecked_size(self)})
        }
        else
        {
            Err(self)
        }
    }
    
    #[inline]
    fn try_reformulate_length_ref<const M: usize>(&self) -> Option<&Self::Array<T, M>>
    {
        if N == M
        {
            Some(unsafe {core::mem::transmute(self)})
        }
        else
        {
            None
        }
    }
        
    #[inline]
    fn try_reformulate_length_mut<const M: usize>(&mut self) -> Option<&mut Self::Array<T, M>>
    {
        if N == M
        {
            Some(unsafe {core::mem::transmute(self)})
        }
        else
        {
            None
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
    fn map2<M>(self, mut map: M) -> Self::Mapped<M>
    where
        M: ~const FnMut<(T,)> + ~const Destruct
    {
        let mut iter = ManuallyDrop::new(self.into_const_iter());
        ArrayOps::fill(const |_| map(iter.deref_mut().next().unwrap()))
    }
    
    #[inline]
    fn zip2<Z>(self, other: Self::Array<Z, N>) -> Self::Zipped<Z>
    {
        let mut iter_self = ManuallyDrop::new(self.into_const_iter());
        let mut iter_other = ManuallyDrop::new(other.into_const_iter());
        ArrayOps::fill(const |_| (iter_self.deref_mut().next().unwrap(), iter_other.deref_mut().next().unwrap()))
    }
    
    #[inline]
    fn enumerate(self) -> Self::Enumerated
    {
        let mut iter_self = ManuallyDrop::new(self.into_const_iter());
        ArrayOps::fill(const |i| (i, iter_self.deref_mut().next().unwrap()))
    }

    #[inline]
    fn differentiate(self) -> Self::Differentiated
    where
        [(); N.saturating_sub(1)]:,
        T: ~const Sub<T> + Copy + ~const Destruct
    {
        let mut iter_self = ManuallyDrop::new(self.into_const_iter());
        if let Some(mut x0) = iter_self.deref_mut().next()
        {
            ArrayOps::fill(const move |_| {
                let x = iter_self.deref_mut().next().unwrap();
                let dx = x - x0;
                x0 = x;
                dx
            })
        }
        else
        {
            unsafe {private::transmute_unchecked_size::<[<T as Sub<T>>::Output; 0], _>([])}
        }
    }

    #[inline]
    fn integrate(self) -> Self
        where
            T: ~const AddAssign<T> + Copy + ~const Destruct
    {
        let mut iter_self = ManuallyDrop::new(self.into_const_iter());
        if let Some(mut x_accum) = iter_self.deref_mut().next()
        {
            ArrayOps::fill(const move |_| {
                let xi = x_accum;
                if let Some(x) = iter_self.deref_mut().next()
                {
                    x_accum += x;
                }
                xi
            })
        }
        else
        {
            // Return empty array
            unsafe {MaybeUninit::assume_init(MaybeUninit::uninit())}
        }
    }
    
    #[inline]
    fn integrate_from<const M: usize>(self, x0: T) -> Self::Resized<M>
    where
        T: ~const AddAssign<T> + Copy + ~const Destruct
    {
        let mut iter_self = ManuallyDrop::new(self.into_const_iter());

        let mut x_accum = x0;

        ArrayOps::fill(const move |_| {
            let xi = x_accum;
            if let Some(x) = iter_self.deref_mut().next()
            {
                x_accum += x;
            }
            xi
        })
    }

    #[inline]
    fn reduce<R>(self, mut reduce: R) -> Option<T>
    where
        R: ~const FnMut(T, T) -> T + ~const Destruct
    {
        let this = ManuallyDrop::new(self);
        if N == 0
        {
            return None
        }
        let mut ptr = this.deref() as *const T;
        let mut i = 1;
        unsafe {
            let mut reduction = core::ptr::read(ptr);
            while i < N
            {
                ptr = ptr.add(1);
                reduction = reduce(reduction, core::ptr::read(ptr));
                i += 1;
            }
            Some(reduction)
        }
    }
    
    fn sum(self) -> T
    where
        T: ~const Default + ~const AddAssign
    {
        let mut iter = ManuallyDrop::new(self.into_const_iter());

        let mut next = ManuallyDrop::new(iter.deref_mut().next());
        let mut reduction = if next.deref().is_some()
        {
            ManuallyDrop::into_inner(next).unwrap()
        }
        else
        {
            return Default::default()
        };
        while {
            next = ManuallyDrop::new(iter.deref_mut().next());
            next.deref().is_some()
        }
        {
            let x = unsafe {ManuallyDrop::into_inner(next).unwrap_unchecked()};
            reduction += x;
        }
        reduction
    }

    #[inline]
    fn sum_from(self, from: T) -> T
    where
        T: ~const AddAssign
    {
        let mut iter = ManuallyDrop::new(self.into_const_iter());

        let mut next;
        let mut reduction = from;
        while {
            next = ManuallyDrop::new(iter.deref_mut().next());
            next.deref().is_some()
        }
        {
            let x = unsafe {ManuallyDrop::into_inner(next).unwrap_unchecked()};
            reduction += x;
        }
        reduction
    }
    
    #[inline]
    fn max(self) -> Option<T>
    where
        T: ~const Ord
    {
        self.reduce(T::max)
    }
        
    #[inline]
    fn min(self) -> Option<T>
    where
        T: ~const Ord
    {
        self.reduce(T::min)
    }

    #[inline]
    fn all(&self) -> bool
    where
        T: ~const Into<bool> + Copy
    {
        let mut iter = ManuallyDrop::new(self.const_iter());

        while let Some(next) = iter.next()
        {
            if !(*next).into()
            {
                return false
            }
        }

        true
    }
    
    #[inline]
    fn any(&self) -> bool
    where
        T: ~const Into<bool> + Copy
    {
        let mut iter = ManuallyDrop::new(self.const_iter());

        while let Some(next) = iter.next()
        {
            if (*next).into()
            {
                return true
            }
        }

        false
    }
    
    #[inline]
    fn eq2<Rhs>(&self, rhs: &[Rhs]) -> bool
    where
        T: ~const PartialEq<Rhs>
    {
        if N != rhs.len()
        {
            return false;
        }

        let mut iter_self = ManuallyDrop::new(self.const_iter());

        while let Some((i, next_self)) = iter_self.next_enumerated()
        {
            let next_rhs = &rhs[i];

            if next_self != next_rhs
            {
                return false
            }
        }
        
        true
    }
    
    fn add_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Add<Rhs>>::Output>
    where
        T: ~const Add<Rhs>,
        Rhs: Copy
    {
        self.map2(const |x| x + rhs)
    }
    fn sub_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Sub<Rhs>>::Output>
    where
        T: ~const Sub<Rhs>,
        Rhs: Copy
    {
        self.map2(const |x| x - rhs)
    }
    fn mul_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Mul<Rhs>>::Output>
    where
        T: ~const Mul<Rhs>,
        Rhs: Copy
    {
        self.map2(const |x| x*rhs)
    }
    fn div_all<Rhs>(self, rhs: Rhs) -> Self::MappedTo<<T as Div<Rhs>>::Output>
    where
        T: ~const Div<Rhs>,
        Rhs: Copy
    {
        self.map2(const |x| x/rhs)
    }
    
    fn add_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Add<Rhs>>::Output>
    where
        T: ~const Add<Rhs>
    {
        self.zip2(rhs).map2(const |(x, rhs)| x + rhs)
    }
    fn sub_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Sub<Rhs>>::Output>
    where
        T: ~const Sub<Rhs>
    {
        self.zip2(rhs).map2(const |(x, rhs)| x - rhs)
    }
    fn mul_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Mul<Rhs>>::Output>
    where
        T: ~const Mul<Rhs>
    {
        self.zip2(rhs).map2(const |(x, rhs)| x*rhs)
    }
    fn div_each<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> Self::MappedTo<<T as Div<Rhs>>::Output>
    where
        T: ~const Div<Rhs>
    {
        self.zip2(rhs).map2(const |(x, rhs)| x/rhs)
    }

    fn mul_dot<Rhs>(self, rhs: Self::MappedTo<Rhs>) -> <T as Mul<Rhs>>::Output
    where
        T: ~const Mul<Rhs, Output: ~const AddAssign + ~const Default>
    {
        self.mul_each(rhs).sum()
    }
    
    fn mul_dot_bias<Rhs>(self, rhs: Self::MappedTo<Rhs>, bias: <T as Mul<Rhs>>::Output) -> <T as Mul<Rhs>>::Output
    where
        T: ~const Mul<Rhs, Output: ~const AddAssign>
    {
        self.mul_each(rhs).sum_from(bias)
    }

    fn mul_outer<Rhs, const M: usize>(&self, rhs: &Self::Array<Rhs, M>) -> Self::MappedTo<Self::Array<<T as Mul<Rhs>>::Output, M>>
    where
        T: ~const Mul<Rhs> + Copy,
        Rhs: Copy
    {
        self.map2(const |u| rhs.map2(const |v| u*v))
    }
    
    #[inline]
    fn chain<const M: usize>(self, rhs: Self::Array<T, M>) -> Self::Array<T, {N + M}>
    {
        unsafe {private::merge_transmute(self, rhs)}
    }
    
    #[inline]
    fn rchain<const M: usize>(self, rhs: Self::Array<T, M>) -> Self::Array<T, {N + M}>
    {
        unsafe {private::merge_transmute(rhs, self)}
    }
    
    #[inline]
    fn array_spread<const M: usize>(self) -> ([Self::Array<T, {N / M}>; M], Self::Array<T, {N % M}>)
    where
        [(); M - 1]:
    {
        let split = self.array_chunks();

        let spread_t = unsafe {core::ptr::read(&split.0 as *const [[T; _]; _])};
        let rest = unsafe {core::ptr::read(&split.1 as *const [T; _])};
        core::mem::forget(split);

        (spread_t.transpose(), rest)
    }
    #[inline]
    fn array_spread_ref<const M: usize>(&self) -> ([&Self::PaddedArray<T, M, {N / M}>; M], &Self::Array<T, {N % M}>)
    where
        [(); M - 1]:
    {
        let (mut left, right) = self.rsplit_ptr::<false>(N % M);

        unsafe {(
            ArrayOps::fill(const |_| {
                let slice = core::mem::transmute(left);
                left = left.add(1);
                slice
            }),
            core::mem::transmute(right)
        )}
    }
    #[inline]
    fn array_spread_mut<const M: usize>(&mut self) -> ([&mut Self::PaddedArray<T, M, {N / M}>; M], &mut Self::Array<T, {N % M}>)
    where
        [(); M - 1]:
    {
        let (mut left, right) = self.rsplit_mut_ptr::<false>(N % M);

        unsafe {(
            ArrayOps::fill(const |_| {
                let slice = core::mem::transmute(left);
                left = left.add(1);
                slice
            }),
            core::mem::transmute(right)
        )}
    }
    
    #[inline]
    fn array_rspread<const M: usize>(self) -> ([T; N % M], [Self::Array<T, {N / M}>; M])
    where
        [(); M - 1]:
    {
        let split = self.array_rchunks();
        
        let start = unsafe {core::ptr::read(&split.0 as *const [T; _])};
        let spread_t = unsafe {core::ptr::read(&split.1 as *const [[T; _]; _])};
        core::mem::forget(split);

        (start, spread_t.transpose())
    }
    #[inline]
    fn array_rspread_ref<const M: usize>(&self) -> (&[T; N % M], [&Self::PaddedArray<T, M, {N / M}>; M])
    where
        [(); M - 1]:
    {
        let (left, mut right) = self.split_ptr::<false>(N % M);

        unsafe {(
            core::mem::transmute(left),
            ArrayOps::fill(const |_| {
                let slice = core::mem::transmute(right);
                right = right.add(1);
                slice
            })
        )}
    }
    #[inline]
    fn array_rspread_mut<const M: usize>(&mut self) -> (&mut [T; N % M], [&mut Self::PaddedArray<T, M, {N / M}>; M])
    where
        [(); M - 1]:
    {
        let (left, mut right) = self.split_mut_ptr::<false>(N % M);

        unsafe {(
            core::mem::transmute(left),
            ArrayOps::fill(const |_| {
                let slice = core::mem::transmute(right);
                right = right.add(1);
                slice
            })
        )}
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
        unsafe {private::split_transmute(self)}
    }
    #[inline]
    fn array_chunks_ref<const M: usize>(&self) -> (&[Self::Array<T, M>; N / M], &Self::Array<T, {N % M}>)
    {
        let (left, right) = self.rsplit_ptr::<false>(N % M);
        unsafe {(core::mem::transmute(left), core::mem::transmute(right))}
    }
    #[inline]
    fn array_chunks_mut<const M: usize>(&mut self) -> (&mut [Self::Array<T, M>; N / M], &mut Self::Array<T, {N % M}>)
    {
        let (left, right) = self.rsplit_mut_ptr::<false>(N % M);
        unsafe {(core::mem::transmute(left), core::mem::transmute(right))}
    }

    #[inline]
    fn array_rchunks<const M: usize>(self) -> (Self::Array<T, {N % M}>, [Self::Array<T, M>; N / M])
    {
        unsafe {private::split_transmute(self)}
    }
    #[inline]
    fn array_rchunks_ref<const M: usize>(&self) -> (&[T; N % M], &[[T; M]; N / M])
    {
        let (left, right) = self.split_ptr::<false>(N % M);
        unsafe {(core::mem::transmute(left), core::mem::transmute(right))}
    }
    #[inline]
    fn array_rchunks_mut<const M: usize>(&mut self) -> (&mut Self::Array<T, {N % M}>, &mut [Self::Array<T, M>; N / M])
    {
        let (left, right) = self.split_mut_ptr::<false>(N % M);
        unsafe {(core::mem::transmute(left), core::mem::transmute(right))}
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
    fn array_chunks_exact_ref<const M: usize>(&self) -> &[Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {core::mem::transmute(self as *const T)}
    }
    #[inline]
    fn array_chunks_exact_mut<const M: usize>(&mut self) -> &mut [Self::Array<T, M>; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {core::mem::transmute(self as *mut T)}
    }
    
    #[inline]
    fn split_array_from<const M: usize>(array: Self::Array<T, {N + M}>) -> (Self::Array<T, M>, Self::Array<T, N>)
    {
        unsafe {private::split_transmute(array)}
    }
    
    #[inline]
    fn split_array_from_ref<const M: usize>(array: &Self::Array<T, {N + M}>) -> (&Self::Array<T, M>, &Self::Array<T, N>)
    {
        let (ptr_left, ptr_right) = array.split_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
    }
    
    #[inline]
    fn split_array_from_mut<const M: usize>(array: &mut Self::Array<T, {N + M}>) -> (&mut Self::Array<T, M>, &mut Self::Array<T, N>)
    {
        let (ptr_left, ptr_right) = array.split_mut_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
    }
    
    #[inline]
    fn rsplit_array_from<const M: usize>(array: Self::Array<T, {N + M}>) -> (Self::Array<T, N>, Self::Array<T, M>)
    {
        unsafe {private::split_transmute(array)}
    }
    
    #[inline]
    fn rsplit_array_from_ref<const M: usize>(array: &Self::Array<T, {N + M}>) -> (&Self::Array<T, N>, &Self::Array<T, M>)
    {
        let (ptr_left, ptr_right) = array.rsplit_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
    }
    
    #[inline]
    fn rsplit_array_from_mut<const M: usize>(array: &mut Self::Array<T, {N + M}>) -> (&mut Self::Array<T, N>, &mut Self::Array<T, M>)
    {
        let (ptr_left, ptr_right) = array.rsplit_mut_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
    }
    
    #[inline]
    fn split_array<const M: usize>(self) -> (Self::Array<T, M>, Self::Array<T, {N - M}>)
    where
        [(); N - M]:
    {
        unsafe {private::split_transmute(self)}
    }
    #[inline]
    fn split_array_ref2<const M: usize>(&self) -> (&Self::Array<T, M>, &Self::Array<T, {N - M}>)
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.split_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
    }
    #[inline]
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut Self::Array<T, M>, &mut Self::Array<T, {N - M}>)
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.split_mut_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
    }
    
    #[inline]
    fn rsplit_array<const M: usize>(self) -> (Self::Array<T, {N - M}>, Self::Array<T, M>)
    where
        [(); N - M]:
    {
        unsafe {private::split_transmute(self)}
    }
    #[inline]
    fn rsplit_array_mut2<const M: usize>(&mut self) -> (&mut Self::Array<T, {N - M}>, &mut Self::Array<T, M>)
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.rsplit_mut_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
    }
    #[inline]
    fn rsplit_array_ref2<const M: usize>(&self) -> (&Self::Array<T, {N - M}>, &Self::Array<T, M>)
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.rsplit_ptr::<false>(M);
        unsafe {(core::mem::transmute(ptr_left), core::mem::transmute(ptr_right))}
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