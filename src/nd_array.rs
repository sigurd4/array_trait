use super::*;

#[const_trait]
pub trait ArrayNd<const N: usize>: private::Array + ArrayPrereq
{
    const DIMENSIONS: [usize; N];
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
    ($t:ty; $($n:ident)+, $a:ident) => {
        [$t; $a]
    };
    ($t:ty; $($n:ident)+, $a:ident $($b:ident)+) => {
        [nd!{$t; $($b)+}; $a]
    };
    ($t:ty; $($n:ident)+) => {
        nd!($t; $($n)+, $($n)+)
    };
}

macro_rules! fill_nd {
    (($fill:ident, $i:ident, $array:ident)) => {
        core::mem::swap($array, &mut MaybeUninit::new($fill($i)));
    };
    (($fill:ident, $i:ident, $array:ident) $a:ident $($b:ident)*) => {
        const J: usize = count!($($b)*);
        $i[J] = 0;
        while $i[J] < $a
        {
            let array = &mut $array[$i[J]];
            fill_nd!(($fill, $i, array) $($b)*);
            $i[J] += 1;
        }
    };
}

macro_rules! impl_nd_array {
    ($a:ident $($($b:ident)+)?) => {
        impl<T, const $a: usize $($(, const $b: usize)+)?> const ArrayNd<{count!{$a $($($b)+)?}}> for nd!{T; $a $($($b)+)?}
        {
            const DIMENSIONS: [usize; count!{$a $($($b)+)?}] = [$a $($(, $b)+)?];
            const FLAT_LENGTH: usize = flat_len!{$a $($($b)+)?};
            type ItemNd = T;
        }
        impl<T, const $a: usize $($(, const $b: usize)+)?> const ArrayNdOps<{count!{$a $($($b)+)?}}, T, {flat_len!{$a $($($b)+)?}}> for nd!{T; $a $($($b)+)?}
        {
            type Mapped<M> = nd!{M; $a $($($b)+)?};

            fn fill_nd<F>(mut fill: F) -> Self
            where
                F: ~const FnMut([usize; count!{$a $($($b)+)?}]) -> T + ~const Destruct
            {
                let mut i = [0; {count!{$a $($($b)+)?}}];
                let mut array: nd!{MaybeUninit<T>; $a $($($b)+)?} =
                    unsafe {private::transmute_unchecked_size(MaybeUninit::<nd!{MaybeUninit<T>; $($($b)+)?}>::uninit_array::<$a>())};
                while i[count!{$($($b)+)?}] < $a
                {
                    let array = &mut array[i[count!($($($b)*)?)]];
                    fill_nd!((fill, i, array) $($($b)+)?);
                    i[count!{$($($b)+)?}] += 1;
                }
                unsafe {private::transmute_unchecked_size(array)}
            }

            fn map_nd<M>(self, mut map: M) -> Self::Mapped<<M as FnOnce<(T,)>>::Output>
            where
                M: ~const FnMut<(T,)> + ~const Destruct
            {
                let mut iter = self.flatten_nd_array().into_const_iter();
                ArrayNdOps::fill_nd(const |_| map(iter.next().unwrap()))
            }
            
            fn enumerate_nd(self) -> Self::Mapped<([usize; {count!{$a $($($b)+)?}}], T)>
            {
                let mut iter = self.flatten_nd_array().into_const_iter();
                ArrayNdOps::fill_nd(const |i| (i, iter.next().unwrap()))
            }

            fn flatten_nd_array(self) -> [T; {flat_len!{$a $($($b)+)?}}]
            {
                unsafe {private::transmute_unchecked_size(self)}
            }

            fn flatten_nd_array_ref(&self) -> &[T; {flat_len!{$a $($($b)+)?}}]
            {
                unsafe {core::mem::transmute(self)}
            }

            fn flatten_nd_array_mut(&mut self) -> &mut [T; {flat_len!{$a $($($b)+)?}}]
            {
                unsafe {core::mem::transmute(self)}
            }
        }
        $(impl_nd_array!($($b)+);)?
    };
}
impl_nd_array!(
    _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 _29
    _30 _31 _32 _33 _34 _35 _36 _37 _38 _39 _40 _41 _42 _43 _44 _45 _46 _47 _48 _49 _50 _51 _52 _53 _54 _55 _56
    _57 _58 _59 _60 _61
);