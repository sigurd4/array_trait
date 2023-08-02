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
#![feature(const_replace)]
#![feature(const_deref)]
#![feature(const_refs_to_cell)]

moddef::moddef!(
    flat(pub) mod {
        array,
        array_nd,

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

    #[inline]
    pub(crate) const fn split_array_mandrop<T, const N: usize, const M: usize>(a: [T; N]) -> (ManuallyDrop<[T; M]>, ManuallyDrop<[T; N - M]>)
    {
        unsafe {transmute_unchecked_size(a)}
    }

    #[inline]
    pub(crate) const fn rsplit_array_mandrop<T, const N: usize, const M: usize>(a: [T; N]) -> (ManuallyDrop<[T; N - M]>, ManuallyDrop<[T; M]>)
    {
        unsafe {transmute_unchecked_size(a)}
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
    fn gpa()
    {
        #[repr(u8)]
        enum Grade
        {
            A = 5,
            B = 4,
            C = 3,
            D = 2,
            E = 1
        }

        const GRADES_UNI: [(u8, Grade); 21] = [
            (5, Grade::C), // Ingeniørrollen
            (5, Grade::A), // Programmering for beregning
            (5, Grade::B), // Elektrisitetslære
            (5, Grade::D), // Digitalteknikk
            (10, Grade::A), // Programmering og mikrokontrollere
            (10, Grade::A), // Matematikk 1
            (5, Grade::C), // Fysikk 1 - Mekanikk
            (5, Grade::A), // Elektrisitetslære 2
            (5, Grade::A), // Programmerbare logiske kretser
            (10, Grade::A), // Matematikk 2
            (5, Grade::C), // Kommunikasjon
            (10, Grade::B), // Analog elektronikk
            (10, Grade::B), // Systems design and engineering
            (5, Grade::C), // Statistikk
            (10, Grade::E), // Signalbehandling
            (10, Grade::C), // Reguleringsteknikk 1
            (5, Grade::B), // Fysikk 2 - Elektromagnetisme
            (10, Grade::C), // Reguleringsteknikk 2
            (10, Grade::C), // Matematikk 3
            (10, Grade::C), // Instrumentering og styring
            (20, Grade::B) // Bacheloroppgave - Automatisk gir-system for Lone Wolf ATV
        ];
        const GRADES_VGS: [u8; 23] = [
            5, // Engelsk
            2, // Spansk II
            4, // Geografi
            4, // Historie
            4, // Kroppsøving
            4, // Matematikk 1T
            5, // Naturfag
            4, // Norsk hovedmål
            4, // Norsk hovedmål, eksamen
            3, // Norsk sidemål
            2, // Norsk sidemål, eksamen
            3, // Norsk
            3, // Religion og etikk
            4, // Samfunnsfag
            4, // Fysikk 1
            4, // Fysikk 2
            5, // Fysikk 2, eksamen
            3, // Kjemi
            4, // Informasjonsteknologi 1
            5, // Informasjonsteknologi 2
            4, // Teknologi og forskningslære 1
            3, // Matematikk R1
            4, // Matematikk R2
        ];

        const GPA_UNI: f32 = GRADES_UNI.map2(const |(pts, grade)| (pts*grade as u8) as u16)
            .sum() as f32
            /GRADES_UNI.map2(const |(pts, _)| pts as u16)
            .sum() as f32;

        println!("{}", GPA_UNI);

        const GPA_VGS: f32 = GRADES_VGS.map2(const |grade| grade as u16)
            .sum() as f32
            /GRADES_VGS.len() as f32;
            
        println!("{}", GPA_VGS);
    }

    #[test]
    fn rotate()
    {
        let a = [1, 2, 3, 4, 5];

        //a.rotate_right2::<4>();
        println!("{:?}", a.into_shift_many_right([-1, -2, -3]));
    }

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

        const ND_T: [[T; 3]; 3] = ND.transpose();

        const FLAT_T: [T; 9] = ND_T.flatten_nd_array();
        assert_eq!(FLAT_T, [1, 4, 7, 2, 5, 8, 3, 6, 9]);
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