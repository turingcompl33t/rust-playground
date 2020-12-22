// quicksort.rs

use super::Sorter;

pub struct QuickSorter;
impl Sorter for QuickSorter {
    fn sort<T>(slice: &mut [T]) 
        where T: Ord,
    {
        let r = slice.len() - 1;
        quicksort(slice, 0, r);
    }
}

fn quicksort<T: Ord>(slice: &mut [T], p: usize, r: usize) {
    // base case: 1-element slice is sorted
    if p >= r {
        return;
    }

    let part = partition(slice, p, r);
    quicksort(slice, p, part.saturating_sub(1));
    quicksort(slice, part + 1, r);
}

// partition the slice p..r, returning the index of the pivot
fn partition<T: Ord>(slice: &mut [T], p: usize, r: usize) -> usize {
    let mut q = p;
    let mut j = p;
    while j < r {
        if slice[j] > slice[r] {
            j += 1;
        } else {
            slice.swap(q, j);
            q += 1;
            j += 1;
        }
    }
    // move the pivot to its final position
    slice.swap(r, q);
    
    q
}


// John's version of QuickSort from stream
#[allow(dead_code)]
fn quicksort_alt<T: Ord>(slice: &mut [T]) {
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
    quicksort_alt(left);
    quicksort_alt(&mut right[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quick_sorter1() {
        let mut v = vec![4, 3, 2, 1];
        QuickSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }

    #[test]
    fn quick_sorter2() {
        let mut v = vec![4, 3, 2, 1, 5];
        QuickSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn quick_sorter3() {
        let mut v = vec![1, 2, 3, 4, 5];
        QuickSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn partition1() {
        let mut v = vec![4, 3, 2, 1];
        let r = v.len() - 1;
        let p = partition(& mut v, 0, r);
        assert_eq!(p, 0);
        assert_eq!(*v.get(p).unwrap(), 1);
    }

    #[test]
    fn partition2() {
        let mut v = vec![4, 3, 1, 2];
        let r = v.len() - 1;
        let p = partition(&mut v, 0, r);
        assert_eq!(p, 1);
        assert_eq!(*v.get(p).unwrap(), 2);
    }

    #[test]
    fn partition3() {
        let mut v = vec![4, 1, 2, 3];
        let r = v.len() - 1;
        let p = partition(&mut v, 0, r);
        assert_eq!(p, 2);
        assert_eq!(*v.get(p).unwrap(), 3);
    }

    #[test]
    fn partition4() {
        let mut v = vec![3, 2, 1, 4];
        let r = v.len() - 1;
        let p = partition(&mut v, 0, r);
        assert_eq!(p, 3);
        assert_eq!(*v.get(p).unwrap(), 4);
    }
}