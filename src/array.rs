
use core::ptr::Pointee;

use super::*;

/// A trait for any array, with elem as an associated type, and length as an assiciated constant.
///
/// # Example
///
/// ```rust
/// #![feature(const_trait_impl)]
/// #![feature(generic_const_exprs)]
///
/// use array_trait::*;
/// 
/// type Arr3 = [i8; 3];
/// 
/// const A: Arr3 = [1, 2, 3];
/// 
/// /// The trait can be used in a function like this:
/// const fn first<'a, T: ~const Array>(array: &'a T) -> Option<&'a <T as AsSlice>::Elem>
/// where
///     [(); T::LENGTH]: // This is required for now.
/// {
///     array.as_array().first()
/// }
/// assert_eq!(first(&A), Some(&1));
/// 
/// // The assiciated constant `LENGTH` equals the length of the array
/// assert_eq!(Arr3::LENGTH, 3);
/// assert_eq!(Arr3::LENGTH, A.len());
/// ```
#[const_trait]
pub trait Array: private::Array + ArrayPrereq<<Self as AsSlice>::Elem> + ~const AsArray + ~const IntoArray + Pointee<Metadata = ()>
/*where
    for<'a> &'a Self: TryFrom<&'a [Self::Elem]>
        + IntoIterator<Elem = &'a Self::Elem, IntoIter = Iter<'a, Self::Elem>>,
    for<'a> &'a mut Self: TryFrom<&'a mut [Self::Elem]> + IntoIterator<Elem = &'a mut Self::Elem, IntoIter = IterMut<'a, Self::Elem>>*/
{
    
}

impl<T, const LENGTH: usize> const Array for [T; LENGTH]
{
    
}

#[cfg(test)]
mod test
{
    #[test]
    fn it_works()
    {
        use crate::*;

        type Arr3 = [i8; 3];
        
        const A: Arr3 = [1, 2, 3];

        /// The trait can be used in a function like this:
        const fn first<'a, T: ~const Array>(array: &'a T) -> Option<&'a <T as AsSlice>::Elem>
        where
            [(); T::LENGTH]: // This is required for now.
        {
            array.as_array().first()
        }
        assert_eq!(first(&A), Some(&1));
        
        // The assiciated constant `LENGTH` equals the length of the array
        assert_eq!(Arr3::LENGTH, 3);
    }
}