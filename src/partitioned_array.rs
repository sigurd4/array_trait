use super::*;

pub const fn split_all_len<const PART_LENGTHS: &'static [usize], const N: usize, const PICK: bool>() -> &'static [usize]
{
    let split = PART_LENGTHS.split_at(N);
    match PICK
    {
        false => split.0,
        true => split.1
    }
}

pub const fn sum_len<const PART_LENGTHS: &'static [usize]>() -> usize
{
    let mut i = 0;
    let mut len = 0;
    while i < PART_LENGTHS.len()
    {
        len += PART_LENGTHS[i];
        i += 1;
    }
    len
}

pub const fn sum_len_eq<const A: &'static [usize], const B: &'static [usize]>() -> bool
{
    sum_len::<{A}>() == sum_len::<{B}>()
}

pub const fn all_len_eq<const A: &'static [usize], const B: &'static [usize]>() -> bool
{
    let len = A.len();
    if len != B.len()
    {
        return false;
    }

    let mut i = 0;
    while i < len
    {
        if A[i] != B[i]
        {
            return false;
        }
        i += 1;
    }

    true
}

pub struct PartitionedArray<T, const PART_LENGTHS: &'static [usize]>([T; sum_len::<{PART_LENGTHS}>()])
where
    [(); sum_len::<{PART_LENGTHS}>()]:;

impl<T, const PART_LENGTHS: &'static [usize]> PartitionedArray<T, PART_LENGTHS>
where
    [(); sum_len::<{PART_LENGTHS}>()]:,
    [(); PART_LENGTHS.len()]:
{
    pub const PARTS: usize = PART_LENGTHS.len();
    pub const PART_LENGTHS: [usize; PART_LENGTHS.len()] = *PART_LENGTHS.split_array_ref2().0;
    pub const SERIALIZED_LENGTH: usize = sum_len::<{PART_LENGTHS}>();

    pub const fn partition(array: [T; sum_len::<PART_LENGTHS>()]) -> Self
    {
        Self(array)
    }

    pub const fn serialize_arrays(self) -> [T; sum_len::<{PART_LENGTHS}>()]
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    pub const fn as_serialize_arrays(&self) -> &[T; sum_len::<{PART_LENGTHS}>()]
    {
        &self.0
    }
    pub const fn as_serialize_arrays_mut(&mut self) -> &mut [T; sum_len::<{PART_LENGTHS}>()]
    {
        &mut self.0
    }
    
    pub const fn as_ptr(&self) -> *const T
    {
        self.0.as_ptr()
    }
    pub const fn as_mut_ptr(&mut self) -> *mut T
    {
        self.0.as_mut_ptr()
    }

    pub const fn part_offsets() -> [usize; PART_LENGTHS.len()]
    {
        Self::PART_LENGTHS.integrate_from(0)
    }
    pub const fn each_ptr(&self) -> [*const T; PART_LENGTHS.len()]
    {
        let ptr = self.as_ptr();
        
        Self::part_offsets()
            .map2(const |offset| unsafe {ptr.add(offset)})
    }
    pub const fn each_mut_ptr(&mut self) -> [*mut T; PART_LENGTHS.len()]
    {
        let ptr = self.as_mut_ptr();

        Self::part_offsets()
            .map2(const |offset| unsafe {ptr.add(offset)})
    }

    pub const fn each_slice(&self) -> [&[T]; PART_LENGTHS.len()]
    {
        self.each_ptr()
            .zip2(Self::PART_LENGTHS)
            .map2(const |(ptr, len)| unsafe {core::slice::from_raw_parts(ptr, len)})
    }
    pub const fn each_slice_mut(&mut self) -> [&mut [T]; PART_LENGTHS.len()]
    {
        self.each_mut_ptr()
            .zip2(Self::PART_LENGTHS)
            .map2(const |(ptr, len)| unsafe {core::slice::from_raw_parts_mut(ptr, len)})
    }

    pub const fn get_slice(&self, index: usize) -> Option<&[T]>
    {
        self.each_ptr()
            .zip2(Self::PART_LENGTHS)
            .get(index)
            .map(const |&(ptr, len)| unsafe {core::slice::from_raw_parts(ptr, len)})
    }
    pub const fn get_slice_mut(&mut self, index: usize) -> Option<&mut [T]>
    {
        self.each_mut_ptr()
            .zip2(Self::PART_LENGTHS)
            .get(index)
            .map(const |&(ptr, len)| unsafe {core::slice::from_raw_parts_mut(ptr, len)})
    }

    pub const fn reinterpret_lengths<const S: usize, const P: &'static [usize]>(self) -> PartitionedArray<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); sum_len_eq::<{PART_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    pub const fn reinterpret_lengths_ref<const S: usize, const P: &'static [usize]>(&self) -> &PartitionedArray<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); sum_len_eq::<{PART_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
    pub const fn reinterpret_lengths_mut<const S: usize, const P: &'static [usize]>(&mut self) -> &mut PartitionedArray<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); sum_len_eq::<{PART_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
    
    pub const fn reformulate_lengths<const S: usize, const P: &'static [usize]>(self) -> PartitionedArray<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); all_len_eq::<{PART_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    pub const fn reformulate_lengths_ref<const S: usize, const P: &'static [usize]>(&self) -> &PartitionedArray<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); all_len_eq::<{PART_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
    pub const fn reformulate_lengths_mut<const S: usize, const P: &'static [usize]>(&mut self) -> &mut PartitionedArray<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); all_len_eq::<{PART_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
}

#[cfg(test)]
mod test
{
    use crate::{PartitionedArray, ArrayOps};

    #[test]
    fn it_works()
    {
        let tuple = ([1u8, 2], [3u8, 4, 5], [6u8, 7]);

        let partition = PartitionedArray::<u8, {&[2usize, 3, 2]}>::partition(tuple.0.chain(tuple.1).chain(tuple.2));
    
        assert_eq!(Some(tuple.0.as_slice()), partition.get_slice(0));
        assert_eq!(Some(tuple.1.as_slice()), partition.get_slice(1));
        assert_eq!(Some(tuple.2.as_slice()), partition.get_slice(2));

        println!("a = {:?}", partition.each_slice());
    }
}