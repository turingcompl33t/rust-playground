// mergesort.rs

// NOTE: I made use of the Clone trait here to implement
// "copying", perhaps it would have been more idiomatic to
// impose the Copy trait bound on T instead?

use super::Sorter;

// a more elegant merge sort with lots of temporary allocations and copies
pub struct MergeSorter;
impl Sorter for MergeSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Sized + Default + Clone,
    {
        let mut sorted = mergesort(&slice);
        slice.swap_with_slice(&mut sorted);
    }
}

fn mergesort<T>(slice: &[T]) -> Box<[T]>
where
    T: Ord + Sized + Default + Clone,
{
    if slice.len() <= 1 {
        let mut tmp = vec![T::default(); slice.len()].into_boxed_slice();
        tmp.clone_from_slice(&slice);
        return tmp;
    }

    let mid = slice.len() / 2;
    let lower = mergesort(&slice[0..mid]);
    let upper = mergesort(&slice[mid..]);

    merged(&lower, &upper)
}

fn merged<T>(a: &[T], b: &[T]) -> Box<[T]>
where
    T: Ord + Sized + Default + Clone,
{
    let a_len = a.len();
    let b_len = b.len();

    // construct a temporary slice that will hold sorted result
    let mut tmp = Vec::with_capacity(a_len + b_len);

    let mut i = 0;
    let mut j = 0;
    while i < a_len && j < b_len {
        if a[i] <= b[j] {
            tmp.push(a[i].clone());
            i += 1;
        } else {
            tmp.push(b[j].clone());
            j += 1;
        }
    }

    // copy the remainder to the new, sorted slice
    if tmp.len() < a_len + b_len {
        let src = if i < a_len { &a[i..] } else { &b[j..] };
        for v in src {
            tmp.push(v.clone());
        }
    }

    tmp.into_boxed_slice()
}

// a memory-efficient merge sort implementation
pub struct InplaceMergeSorter;
impl Sorter for InplaceMergeSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone + Default,
    {
        let len = slice.len();

        // create a second copy of the input
        let mut tmp = vec![T::default(); len];
        tmp.clone_from_slice(&slice);

        split_and_merge(slice, &mut tmp, 0, len);
    }
}

// sort the range [begin, end) from src into dst
fn split_and_merge<T>(dst: &mut [T], src: &mut [T], begin: usize, end: usize)
where
    T: Ord + Clone,
{
    if begin > end || end - begin <= 1 {
        return;
    }

    let mid = (begin + end) / 2;
    split_and_merge(src, dst, begin, mid);
    split_and_merge(src, dst, mid, end);
    swap_merge(dst, src, begin, mid, end);
}

// merge two sorted ranges
fn swap_merge<T>(dst: &mut [T], src: &mut [T], begin: usize, mid: usize, end: usize)
where
    T: Ord + Clone,
{
    let mut i = begin;
    let mut j = mid;
    for k in begin..end {
        if i < mid && (j >= end || src[i] <= src[j]) {
            dst[k] = src[i].clone();
            i += 1;
        } else {
            dst[k] = src[j].clone();
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sorter1() {
        let mut v = vec![4, 3, 2, 1];
        MergeSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }

    #[test]
    fn inplace_merge_sorter1() {
        let mut v = vec![4, 3, 2, 1];
        InplaceMergeSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }
}
