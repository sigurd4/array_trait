use core::borrow::{Borrow, BorrowMut};

use slice_trait::SlicePrereq;


pub trait ArrayPrereq<T> = Sized
    + IntoIterator<Item = T>
    + AsRef<[T]>
    + AsMut<[T]>
    + Borrow<[T]>
    + BorrowMut<[T]>
    + SlicePrereq<T>;