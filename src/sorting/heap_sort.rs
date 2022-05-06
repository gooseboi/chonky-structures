use super::Sorter;

pub struct HeapSort;

fn heapify<T: Ord>(slice: &mut [T], root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < slice.len() && slice[largest] < slice[left] {
        largest = left;
    }

    if right < slice.len() && slice[largest] < slice[right] {
        largest = right;
    }

    if largest != root {
        slice.swap(root, largest);
        heapify(slice, largest);
    }
}

impl<T> Sorter<T> for HeapSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for i in (0..(slice.len() / 2)).rev() {
            heapify(slice, i);
        }

        for i in (0..slice.len()).rev() {
            slice.swap(0, i);
            heapify(&mut slice[..i], 0);
        }
    }
}

#[test]
fn it_works() {
    let mut v = vec![1, 4, 3, 5, 2];
    HeapSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}
