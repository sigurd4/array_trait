[![Build Status (nightly)](https://github.com/sigurd4/array_trait/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/array_trait/actions/workflows/build-nightly.yml)
<!--[![Build Status (nightly, all features)](https://github.com/sigurd4/array_trait/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/array_trait/actions/workflows/build-nightly-all-features.yml)-->

[![Build Status (stable)](https://github.com/sigurd4/array_trait/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/array_trait/actions/workflows/build-stable.yml)
<!--[![Build Status (stable, all features)](https://github.com/sigurd4/array_trait/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/array_trait/actions/workflows/build-stable-all-features.yml)-->

[![Test Status](https://github.com/sigurd4/array_trait/workflows/Test/badge.svg)](https://github.com/sigurd4/array_trait/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/array_trait/workflows/Lint/badge.svg)](https://github.com/sigurd4/array_trait/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/array_trait.svg)](https://crates.io/crates/array_trait)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/array_trait)](https://docs.rs/array_trait)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/array_trait)](https://app.codecov.io/github/sigurd4/array_trait)

# array_trait

A trait for any array, with item as an associated type, and length as an assiciated constant.

This crate is a subset of the crate [`array_ops`](https://crates.io/crates/array_ops).

## Examples

```rust
use array_trait::*;

type Arr3 = [i8; 3];

const A: Arr3 = [1, 2, 3];

// The assiciated constant `LENGTH` equals the length of the array
assert_eq!(Arr3::LENGTH, 3);
assert_eq!(Arr3::LENGTH, A.len());
```

```rust
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]

use array_trait::*;

type Arr3 = [i8; 3];

const A: Arr3 = [1, 2, 3];

/// The trait can be used in a function like this:
const fn first<'a, T: ~const Array>(array: &'a T) -> Option<&'a T::Item>
where
    [(); T::LENGTH]: // This is required for now.
{
    array.as_array().first()
}
assert_eq!(first(&A), Some(&1));
```

## N-dimensional arrays

There is also a trait for N-dimensional arrays, which contain information on its inner structure, and supports a depth up to 64 dimensions.

The associated constants `DIMENSIONS` and `FLAT_LENGTH` vary depending on the chosen depth.

 The assiciated type `ItemNd` represents the innermost type given a chosen depth.

## Examples

```rust
use array_trait::*;

type Mat2x3 = [[i8; 3]; 2];

/// The number of dimensions
const DEPTH: usize = 2;

// `FLAT_LENGTH` is the combined length if the N-dimensional array was flattened,
// i.e. the product of the lengths of each dimension.
assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::FLAT_LENGTH, 6);

// `DIMENSIONS` contains the lengths of each dimension ordered outermost to innermost.
assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::DIMENSIONS, [2, 3]);
```

## More operations

For more operations with arrays, using this as a basis, check out the crate [array__ops](https://www.crates.io/crates/array__ops).