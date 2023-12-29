#![cfg_attr(not(test), no_std)]

#![feature(const_trait_impl)]
#![feature(trait_alias)]

#![feature(generic_const_exprs)]

moddef::moddef!(
    flat(pub) mod {
        array,
        array_nd,
    }
);

use core::borrow::{Borrow, BorrowMut};

mod private
{
    pub trait Array {}
    impl<Item, const LENGTH: usize> Array for [Item; LENGTH] {}
}

#[cfg(test)]
mod test
{
    #[test]
    fn test()
    {
        
    }
}

use slice_trait::SlicePrereq;

pub trait ArrayPrereq = Sized
+ IntoIterator
+ AsRef<[<Self as IntoIterator>::Item]>
+ AsMut<[<Self as IntoIterator>::Item]>
+ Borrow<[<Self as IntoIterator>::Item]>
+ BorrowMut<[<Self as IntoIterator>::Item]>
+ SlicePrereq<<Self as IntoIterator>::Item>;