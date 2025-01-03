use super::Sorter;

pub struct SelectionSort;

pub fn sort<T>(slice: &mut [T])
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

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        sort(slice);
    }
}
