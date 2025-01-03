pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

pub mod bubble_sort;
pub use bubble_sort::BubbleSort;
pub mod heap_sort;
pub use heap_sort::HeapSort;
pub mod insertion_sort;
pub use insertion_sort::InsertionSort;
pub mod merge_sort;
pub use merge_sort::MergeSort;
pub mod quick_sort;
pub use quick_sort::QuickSort;
pub mod selection_sort;
pub use selection_sort::SelectionSort;

#[cfg(test)]
mod tests;
