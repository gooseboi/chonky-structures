use super::Sorter;

pub struct BubbleSort;

pub fn sort<T>(slice: &mut [T])
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

impl<T> Sorter<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        sort(slice);
    }
}
