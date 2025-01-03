use super::*;

#[test]
fn test_all() {
    let list = [8, 4, 2, 17, 5, 3, 1, 13, 9, 6, 7, 12];
    let fun = |sorter: &dyn Sorter<_>| {
        let mut list = list.clone();
        sorter.sort(&mut list);
        list
    };
    macro_rules! test_sorter {
        ($ty:ident) => {
            let ret = fun(&$ty {});
            assert_eq!(ret, [1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 17]);
        };
    }
    test_sorter!(BubbleSort);
    test_sorter!(HeapSort);
    test_sorter!(QuickSort);
    test_sorter!(InsertionSort);
    test_sorter!(SelectionSort);
    //test_sorter!(MergeSort);
}
