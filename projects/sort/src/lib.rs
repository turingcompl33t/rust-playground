// lib.rs

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone + Default;
}

pub mod bubblesort;
pub mod insertionsort;
pub mod mergesort;
pub mod quicksort;
pub mod selectionsort;

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_sorter() {
        let mut v = vec![4, 3, 2, 1];
        StdSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }
}
