use super::*;

pub trait ArrayNd<const DEPTH: usize>: Array
{
    const DIMENSIONS: [usize; DEPTH];
    const FLAT_LENGTH: usize;
    type ItemNd;
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
            type ItemNd = T;
        }
        $(impl_nd_array!($($b)+);)?
    };
}

mod r#impl
{
    use super::*;

    impl_nd_array!(
        _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16
    );
}
/*
 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 _29
        _30 _31 _32 _33 _34 _35 _36 _37 _38 _39 _40 _41 _42 _43 _44 _45 _46 _47 _48 _49 _50 _51 _52 _53 _54 _55 _56
        _57 _58 _59 _60 _61
*/