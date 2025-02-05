#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(trait_alias)]
#![feature(associated_const_equality)]
#![feature(generic_const_exprs)]
#![recursion_limit = "256"]

//! A trait for any array, with item as an associated type, and length as an assiciated constant.
//!
//! This crate is a subset of the crate [`array_ops`](https://crates.io/crates/array_ops).
//!
//! # Examples
//!
//! ```rust
//! use array_trait::*;
//!
//! type Arr3 = [i8; 3];
//!
//! const A: Arr3 = [1, 2, 3];
//!
//! // The assiciated constant `LENGTH` equals the length of the array
//! assert_eq!(Arr3::LENGTH, 3);
//! assert_eq!(Arr3::LENGTH, A.len());
//! ```
//!
//! ```rust
//! #![feature(const_trait_impl)]
//! #![feature(generic_const_exprs)]
//!
//! use array_trait::*;
//!
//! type Arr3 = [i8; 3];
//!
//! const A: Arr3 = [1, 2, 3];
//!
//! /// The trait can be used in a function like this:
//! const fn first<'a, T: ~const Array>(array: &'a T) -> Option<&'a <T as AsSlice>::Item>
//! where
//!     [(); T::LENGTH]: // This is required for now.
//! {
//!     array.as_array().first()
//! }
//! assert_eq!(first(&A), Some(&1));
//! ```
//!
//! # N-dimensional arrays
//!
//! There is also a trait for N-dimensional arrays, which contain information on its inner structure, and supports a depth up to 64 dimensions.
//!
//! The associated constants [DIMENSIONS](ArrayNd::DIMENSIONS) and [FLAT_LENGTH](ArrayNd::FLAT_LENGTH) vary depending on the chosen depth.
//!
//!  The assiciated type [ItemNd](ArrayNd::ItemNd) represents the innermost type given a chosen depth.
//!
//! # Examples
//!
//! ```rust
//! #![feature(generic_const_exprs)]
//!
//! use array_trait::*;
//!
//! type Mat2x3 = [[i8; 3]; 2];
//!
//! /// The number of dimensions
//! const DEPTH: usize = 2;
//!
//! // `FLAT_LENGTH` is the combined length if the N-dimensional array was flattened,
//! // i.e. the product of the lengths of each dimension.
//! assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::FLAT_LENGTH, 6);
//!
//! // `DIMENSIONS` contains the lengths of each dimension ordered outermost to innermost.
//! assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::DIMENSIONS, [2, 3]);
//! ```

moddef::moddef!(
    flat(pub) mod {
        array,
        array_nd,
        into_array,
        as_array,
        prereq
    }
);

#[cfg(feature = "alloc")]
extern crate alloc;

pub use slice_trait::*;

mod private
{
    use crate::AsArray;

    pub trait Array: AsArray {}
    impl<Item, const LENGTH: usize> Array for [Item; LENGTH] {}
}

#[cfg(test)]
mod test
{
    #[test]
    fn test() {}
}