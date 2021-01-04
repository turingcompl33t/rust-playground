// insertionsort.rs

use super::Sorter;

pub struct InsertionSorter;
impl Sorter for InsertionSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;
            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i, i - 1);
                i -= 1;
            }
        }
    }
}

pub struct BinaryInsertionSorter;
impl Sorter for BinaryInsertionSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                Ok(i) => i,
                Err(i) => i,
            };
            slice[i..=unsorted].rotate_right(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sorter1() {
        let mut v = vec![4, 3, 2, 1];
        InsertionSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }

    #[test]
    fn insertion_sorter2() {
        let mut v = vec![4, 3, 2, 1, 5];
        InsertionSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn binary_insertion_sorter1() {
        let mut v = vec![4, 3, 2, 1];
        BinaryInsertionSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }

    #[test]
    fn binary_insertion_sorter2() {
        let mut v = vec![4, 3, 2, 1, 5];
        BinaryInsertionSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4, 5]);
    }
}
