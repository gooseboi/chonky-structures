use super::Sorter;

pub struct InsertionSort;

pub fn sort<T>(slice: &mut [T])
where
    T: Ord,
{
    for i in 0..slice.len() {
        let mut j = i;
        while j != 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        sort(slice);
    }
}
