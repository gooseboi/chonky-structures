use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let mut smallest = i;
            for j in i..slice.len() {
                if slice[smallest] > slice[j] {
                    smallest = j;
                }
            }

            if smallest != i {
                slice.swap(smallest, i);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut v = vec![1, 4, 3, 5, 2];
    SelectionSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}
