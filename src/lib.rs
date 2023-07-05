#![cfg_attr(not(test), no_std)]

#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(const_maybe_uninit_assume_init)]
#![feature(const_maybe_uninit_array_assume_init)]
#![feature(const_swap)]
#![feature(associated_const_equality)]
#![feature(const_closures)]
#![feature(const_option)]
#![feature(associated_type_defaults)]
#![feature(trait_alias)]
#![feature(unboxed_closures)]
#![feature(concat_idents)]
#![feature(decl_macro)]
#![feature(generic_arg_infer)]

moddef::moddef!(
    flat(pub) mod {
        array,
        nd_array,

        array_ops,
        array_nd_ops,
        array_2d_ops,

        padded,
    
        const_iterator,
        into_const_iter,
        const_iter,
        const_iter_mut
    }
);

use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Index, Range, RangeInclusive, RangeFrom, RangeTo, RangeToInclusive, IndexMut, RangeFull},
    mem::MaybeUninit, marker::Destruct
};

mod private
{
    use core::mem::ManuallyDrop;

    #[const_trait]
    pub trait Array {}
    impl<Item, const LENGTH: usize> const Array for [Item; LENGTH] {}
    
    union Transmutation<A, B>
    {
        a: ManuallyDrop<A>,
        b: ManuallyDrop<B>
    }
    //#[deprecated]
    #[inline]
    pub(crate) const unsafe fn transmute_unchecked_size<A, B>(from: A) -> B
    {
        #[cfg(test)]
        if core::mem::size_of::<A>() != core::mem::size_of::<B>()
        {
            panic!("Cannot transmute due to unequal size")
        }
        ManuallyDrop::into_inner(unsafe {Transmutation {a: ManuallyDrop::new(from)}.b})
    }
}

pub trait ArrayPrereq = Sized
+ IntoIterator
+ AsRef<[<Self as IntoIterator>::Item]>
+ AsMut<[<Self as IntoIterator>::Item]>
+ Borrow<[<Self as IntoIterator>::Item]>
+ BorrowMut<[<Self as IntoIterator>::Item]>
+ ~const Index<usize, Output = <[<Self as IntoIterator>::Item] as Index<usize>>::Output>
+ ~const Index<Range<usize>, Output = <[<Self as IntoIterator>::Item] as Index<Range<usize>>>::Output>
+ ~const Index<RangeInclusive<usize>, Output = <[<Self as IntoIterator>::Item] as Index<RangeInclusive<usize>>>::Output>
+ ~const Index<RangeFrom<usize>, Output = <[<Self as IntoIterator>::Item] as Index<RangeFrom<usize>>>::Output>
+ ~const Index<RangeTo<usize>, Output = <[<Self as IntoIterator>::Item] as Index<RangeTo<usize>>>::Output>
+ ~const Index<RangeToInclusive<usize>, Output = <[<Self as IntoIterator>::Item] as Index<RangeToInclusive<usize>>>::Output>
+ ~const Index<RangeFull, Output = <[<Self as IntoIterator>::Item] as Index<RangeFull>>::Output>
+ ~const IndexMut<usize>
+ ~const IndexMut<Range<usize>>
+ ~const IndexMut<RangeInclusive<usize>>
+ ~const IndexMut<RangeFrom<usize>>
+ ~const IndexMut<RangeTo<usize>>
+ ~const IndexMut<RangeToInclusive<usize>>
+ ~const IndexMut<RangeFull>;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works()
    {
        let str = b"abcdefghijklmnopqrstuvwxyz".map(|c| (c as char).to_string());
        
        println!("str: {:?}", str);
        println!("spread: {:?}", str.array_spread_ref::<3>());
        println!("chunks: {:?}", str.array_chunks_ref::<3>());
    }

    #[test]
    fn nd_test()
    {
        type T = u8;

        const ND: [[T; 3]; 3] = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];
        const FLAT: [T; 9] = ND.flatten_nd_array();
        assert_eq!(FLAT, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn generate_impl_nd_array_macro_args()
    {
        const R: usize = 110;

        print!("impl_nd_array!(\n   ");
        let mut c = 0;
        for i in 0usize..256
        {
            c += (i.max(1)).ilog10() as usize + 3;
            if c > R
            {
                print!("\n   ");
                c = 0;
            }
            print!(" _{}", i);
        }
        println!("\n);")
    }
}