#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(const_maybe_uninit_assume_init)]
#![feature(const_maybe_uninit_array_assume_init)]
#![feature(const_swap)]
#![feature(associated_const_equality)]
#![feature(generic_arg_infer)]
#![feature(const_closures)]
#![feature(const_option)]

moddef::pub_flat_mods!(
    padded

    const_iterator
    into_const_iter
    const_iter
    const_iter_mut
);

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Index, Range, RangeInclusive, RangeFrom, RangeTo, RangeToInclusive, IndexMut, RangeFull},
    mem::MaybeUninit, marker::Destruct
};

mod private
{
    use std::mem::{ManuallyDrop, MaybeUninit};

    #[const_trait]
    pub trait Array {}
    impl<Item, const LENGTH: usize> const Array for [Item; LENGTH] {}
    
    union Transmutation<A, B>
    {
        a: ManuallyDrop<A>,
        b: ManuallyDrop<B>
    }
    //#[deprecated]
    #[inline]
    pub(crate) const unsafe fn transmute_unchecked_size_manually_drop<A, B>(from: ManuallyDrop<A>) -> ManuallyDrop<B>
    {
        let t = Transmutation
        {
            a: from
        };
        unsafe {t.b}
    }
    //#[deprecated]
    #[inline]
    pub(crate) const unsafe fn transmute_unchecked_size<A, B>(from: A) -> B
    {
        ManuallyDrop::into_inner(transmute_unchecked_size_manually_drop(ManuallyDrop::new(from)))
    }
    
    pub(crate) const fn transpose_matrix<T, const H: usize, const W: usize>(matrix: [[T; W]; H])
        -> [[T; H]; W]
    {
        let mut matrix: [[MaybeUninit<T>; W]; H] = unsafe {
            transmute_unchecked_size(matrix)
        };
        let mut matrix_t: [[MaybeUninit<T>; H]; W] = unsafe {
            transmute_unchecked_size(MaybeUninit::<[T; H]>::uninit_array::<W>())
        };
        let mut i = 0;
        while i < H
        {
            let mut j = 0;
            while j < W
            {
                std::mem::swap(&mut matrix_t[j][i], &mut matrix[i][j]);
                j += 1;
            }
            i += 1;
        }
        unsafe {
            transmute_unchecked_size(matrix_t)
        }
    }
}

#[const_trait]
pub trait Array: private::Array
    + Sized
    + IntoIterator//<IntoIter = std::array::IntoIter<<Self as IntoIterator>::Item, {Self::_LENGTH}>>
    + AsRef<[Self::Item]>
    + AsMut<[Self::Item]>
    + Borrow<[Self::Item]>
    + BorrowMut<[Self::Item]>
    + ~const Index<usize, Output = <[Self::Item] as Index<usize>>::Output>
    + ~const Index<Range<usize>, Output = <[Self::Item] as Index<Range<usize>>>::Output>
    + ~const Index<RangeInclusive<usize>, Output = <[Self::Item] as Index<RangeInclusive<usize>>>::Output>
    + ~const Index<RangeFrom<usize>, Output = <[Self::Item] as Index<RangeFrom<usize>>>::Output>
    + ~const Index<RangeTo<usize>, Output = <[Self::Item] as Index<RangeTo<usize>>>::Output>
    + ~const Index<RangeToInclusive<usize>, Output = <[Self::Item] as Index<RangeToInclusive<usize>>>::Output>
    + ~const Index<RangeFull, Output = <[Self::Item] as Index<RangeFull>>::Output>
    + ~const IndexMut<usize>
    + ~const IndexMut<Range<usize>>
    + ~const IndexMut<RangeInclusive<usize>>
    + ~const IndexMut<RangeFrom<usize>>
    + ~const IndexMut<RangeTo<usize>>
    + ~const IndexMut<RangeToInclusive<usize>>
    + ~const IndexMut<RangeFull>
