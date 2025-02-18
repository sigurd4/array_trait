use super::*;

/// A trait for N-dimensional arrays with depth up to 64.
/// 
/// The associated constants [DIMENSIONS](ArrayNd::DIMENSIONS) and [FLAT_LENGTH](ArrayNd::FLAT_LENGTH) vary depending on the chosen depth.
/// 
/// The assiciated type [ItemNd](ArrayNd::ItemNd) represents the innermost type given a chosen depth.
/// 
/// # Examples
/// 
/// ```rust
/// #![feature(generic_const_exprs)]
/// 
/// use array_trait::*;
/// 
/// type Mat2x3 = [[i8; 3]; 2];
/// 
/// /// The number of dimensions
/// const DEPTH: usize = 2;
/// 
/// // `FLAT_LENGTH` is the combined length if the N-dimensional array was flattened,
/// // i.e. the product of the lengths of each dimension.
/// assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::FLAT_LENGTH, 6);
/// 
/// // `DIMENSIONS` contains the lengths of each dimension ordered outermost to innermost.
/// assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::DIMENSIONS, [2, 3]);
/// ```
pub trait ArrayNd<const DEPTH: usize>: Array
{
    /// The dimensions of the n-dimensional containing the lengths of each level of array from outermost to innermost
    const DIMENSIONS: [usize; DEPTH];
    /// The product of the lengths of each dimension.
    const FLAT_LENGTH: usize;
    type ElemNd;
}

macro_rules! count {
    () => {0};
    ($a:ident) => {1};
    ($a:ident $($b:ident)+) => {1 $(+ count!($b))+};
}
macro_rules! flat_len {
    () => {0};
    ($a:ident $($b:ident)*) => {$a $(* $b)*}
}

macro_rules! nd {
    ($t:ty;) => {
        $t
    };
    ($t:ty; $a:ident) => {
        [$t; $a]
    };
    ($t:ty; $a:ident $($b:ident)+) => {
        [nd!{$t; $($b)+}; $a]
    };
}

macro_rules! impl_nd_array {
    ($a:ident $($($b:ident)+)?) => {
        impl<T, const $a: usize $($(, const $b: usize)+)?> /*const*/ ArrayNd<{count!{$a $($($b)+)?}}> for nd!{T; $a $($($b)+)?}
        {
            const DIMENSIONS: [usize; count!{$a $($($b)+)?}] = [$a $($(, $b)+)?];
            const FLAT_LENGTH: usize = flat_len!{$a $($($b)+)?};
            type ElemNd = T;
        }
        $(impl_nd_array!($($b)+);)?
    };
}

mod r#impl
{
    use super::*;

    impl_nd_array!(
        _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16
        _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 _29 _30 _31 _32
        _33 _34 _35 _36 _37 _38 _39 _40 _41 _42 _43 _44 _45 _46 _47 _48
        _49 _50 _51 _52 _53 _54 _55 _56 _57 _58 _59 _60 _61 _62 _63 _64
    );
}
/*
_65 _66 _67 _68 _69 _70 _71 _72 _73 _74 _75 _76 _77 _78 _79 _80
_81 _82 _83 _84 _85 _86 _87 _88 _89 _90 _91 _92 _93 _94 _95 _96
_97 _98 _99 _100 _101 _102 _103 _104 _105 _106 _107 _108 _109 _110 _111 _112
_113 _114 _115 _116 _117 _118 _119 _120 _121 _122 _123 _124 _125 _126 _127 _128
*/

#[cfg(test)]
mod test
{
    #[test]
    fn test()
    {
        use crate::*;

        type Mat2x3 = [[i8; 3]; 2];

        /// The number of dimensions
        const DEPTH: usize = 2;
        
        // `FLAT_LENGTH` is the combined length if the N-dimensional array was flattened
        assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::FLAT_LENGTH, 6);
        
        // `DIMENSIONS` contains the lengths of each dimension ordered outermost to innermost.
        assert_eq!(<Mat2x3 as ArrayNd<DEPTH>>::DIMENSIONS, [2, 3]);
    }
}