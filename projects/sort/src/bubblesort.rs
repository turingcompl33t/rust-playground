// bubblesort.rs

use super::Sorter;

pub struct BubbleSorter;

impl Sorter for BubbleSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_sorter1() {
        let mut v = vec![4, 3, 2, 1];
        BubbleSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }

    #[test]
    fn bubble_sorter2() {
        let mut v = vec![4, 3, 2, 1, 5];
        BubbleSorter::sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4, 5]);
    }
}
