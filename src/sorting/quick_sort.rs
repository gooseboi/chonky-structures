use super::Sorter;

pub struct QuickSort {}

pub fn sort<T>(slice: &mut [T])
where
    T: Ord,
{
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    };

    let (pivot, rest) = slice.split_first_mut().expect("slice cannot be empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while right != 0 && left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            right -= 1;
        } else {
            // element is on the wrong side
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // left was an index into rest, which is one less than the length of slice
    let left = left + 1;
    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left);
    sort(left);
    sort(right);
}

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        sort(slice);
    }
}
