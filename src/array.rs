
use super::*;

#[const_trait]
pub trait Array: private::Array + ArrayPrereq
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
    
    /// Returns self as an array
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn into_array(self) -> [Self::Item; Self::LENGTH];

    /// Returns self as an array-slice
    /// 
    /// Similar to [Array::into_array](Array::into_array), but is passed by reference.
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn as_array(&self) -> &[Self::Item; Self::LENGTH];
    
    /// Returns self as a mutable array-slice
    /// 
    /// Similar to [Array::into_array](Array::into_array), but is passed by mutable reference.
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn as_array_mut(&mut self) -> &mut [Self::Item; Self::LENGTH];
}

impl<Item, const LENGTH: usize> const Array for [Item; LENGTH]
{
    const LENGTH: usize = LENGTH;

    fn into_array(self) -> [Self::Item; Self::LENGTH]
    {
        let array = unsafe {core::mem::transmute_copy(&self)};
        core::mem::forget(self);
        array
    }
    fn as_array(&self) -> &[Self::Item; Self::LENGTH]
    {
        unsafe {core::mem::transmute(self)}
    }
    fn as_array_mut(&mut self) -> &mut [Self::Item; Self::LENGTH]
    {
        unsafe {core::mem::transmute(self)}
    }
}