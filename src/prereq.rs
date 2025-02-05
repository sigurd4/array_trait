use core::borrow::{Borrow, BorrowMut};

use slice_trait::SlicePrereq;


pub trait ArrayPrereq = Sized
    + IntoIterator
    + AsRef<[<Self as IntoIterator>::Item]>
    + AsMut<[<Self as IntoIterator>::Item]>
    + Borrow<[<Self as IntoIterator>::Item]>
    + BorrowMut<[<Self as IntoIterator>::Item]>
    + SlicePrereq<<Self as IntoIterator>::Item>;