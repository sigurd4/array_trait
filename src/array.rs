
use super::*;

/// A trait for any array, with item as an associated type, and length as an assiciated constant.
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
/// const fn first<'a, T: ~const Array>(array: &'a T) -> Option<&'a <T as AsSlice>::Item>
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
pub trait Array: private::Array + ArrayPrereq<<Self as AsSlice>::Item> + ~const AsArray + ~const IntoArray
/*where
    for<'a> &'a Self: TryFrom<&'a [Self::Item]>
        + IntoIterator<Item = &'a Self::Item, IntoIter = Iter<'a, Self::Item>>,
    for<'a> &'a mut Self: TryFrom<&'a mut [Self::Item]> + IntoIterator<Item = &'a mut Self::Item, IntoIter = IterMut<'a, Self::Item>>*/
{
    
}

impl<Item, const LENGTH: usize> const Array for [Item; LENGTH]
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
        const fn first<'a, T: ~const Array>(array: &'a T) -> Option<&'a <T as AsSlice>::Item>
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