/*where
    for<'a> &'a Self: TryFrom<&'a [Self::Item]>
        + IntoIterator<Item = &'a Self::Item, IntoIter = Iter<'a, Self::Item>>,
    for<'a> &'a mut Self: TryFrom<&'a mut [Self::Item]> + IntoIterator<Item = &'a mut Self::Item, IntoIter = IterMut<'a, Self::Item>>*/
{
    /// Length of array as compile-time constant
    /// 
    /// ## Example 1: Length
    /// [Array::LENGTH](Array::LENGTH) will always equal the actual length of the array.
    /// 
    /// ```rust
    /// use array_trait::Array;
    /// 
    /// const L: usize = 4;
    /// 
    /// let array: [f32; L] = [1.0, 2.0, 3.0, 4.0];
    /// 
    /// assert_eq!(<[f32; L]>::LENGTH, L);
    /// assert_eq!(<[f32; L]>::LENGTH, array.len());
    /// ```
    /// 
    /// ## Example 2: Generic const-expression usage
    /// This can be used in const-expressions as shown below.
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(iter_array_chunks)]
    /// 
    /// use array_trait::Array;
    /// 
    /// fn first_half<T: Array>(array: T) -> [T::Item; T::LENGTH/2]
    /// {
    ///     array.into_iter().array_chunks().next().unwrap()
    /// }
    /// 
    /// assert_eq!(first_half([1.0, 2.0, 3.0, 4.0]), [1.0, 2.0]);
    /// ```
    const LENGTH: usize;

    fn from_const_fn(fill: impl ~const FnMut(usize) -> Self::Item + ~const Destruct) -> Self;

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
    /// use array_trait::Array;
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
    #[inline]
    fn into_const_iter<const N: usize>(self) -> IntoConstIter<Self::Item, N>
    where
        Self: Array<LENGTH = {N}>
    {
        IntoConstIter::from(self.into_array())
    }
    /// Makes a const iterator over the array-slice.
    /// 
    /// The const iterator does not implement [std::iter::Iterator](Iterator), and as such is more limited in its usage.
    /// However it can be used at compile-time.
    #[inline]
    fn const_iter<const N: usize>(&self) -> ConstIter<'_, Self::Item, N>
    where
        Self: Array<LENGTH = {N}>
    {
        ConstIter::from(self.as_array())
    }
    /// Makes a mutable const iterator over the mutable array-slice.
    /// 
    /// The const iterator does not implement [std::iter::Iterator](Iterator), and as such is more limited in its usage.
    /// However it can be used at compile-time.
    #[inline]
    fn const_iter_mut<const N: usize>(&mut self) -> ConstIterMut<'_, Self::Item, N>
    where
        Self: Array<LENGTH = {N}>
    {
        ConstIterMut::from(self.as_array_mut())
    }

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
    /// use array_trait::Array;
    /// 
    /// const A: [u8; 4] = [1, 2, 3, 4];
    /// const B: [i8; 4] = A.const_map(const |b| -(b as i8));
    /// 
    /// assert_eq!(B, [-1, -2, -3, -4]);
    /// ```
    #[inline]
    fn const_map<T, const N: usize>(self, mut map: impl ~const FnMut(Self::Item) -> T + ~const Destruct) -> [T; N]
    where
        Self: Array<LENGTH = {N}>
    {
        let mut iter = self.into_const_iter();
        <[T; N]>::from_const_fn(const |_| map(iter.next().unwrap()))
    }

    /// Returns self as an array
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn into_array<const N: usize>(self) -> [Self::Item; N]
    where
        Self: Array<LENGTH = {N}>;

    /// Returns self as an array-slice
    /// 
    /// Similar to [Array::into_array](Array::into_array), but is passed by reference.
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn as_array<const N: usize>(&self) -> &[Self::Item; N]
    where
        Self: Array<LENGTH = {N}>;
    
    /// Returns self as a mutable array-slice
    /// 
    /// Similar to [Array::into_array](Array::into_array), but is passed by mutable reference.
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn as_array_mut<const N: usize>(&mut self) -> &mut [Self::Item; N]
    where
        Self: Array<LENGTH = {N}>;

    /// Chains two arrays with the same item together.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array_trait::Array;
    /// 
    /// let a = ["one", "two"];
    /// let b = ["three"];
    /// 
    /// assert_eq!(a.chain(b), ["one", "two", "three"]);
    /// ```
    #[inline]
    fn chain<const M: usize, const N: usize>(self, rhs: [Self::Item; M]) -> [Self::Item; N + M]
    where
        Self: Array<LENGTH = {N}>
    {
        unsafe {private::transmute_unchecked_size((self.into_array(), rhs))}
    }

    /// Distributes items of an array equally across a given width, then provides the rest as a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::Array;
    /// 
    /// let array = ["ping 1", "pong 1", "ping 2", "pong 2", "ping 3", "pong 3", "uhh..."];
    /// 
    /// let ([ping, pong], rest) = array.spread_into::<2, _>();
    /// 
    /// assert_eq!(ping, ["ping 1", "ping 2", "ping 3"]);
    /// assert_eq!(pong, ["pong 1", "pong 2", "pong 3"]);
    /// assert_eq!(rest, ["uhh..."]);
    /// ```
    #[inline]
    fn spread_into<const M: usize, const N: usize>(self) -> ([[Self::Item; N / M]; M], [Self::Item; N % M])
    where
        [(); M - 1]:,
        Self: Array<LENGTH = {N}>,
        Self::Item: Copy
    {
        let (spread_t, rest): ([[Self::Item; M]; N / M], [Self::Item; N % M]) = unsafe {
            private::transmute_unchecked_size(self.into_array())
        };
        (private::transpose_matrix(spread_t), rest)
    }

    /// Distributes items of an array-slice equally across a given width, then provides the rest as a separate array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    fn as_spread<const M: usize, const N: usize>(&self)
        -> ([&[Padded<Self::Item, M>; N / M]; M], &[Self::Item; N % M])
    where
        [(); M - 1]:,
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array() as *const Self::Item;
        let mut ptr = start;
        let mut spread = MaybeUninit::uninit_array();

        let mut i = 0;
        while i < M
        {
            spread[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            i += 1;
        }

        (unsafe {MaybeUninit::array_assume_init(spread)}, unsafe {std::mem::transmute(start.add((Self::LENGTH / M) * M))})
    }
    
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
    /// use array_trait::Array;
    /// 
    /// let mut array = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20"];
    /// 
    /// let (threes, _) = array.as_spread_mut::<3, _>();
    /// 
    /// for fizz in threes.into_iter().last().unwrap()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let (fives, _) = array.as_spread_mut::<5, _>();
    /// 
    /// for buzz in fives.into_iter().last().unwrap()
    /// {
    ///     **buzz = "buzz";
    /// }
    /// 
    /// let (fifteens, _) = array.as_spread_mut::<15, _>();
    /// 
    /// for fizzbuzz in fifteens.into_iter().last().unwrap()
    /// {
    ///     **fizzbuzz = "fizzbuzz";
    /// }
    /// 
    /// assert_eq!(array, ["1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14", "fizzbuzz", "16", "17", "fizz", "19", "buzz"]);
    /// 
    /// ```
    fn as_spread_mut<const M: usize, const N: usize>(&mut self)
        -> ([&mut [Padded<Self::Item, M>; N / M]; M], &mut [Self::Item; N % M])
    where
        [(); M - 1]:,
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array_mut() as *mut Self::Item;
        let mut ptr = start;
        let mut spread = MaybeUninit::uninit_array();

        let mut i = 0;
        while i < M
        {
            spread[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            i += 1;
        }

        (unsafe {MaybeUninit::array_assume_init(spread)}, unsafe {std::mem::transmute(start.add((Self::LENGTH / M) * M))})
    }
    
    /// Distributes items of an array equally across a given width, then provides the leftmost rest as a separate array.
    #[inline]
    fn rspread_into<const M: usize, const N: usize>(self) -> ([Self::Item; N % M], [[Self::Item; N / M]; M])
    where
        [(); M - 1]:,
        Self: Array<LENGTH = {N}>,
        Self::Item: Copy
    {
        let (start, spread_t): ([Self::Item; N % M], [[Self::Item; M]; N / M]) = unsafe {
            private::transmute_unchecked_size(self.into_array())
        };
        (start, private::transpose_matrix(spread_t))
    }

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
    /// use array_trait::Array;
    /// 
    /// let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    /// 
    /// let (zero, [odd, even]) = array.as_rspread::<2, _>();
    /// 
    /// assert_eq!(*zero, [0]);
    /// assert_eq!(odd.each_ref().map(|padding| **padding), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    /// assert_eq!(even.each_ref().map(|padding| **padding), [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    /// ```
    fn as_rspread<const M: usize, const N: usize>(&self)
        -> (&[Self::Item; N % M], [&[Padded<Self::Item, M>; N / M]; M])
    where
        [(); M - 1]:,
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array() as *const Self::Item;
        let mut ptr = unsafe {start.add(Self::LENGTH % M)};
        let mut spread = MaybeUninit::uninit_array();

        let mut i = 0;
        while i < M
        {
            spread[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            i += 1;
        }

        (unsafe {std::mem::transmute(start)}, unsafe {MaybeUninit::array_assume_init(spread)})
    }
    
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
    /// use array_trait::Array;
    /// 
    /// let mut array = ["the", "beat", "goes", "1", "2", "3", "4", "5", "6", "7", "8"];
    /// 
    /// let (start, [boots, n, cats, and]) = array.as_rspread_mut::<4, _>();
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
    fn as_rspread_mut<const M: usize, const N: usize>(&mut self)
        -> (&mut [Self::Item; N % M], [&mut [Padded<Self::Item, M>; N / M]; M])
    where
        [(); M - 1]:,
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array_mut() as *mut Self::Item;
        let mut ptr = unsafe {start.add(Self::LENGTH % M)};
        let mut spread = MaybeUninit::uninit_array();

        let mut i = 0;
        while i < M
        {
            spread[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            i += 1;
        }

        (unsafe {std::mem::transmute(start)}, unsafe {MaybeUninit::array_assume_init(spread)})
    }
    
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
    /// use array_trait::Array;
    /// 
    /// let array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.spread_into_exact::<2, _>();
    /// 
    /// assert_eq!(lower_case, *b"abcdefghijklmnopqrstuvwxyz");
    /// assert_eq!(upper_case, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// ```
    #[inline]
    fn spread_into_exact<const M: usize, const N: usize>(self) -> [[Self::Item; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:,
        Self: Array<LENGTH = {N}>
    {
        let spread_t: [[Self::Item; M]; N / M] = unsafe {
            private::transmute_unchecked_size(self.into_array())
        };
        private::transpose_matrix(spread_t)
    }
    
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
    /// use array_trait::Array;
    /// 
    /// let statement = ["s", "he", "be", "lie", "ve", "d"];
    /// 
    /// let [interpretation2, interpretation1] = statement.as_spread_exact::<2, _>();
    /// 
    /// assert_eq!(interpretation1.each_ref().map(|padding| &**padding), ["he", "lie", "d"].each_ref());
    /// assert_eq!(interpretation2.each_ref().map(|padding| &**padding), ["s", "be", "ve"].each_ref());
    /// ```
    fn as_spread_exact<const M: usize, const N: usize>(&self) -> [&[Padded<Self::Item, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:,
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array() as *const Self::Item;
        let mut ptr = start;
        let mut spread = MaybeUninit::uninit_array();

        let mut i = 0;
        while i < M
        {
            spread[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            i += 1;
        }

        unsafe {MaybeUninit::array_assume_init(spread)}
    }

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
    /// use array_trait::Array;
    /// 
    /// let mut array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.as_spread_exact_mut::<2, _>();
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
    fn as_spread_exact_mut<const M: usize, const N: usize>(&mut self) -> [&mut [Padded<Self::Item, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:,
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array_mut() as *mut Self::Item;
        let mut ptr = start;
        let mut spread = MaybeUninit::uninit_array();

        let mut i = 0;
        while i < M
        {
            spread[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(1)};
            i += 1;
        }

        unsafe {MaybeUninit::array_assume_init(spread)}
    }

    /// Divides an array into chunks, then yielding the rest in a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::Array;
    /// 
    /// let array = ["carrot", "potato", "beet", "tomato", "kiwi", "banana", "cherry", "peach", "strawberry", "nine volt batteries"];
    /// 
    /// let ([root_vegetables, technically_berries, stone_fruits], not_for_human_consumption) = array.into_chunks::<3, _>();
    /// 
    /// assert_eq!(root_vegetables, ["carrot", "potato", "beet"]);
    /// assert_eq!(technically_berries, ["tomato", "kiwi", "banana"]);
    /// assert_eq!(stone_fruits, ["cherry", "peach", "strawberry"]);
    /// assert_eq!(not_for_human_consumption, ["nine volt batteries"]);
    /// ```
    #[inline]
    fn into_chunks<const M: usize, const N: usize>(self) -> ([[Self::Item; M]; N / M], [Self::Item; N % M])
    where
        Self: Array<LENGTH = {N}>
    {
        unsafe {private::transmute_unchecked_size(self.into_array())}
    }

    /// Divides an array-slice into chunks, then yielding the rest in a separate array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::Array;
    /// 
    /// let transistors = ["2N3904", "2N2222A", "BC107", "AC127", "OC7", "NKT275", "2SK30A", "2N5458", "J108", "2N7000", "BS170"];
    /// 
    /// let ([silicon_bjts, germanium_bjts, jfets], mosfets) = transistors.as_chunks::<3, _>();
    /// 
    /// assert_eq!(silicon_bjts, &["2N3904", "2N2222A", "BC107"]);
    /// assert_eq!(germanium_bjts, &["AC127", "OC7", "NKT275"]);
    /// assert_eq!(jfets, &["2SK30A", "2N5458", "J108"]);
    /// assert_eq!(mosfets, &["2N7000", "BS170"]);
    /// ```
    fn as_chunks<const M: usize, const N: usize>(&self) -> ([&[Self::Item; M]; N / M], &[Self::Item; N % M])
    where
        Self: Array<LENGTH = {N}>
    {
        let mut ptr = self.as_array() as *const Self::Item;
        let mut chunks = MaybeUninit::uninit_array();
        
        let mut i = 0;
        while i < Self::LENGTH / M
        {
            chunks[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            i += 1;
        }

        (unsafe {MaybeUninit::array_assume_init(chunks)}, unsafe {std::mem::transmute(ptr)})
    }
    
    /// Divides a mutable array-slice into chunks, then yielding the rest in a separate mutable array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array_trait::Array;
    /// 
    /// let mut array = [0, 1, 0, 1, 0, 1, 6];
    /// 
    /// let (pairs, last) = array.as_chunks_mut::<2, _>();
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
    fn as_chunks_mut<const M: usize, const N: usize>(&mut self)
        -> ([&mut [Self::Item; M]; N / M], &mut [Self::Item; N % M])
    where
        Self: Array<LENGTH = {N}>
    {
        let mut ptr = self.as_array_mut() as *mut Self::Item;
        let mut chunks = MaybeUninit::uninit_array();
        
        let mut i = 0;
        while i < Self::LENGTH / M
        {
            chunks[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            i += 1;
        }

        (unsafe {MaybeUninit::array_assume_init(chunks)}, unsafe {std::mem::transmute(ptr)})
    }
    
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate mutable array-slice.
    #[inline]
    fn into_rchunks<const M: usize, const N: usize>(self) -> ([Self::Item; N % M], [[Self::Item; M]; N / M])
    where
        Self: Array<LENGTH = {N}>
    {
        unsafe {private::transmute_unchecked_size(self.into_array())}
    }

    /// Divides an array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn as_rchunks<const M: usize, const N: usize>(&self) -> (&[Self::Item; N % M], [&[Self::Item; M]; N / M])
    where
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array() as *const Self::Item;
        let mut ptr = unsafe {start.add(Self::LENGTH % M)};
        let mut chunks = MaybeUninit::uninit_array();
        
        let mut i = 0;
        while i < Self::LENGTH / M
        {
            chunks[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            i += 1;
        }

        (unsafe {std::mem::transmute(start)}, unsafe {MaybeUninit::array_assume_init(chunks)})
    }

    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn as_rchunks_mut<const M: usize, const N: usize>(&mut self) -> (&mut [Self::Item; N % M], [&mut [Self::Item; M]; N / M])
    where
        Self: Array<LENGTH = {N}>
    {
        let start = self.as_array_mut() as *mut Self::Item;
        let mut ptr = unsafe {start.add(Self::LENGTH % M)};
        let mut chunks = MaybeUninit::uninit_array();
        
        let mut i = 0;
        while i < Self::LENGTH / M
        {
            chunks[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            i += 1;
        }

        (unsafe {std::mem::transmute(start)}, unsafe {MaybeUninit::array_assume_init(chunks)})
    }
    
    /// Divides an array into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    #[inline]
    fn into_chunks_exact<const M: usize, const N: usize>(self) -> [[Self::Item; M]; N / M]
    where
        [(); 0 - N % M]:,
        Self: Array<LENGTH = {N}>
    {
        unsafe {private::transmute_unchecked_size(self.into_array())}
    }

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
    /// use array_trait::Array;
    /// 
    /// let array = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    /// 
    /// let [lower_half, upper_half] = array.as_chunks_exact::<5, _>();
    /// 
    /// assert_eq!(lower_half, &[0.0, 0.1, 0.2, 0.3, 0.4]);
    /// assert_eq!(upper_half, &[0.5, 0.6, 0.7, 0.8, 0.9]);
    /// ```
    fn as_chunks_exact<const M: usize, const N: usize>(&self) -> [&[Self::Item; M]; N / M]
    where
        [(); 0 - Self::LENGTH % M]:,
        Self: Array<LENGTH = {N}>
    {
        let mut ptr = self.as_array() as *const Self::Item;
        let mut chunks = MaybeUninit::uninit_array();
        
        let mut i = 0;
        while i < Self::LENGTH / M
        {
            chunks[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            i += 1;
        }

        unsafe {MaybeUninit::array_assume_init(chunks)}
    }

    /// Divides a mutable array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    fn as_chunks_exact_mut<const M: usize, const N: usize>(&mut self) -> [&mut [Self::Item; M]; N / M]
    where
        [(); 0 - Self::LENGTH % M]:,
        Self: Array<LENGTH = {N}>
    {
        let mut ptr = self.as_array_mut() as *mut Self::Item;
        let mut chunks = MaybeUninit::uninit_array();
        
        let mut i = 0;
        while i < Self::LENGTH / M
        {
            chunks[i] = unsafe {std::mem::transmute(ptr)};
            ptr = unsafe {ptr.add(M)};
            i += 1;
        }

        unsafe {MaybeUninit::array_assume_init(chunks)}
    }

    /// Splits an array at a chosen index.
    #[inline]
    fn split_array<const M: usize, const N: usize>(self) -> ([Self::Item; M], [Self::Item; N - M])
    where
        Self: Array<LENGTH = {N}>
    {
        unsafe {private::transmute_unchecked_size(self.into_array())}
    }
    /// Splits an array at a chosen index as array-slices.
    #[inline]
    fn split_array_ref2<const M: usize, const N: usize>(&self) -> (&[Self::Item; M], &[Self::Item; N - M])
    where
        Self: Array<LENGTH = {N}>
    {
        let ptr = self.as_array() as *const Self::Item;
        unsafe {(std::mem::transmute(ptr), std::mem::transmute(ptr.add(M)))}
    }
    /// Splits an array at a chosen index as mutable array-slices.
    #[inline]
    fn split_array_mut2<const M: usize, const N: usize>(&mut self) -> (&mut [Self::Item; M], &mut [Self::Item; N - M])
    where
        Self: Array<LENGTH = {N}>
    {
        let ptr = self.as_array_mut() as *mut Self::Item;
        unsafe {(std::mem::transmute(ptr), std::mem::transmute(ptr.add(M)))}
    }
    
    /// Splits an array at a chosen index, where the index goes from right to left.
    #[inline]
    fn rsplit_array<const M: usize, const N: usize>(self) -> ([Self::Item; Self::LENGTH - M], [Self::Item; M])
    where
        Self: Array<LENGTH = {N}>
    {
        unsafe {private::transmute_unchecked_size(self.into_array())}
    }
    /// Splits an array at a chosen index as array-slices, where the index goes from right to left.
    #[inline]
    fn rsplit_array_ref2<const M: usize, const N: usize>(&self) -> (&[Self::Item; N - M], &[Self::Item; M])
    where
        Self: Array<LENGTH = {N}>
    {
        let ptr = self.as_array() as *const Self::Item;
        unsafe {(std::mem::transmute(ptr), std::mem::transmute(ptr.add(Self::LENGTH - M)))}
    }
    /// Splits an array at a chosen index as mutable array-slices, where the index goes from right to left.
    #[inline]
    fn rsplit_array_mut2<const M: usize, const N: usize>(&mut self) -> (&mut [Self::Item; N - M], &mut [Self::Item; M])
    where
        Self: Array<LENGTH = {N}>
    {
        let ptr = self.as_array_mut() as *mut Self::Item;
        unsafe {(std::mem::transmute(ptr), std::mem::transmute(ptr.add(Self::LENGTH - M)))}
    }

    /// Transposes a two-dimensional array (as if it were a matrix)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array_trait::Array;
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
    #[inline]
    fn transpose<const M: usize, const N: usize>(self) -> [[<Self::Item as IntoIterator>::Item; N]; M]
    where
        Self::Item: Array<LENGTH = {M}>,
        Self: Array<LENGTH = {N}>
    {
        private::transpose_matrix(unsafe {private::transmute_unchecked_size(self.into_array())})
    }
}

impl<Item, const LENGTH: usize> const Array for [Item; LENGTH]
{
    const LENGTH: usize = LENGTH;

    #[inline]
    fn from_const_fn(mut fill: impl ~const FnMut(usize) -> Self::Item + ~const Destruct) -> Self
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
    fn into_array<const N: usize>(self) -> [Self::Item;  N]
    where
        Self: Array<LENGTH = {N}>
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    #[inline]
    fn as_array<const N: usize>(&self) -> &[Self::Item; N]
    where
        Self: Array<LENGTH = {N}>
    {
        unsafe {std::mem::transmute(self)}
    }
    #[inline]
    fn as_array_mut<const N: usize>(&mut self) -> &mut [Self::Item; N]
    where
        Self: Array<LENGTH = {N}>
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
        let str = b"abcdefghijklmnopqrstuvwxyz".map(|c| (c as char).to_string());
        
        println!("str: {:?}", str);
        println!("spread: {:?}", str.as_spread::<3, _>());
        println!("chunks: {:?}", str.as_chunks::<3, _>());
    }
}