// selectionsort.rs

use super::Sorter;

pub struct SelectionSorter;
impl Sorter for SelectionSorter {
    fn sort<T>(slice: &mut [T])
        where T: Ord
    {
        for unsorted in 0..slice.len() {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice is nonempty");
            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sorter() {
        let mut v = vec![4, 3, 2, 1];
        SelectionSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }
}