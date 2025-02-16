use slice_trait::AsSlice;

#[const_trait]
pub trait AsArray: ~const AsSlice
{
    /// Length of array as compile-time constant
    /// 
    /// ## Example 1: Length
    /// [Array::LENGTH](Array::LENGTH) will always equal the actual length of the array.
    /// 
    /// ```rust
    /// use array_trait::*;
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
    /// use array_trait::*;
    /// 
    /// fn first_half<T: Array>(array: T) -> [<T as core::iter::IntoIterator>::Elem; T::LENGTH/2]
    /// {
    ///     array.into_iter().array_chunks().next().unwrap()
    /// }
    /// 
    /// assert_eq!(first_half([1.0, 2.0, 3.0, 4.0]), [1.0, 2.0]);
    /// ```
    const LENGTH: usize;

    /// Returns self as an array-slice
    /// 
    /// Similar to [Array::into_array](Array::into_array), but is passed by reference.
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn as_array(&self) -> &[Self::Elem; Self::LENGTH];
    
    /// Returns self as a mutable array-slice
    /// 
    /// Similar to [Array::into_array](Array::into_array), but is passed by mutable reference.
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn as_array_mut(&mut self) -> &mut [Self::Elem; Self::LENGTH];
}

impl<T, const LENGTH: usize> const AsArray for [T; LENGTH]
{
    const LENGTH: usize = LENGTH;

    fn as_array(&self) -> &[Self::Elem; Self::LENGTH]
    {
        unsafe {core::mem::transmute(self)}
    }
    fn as_array_mut(&mut self) -> &mut [Self::Elem; Self::LENGTH]
    {
        unsafe {core::mem::transmute(self)}
    }
}

/*#[cfg(feature = "alloc")]
impl<T, const N: usize> const AsArray for alloc::boxed::Box<T>
where
    T: ~const AsArray<LENGTH = {N}>,
    [(); T::LENGTH]:
{
    const LENGTH: usize = N;

    fn as_array(&self) -> &[Self::Elem; N]
    {
        let arr = (**self).as_array();
        unsafe {core::mem::transmute(arr)}
    }
    fn as_array_mut(&mut self) -> &mut [Self::Elem; N]
    {
        let arr = (**self).as_array_mut();
        unsafe {core::mem::transmute(arr)}
    }
}*/

#[cfg(feature = "alloc")]
impl<T, const N: usize> const AsArray for alloc::boxed::Box<[T; N]>
{
    const LENGTH: usize = N;

    fn as_array(&self) -> &[Self::Elem; Self::LENGTH]
    {
        let a: &[T; N] = self;
        unsafe {core::mem::transmute(a)}
    }
    fn as_array_mut(&mut self) -> &mut [Self::Elem; Self::LENGTH]
    {
        let a: &mut [T; N] = self;
        unsafe {core::mem::transmute(a)}
    }
}