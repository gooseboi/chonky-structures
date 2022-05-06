use super::Sorter;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i] < slice[i - 1] {
                    slice.swap(i, i - 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut v = vec![1, 4, 3, 5, 2];
    BubbleSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}
