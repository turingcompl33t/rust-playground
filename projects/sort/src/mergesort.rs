// mergesort.rs

use super::Sorter;

pub struct MergeSorter;
impl Sorter for MergeSorter {
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
    merge(dst, src, begin, mid, end);
}

// merge two sorted ranges
fn merge<T>(dst: &mut [T], src: &mut [T], begin: usize, mid: usize, end: usize)
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
}
