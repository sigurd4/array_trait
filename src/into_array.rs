use slice_trait::IntoBoxedSlice;

use crate::AsArray;

#[const_trait]
pub trait IntoArray: ~const AsArray + IntoBoxedSlice
{
    /// Returns self as an array
    /// 
    /// Useful in the case where a trait is implemented using a generic bound to the [Array](Array) trait.
    /// In this case, the compiler does not automatically know that the type with the [Array](Array)-trait is an actual array.
    /// This method lets you tell the compiler that you are now working with an actual array, and not just something
    /// which implements the trait [Array](Array).
    fn into_array(self) -> [Self::Elem; Self::LENGTH];
}

impl<T, const LENGTH: usize> const IntoArray for [T; LENGTH]
{
    fn into_array(self) -> [Self::Elem; Self::LENGTH]
    {
        let array = unsafe {core::mem::transmute_copy(&self)};
        core::mem::forget(self);
        array
    }
}