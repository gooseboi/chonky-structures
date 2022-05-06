use super::Sorter;

pub struct InsertionSort;

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
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
}

#[test]
fn it_works() {
    let mut v = vec![1, 4, 3, 5, 2];
    InsertionSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}
