// quicksort.rs

use super::Sorter;

pub struct QuickSorter;
impl Sorter for QuickSorter {
    fn sort<T>(slice: &mut [T]) 
        where T: Ord,
    {
        quicksort(slice);
    }
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        },
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is nonempty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if 0 == right {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if 0 == right {
                break;
            }
            right -= 1;
        }
    }

    let left = left + 1;

    // place the pivot at its final position
    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left - 1);
    quicksort(left);
    quicksort(&mut right[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quick_sorter() {
        let mut v = vec![4, 3, 2, 1];
        QuickSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }
}