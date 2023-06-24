#![feature(generic_const_exprs)]
#![feature(iter_array_chunks)]

mod private
{
    pub trait Array {}
    impl<Item, const LENGTH: usize> Array for [Item; LENGTH] {}
}

pub trait Array: private::Array
{
    type Item;
    const LENGTH: usize;

    fn into_array(self) -> [Self::Item; Self::LENGTH];
    fn as_array(&self) -> &[Self::Item; Self::LENGTH];
    fn as_array_mut(&mut self) -> &mut [Self::Item; Self::LENGTH];
}
impl<Item, const LENGTH: usize> Array for [Item; LENGTH]
{
    type Item = Item;
    const LENGTH: usize = LENGTH;

    fn into_array(self) -> [Self::Item; Self::LENGTH]
    {
        self.into_iter().array_chunks().next().unwrap()
    }
    fn as_array(&self) -> &[Self::Item; Self::LENGTH]
    {
        unsafe {std::mem::transmute(self)}
    }
    fn as_array_mut(&mut self) -> &mut [Self::Item; Self::LENGTH]
    {
        unsafe {std::mem::transmute(self)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        let a = [1.0, 2.0, 3.0];
        {
            let a_ref: &[f32; 3] = a.as_array();
            assert_eq!(a, *a_ref);
        }
        let mut a: [f32; 3] = a.into_array();
        {
            let _a_mut: &[f32; 3] = a.as_array_mut();
        }
    }
}
