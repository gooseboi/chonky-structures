use super::Sorter;

pub struct MergeSort {}

fn merge<T>(slice: &mut [T], split: usize) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut temp = Vec::with_capacity(slice.len());
    let mut i = 0;
    let mut j = split + 1;
    while i < split && j < slice.len() {
        if slice[i] < slice[j] {
            temp.push(slice[i].clone());
            i += 1;
        } else {
            temp.push(slice[j].clone());
            j += 1;
        }
    }

    while i < split {
        temp.push(slice[i].clone());
        i += 1;
    }

    while j < slice.len() {
        temp.push(slice[j].clone());
        j += 1;
    }
    temp
}
pub fn sort<T>(slice: &mut [T])
where
    T: Ord + Clone + core::fmt::Debug,
{
    println!("{:#?}", slice);
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {
            let mid = slice.len() / 2;
            sort(&mut slice[..=mid]);
            sort(&mut slice[(mid + 1)..]);
            let ret = merge(slice, mid);
            slice.clone_from_slice(&ret);
        }
    }
}

impl<T: Clone + core::fmt::Debug> Sorter<T> for MergeSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        sort(slice);
    }
}
