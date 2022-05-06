pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;
