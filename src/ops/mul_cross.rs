use core::ops::{Mul, Sub, Add};

use crate::ArrayOps;

#[const_trait]
pub trait MulCross<Rhs>
{
    type Output;

    fn mul_cross(&self, rhs: &Rhs) -> Self::Output;
}

impl<T, Rhs> const MulCross<[Rhs; 3]> for [T; 3]
where
    Rhs: Copy,
    T: ~const Mul<Rhs> + Copy,
    <T as Mul<Rhs>>::Output: ~const Sub
{
    type Output = [<<T as Mul<Rhs>>::Output as Sub>::Output; 3];

    fn mul_cross(&self, rhs: &[Rhs; 3]) -> Self::Output
    {
        const L: usize = 3;
        ArrayOps::fill(const |i| self[(i + 1) % L]*rhs[(i + 2) % L] - self[(i + 2) % L]*rhs[(i + 1) % L])
    }
}

impl<T, Rhs> const MulCross<[Rhs; 7]> for [T; 7]
where
    Rhs: Copy,
    T: ~const Mul<Rhs> + Copy,
    <T as Mul<Rhs>>::Output: ~const Sub,
    <<T as Mul<Rhs>>::Output as Sub>::Output: ~const Add,
    <<<T as Mul<Rhs>>::Output as Sub>::Output as Add>::Output: ~const Add<<<T as Mul<Rhs>>::Output as Sub>::Output>
{
    type Output = [<<<<T as Mul<Rhs>>::Output as Sub>::Output as Add>::Output as Add<<<T as Mul<Rhs>>::Output as Sub>::Output>>::Output; 7];

    fn mul_cross(&self, rhs: &[Rhs; 7]) -> Self::Output
    {
        const L: usize = 7;
        ArrayOps::fill(const |i|
              (self[(i + 1) % L]*rhs[(i + 3) % L] - self[(i + 3) % L]*rhs[(i + 1) % L])
            + (self[(i + 2) % L]*rhs[(i + 6) % L] - self[(i + 6) % L]*rhs[(i + 2) % L])
            + (self[(i + 4) % L]*rhs[(i + 5) % L] - self[(i + 5) % L]*rhs[(i + 4) % L])
        )
    }
}