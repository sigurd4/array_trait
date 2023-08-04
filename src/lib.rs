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
#![feature(const_slice_index)]
#![feature(const_ptr_read)]
#![feature(const_ptr_write)]
#![feature(transmutability)]
#![feature(const_maybe_uninit_as_mut_ptr)]
#![feature(const_option_ext)]
#![feature(const_borrow)]

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
    use core::mem::{ManuallyDrop, MaybeUninit, BikeshedIntrinsicFrom, Assume};

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
        /*#[cfg(test)]
        if core::mem::size_of::<A>() != core::mem::size_of::<B>() && core::mem::align_of::<A>() != core::mem::align_of::<B>()
        {
            panic!("Cannot transmute due to unequal size or alignment")
        }*/
        
        /*let from = ManuallyDrop::new(from);
        core::ptr::read(core::mem::transmute(from.deref()))*/

        ManuallyDrop::into_inner(unsafe {Transmutation {a: ManuallyDrop::new(from)}.b})
    }

    #[allow(unused)]
    pub struct Context;

    #[inline]
    pub(crate) const unsafe fn transmute<A, B>(from: A) -> B
    where
        B: BikeshedIntrinsicFrom<A, Context, {Assume::LIFETIMES + Assume::SAFETY + Assume::VALIDITY}>
    {
        #[allow(deprecated)]
        transmute_unchecked_size(from)
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

    #[inline]
    pub(crate) const unsafe fn take_and_leave_uninit<T>(from: &mut T) -> T
    {
        core::mem::replace(from, unsafe {MaybeUninit::assume_init(MaybeUninit::uninit())})
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

    use std::time::SystemTime;

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
    fn benchmark()
    {
        const N: usize = 64;
        const M: usize = 256;
        
        assert_eq!(<[[[u8; 2]; N]; M]>::DIMENSIONS, [M, N, 2]);

        let a: [[[u8; 2]; N]; M] = ArrayNdOps::fill_nd(const |i| i.map2(const |i| i as u8));

        let t0 = SystemTime::now();
        for m in 0..M
        {
            for n in 0..N
            {
                //<[u8; N]>::fill(|i| i as u8);
                //a[m].truncate::<{N/2}>();
                //a[m].resize::<{N/2}, _>(|i| [m as u8, i as u8]);
                //let (matrix, _) = a[m].array_spread::<3>();
                for k in 0..2
                {
                    let i = [m, n, k];
                    let _ = *a.get_nd(i).unwrap();
                }
            }
        }
        let t = t0.elapsed().unwrap();
        println!("t = {:?}", t); //10.5832ms
    }

    #[test]
    fn reduce()
    {
        const A: [[(u8, u8); 3]; 2] = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)]
        ];
        
        let r: (u8, u8) = A.reduce_nd(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2)).unwrap();
        
        assert_eq!(r, (3, 6));
    }

    #[test]
    fn rotate()
    {
        let mut a = [1, 2, 3, 4, 5];

        a.rotate_left2(2);
        println!("{:?}", a);
    }

    #[test]
    fn test_spread_align()
    {
        let str = b"abcdefghijklmnopqrstuvwxyz".map(|c| c as char);
        
        println!("Alignment char = {}", core::mem::align_of::<char>());
        println!("Alignment padded x3 char = {}", core::mem::align_of::<Padded<char, 3>>());
        
        println!("Alignment String = {}", core::mem::align_of::<String>());
        println!("Alignment padded x3 String = {}", core::mem::align_of::<Padded<String, 3>>());

        println!("str: {:?}", str);
        println!("spread: {:?}", str.array_spread_ref::<3>());
        println!("chunks: {:?}", str.array_chunks_ref::<3>());

        assert_eq!(
            str.array_spread::<3>(),
            (
                [
                    ['a', 'd', 'g', 'j', 'm', 'p', 's', 'v'],
                    ['b', 'e', 'h', 'k', 'n', 'q', 't', 'w'],
                    ['c', 'f', 'i', 'l', 'o', 'r', 'u', 'x']
                ],
                ['y', 'z']
            )
        );
        assert_eq!(
            str.array_chunks::<3>(),
            (
                [
                    ['a', 'b', 'c'],
                    ['d', 'e', 'f'],
                    ['g', 'h', 'i'],
                    ['j', 'k', 'l'],
                    ['m', 'n', 'o'],
                    ['p', 'q', 'r'],
                    ['s', 't', 'u'],
                    ['v', 'w', 'x']
                ],
                ['y', 'z']
            )
        );
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