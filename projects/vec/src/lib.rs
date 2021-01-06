// lib.rs
// Re-implementation of std::vec::Vec from Ryan Levick's stream.

use std::mem;
use std::ptr;
use std::ptr::NonNull;
use std::{alloc, ops::Bound, ops::RangeBounds};

pub struct Vector<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, value: T) {
        assert_ne!(mem::size_of::<T>(), 0, "Zero-size types not supported");

        if self.capacity == 0 {
            // vector must be initialized

            let layout = alloc::Layout::array::<T>(4).expect("Failed to allocate");

            // SAFETY: the layout is hardcoded to be 4*size_of<T>, and size_of<T> is nonzero
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Failed to allocate");

            // SAFETY: ptr is non-null, and we have allocated sufficient space for 4 Ts;
            // the memory (uninitialized) at ptr is not read by the call to ptr::write()
            unsafe {
                ptr::write(ptr.as_ptr(), value);
            }

            self.ptr = ptr;
            self.capacity = 4;
        } else if self.len < self.capacity {
            // pushed value fits within existing allocation

            let offset = self
                .len
                .checked_mul(mem::size_of::<T>())
                .expect("Memory address overflow");
            assert!(offset < isize::MAX as usize, "Wrapped isize");

            // SAFETY: offset cannot overflow usize, ptr is non-null, and writing to offset at self.len is valid
            unsafe {
                ptr::write(self.ptr.as_ptr().add(self.len), value);
            }
        } else {
            // internal buffer must reallocate
            debug_assert_eq!(self.len, self.capacity);

            let new_capacity = self.capacity.checked_mul(2).expect("Wrapped capacity");

            let size = mem::size_of::<T>() * self.capacity;
            let align = mem::align_of::<T>();
            size.checked_add(size % align).expect("Failed to allocate");
            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size) as *mut T;
                let ptr = NonNull::new(ptr).expect("Failed to allocate");

                ptr::write(ptr.as_ptr().add(self.len), value);
                ptr
            };
            self.ptr = ptr;
            self.capacity = new_capacity;
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                Some(unsafe { ptr::read(self.ptr.as_ptr().add(self.len)) })
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        let ret = unsafe {
            // read the value out
            let tmp = ptr::read(self.ptr.as_ptr().add(index));
            // shift the remaining items down
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index - 1,
            );
            tmp
        };

        self.len -= 1;
        ret
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        // SAFETY: T at index indicated by index is valid
        unsafe { self.ptr.as_ptr().add(index).as_ref() }
    }

    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        // SAFETY: T at index indicated by index is valid
        unsafe { self.ptr.as_ptr().add(index).as_mut() }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(&self)
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let begin = match range.start_bound() {
            Bound::Included(&i) => i,
            Bound::Unbounded => 0,
            _ => panic!("Malformed RangeBounds in drain()"),
        };
        let end = match range.end_bound() {
            Bound::Excluded(&i) => i,
            Bound::Unbounded => self.len,
            _ => panic!("Malformed RangeBounds in drain()"),
        };

        println!("{} {}", begin, end);
        println!("{}", end - begin);
        Drain {
            idx: begin,
            cur: 0,
            len: end - begin,
            vec: self,
        }
    }

    pub fn drain_filter<F>(&mut self, filter: F) -> DrainFilter<'_, T, F>
    where
        F: FnMut(&mut T) -> bool,
    {
        DrainFilter {idx: 0, filter, vec: self}
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.capacity == 0 {
            return;
        }

        unsafe {
            // drop all of the values in the vector
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));

            // deallocate the memory used by the vector
            let layout = alloc::Layout::from_size_align_unchecked(
                mem::size_of::<T>() * self.capacity,
                mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

pub struct Iter<'a, T> {
    vec: &'a Vector<T>,
    idx: usize,
}

impl<'a, T> Iter<'a, T> {
    fn new(vec: &'a Vector<T>) -> Self {
        Self { vec, idx: 0 }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.idx;
        self.idx += 1;
        self.vec.get(tmp)
    }
}

pub struct Drain<'a, T> {
    idx: usize,
    cur: usize,
    len: usize,
    vec: &'a mut Vector<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.len {
            self.cur += 1;
            Some(self.vec.remove(self.idx))
        } else {
            None
        }
    }
}

pub struct DrainFilter<'a, T, F> {
    idx: usize,
    filter: F,
    vec: &'a mut Vector<T>,
}

impl<'a, T, F> Iterator for DrainFilter<'a, T, F>
where
    F: FnMut(&mut T) -> bool,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let func = &mut self.filter;
        while self.idx < self.vec.len() {
            if func(self.vec.get_mut(self.idx).unwrap()) {
                let tmp = self.idx;
                self.idx += 1;
                return Some(self.vec.remove(tmp));
            }
            self.idx += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc1() {
        let vec = Vector::<i32>::new();
        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn alloc2() {
        let mut vec = Vector::<i32>::new();
        assert!(vec.is_empty());

        for i in 0..3 {
            vec.push(i);
        }

        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 3);
        assert!(!vec.is_empty());
    }

    #[test]
    fn alloc3() {
        let mut vec = Vector::<i32>::new();
        assert!(vec.is_empty());

        for i in 0..5 {
            vec.push(i);
        }

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
        assert!(!vec.is_empty());
    }

    #[test]
    fn get1() {
        let mut vec = Vector::<i32>::new();
        for i in 0..3 {
            vec.push(i);
        }

        for i in 0..vec.len() {
            assert_eq!(*vec.get(i).unwrap(), i as i32);
        }
    }

    #[test]
    fn pop1() {
        let mut vec = Vector::<i32>::new();
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn pop2() {
        let mut vec = Vector::<i32>::new();
        vec.push(0);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.pop(), Some(0));
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn remove1() {
        let mut vec = Vector::<i32>::new();
        for i in 0..3 {
            vec.push(i);
        }

        assert_eq!(vec.len(), 3);

        for i in 0..vec.len() {
            assert_eq!(vec.remove(0), i as i32);
            assert_eq!(vec.len(), 3 - i - 1);
        }
    }

    #[test]
    fn iter1() {
        let mut vec = Vector::<i32>::new();
        for i in 0..3 {
            vec.push(i);
        }

        let std: Vec<i32> = vec.iter().cloned().collect();
        assert_eq!(std, vec![0, 1, 2]);
    }

    #[test]
    fn iter2() {
        let mut vec = Vector::<i32>::new();
        for i in 0..5 {
            vec.push(i);
        }

        let std: Vec<i32> = vec.iter().cloned().collect();
        assert_eq!(std, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn drain1() {
        let mut vec = Vector::<i32>::new();
        for i in 0..5 {
            vec.push(i);
        }

        let res: Vec<_> = vec.drain(..).collect();
        assert_eq!(res, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn drain2() {
        let mut vec = Vector::<i32>::new();
        for i in 0..5 {
            vec.push(i);
        }

        let res: Vec<_> = vec.drain(..3).collect();
        assert_eq!(res, vec![0, 1, 2]);
    }

    #[test]
    fn drain3() {
        let mut vec = Vector::<i32>::new();
        for i in 0..5 {
            vec.push(i);
        }

        let res: Vec<_> = vec.drain(2..).collect();
        assert_eq!(res, vec![2, 3, 4]);
    }

    #[test]
    fn drain_filter1() {
        let mut vec = Vector::<i32>::new();
        for i in 0..5 {
            vec.push(i);
        }

        let drained: Vec<_> = vec.drain_filter(|x| *x % 2 == 0).collect();
        let remaining: Vec<_> = vec.iter().cloned().collect();
        
        assert_eq!(drained, vec![0, 2, 4]);
        assert_eq!(remaining, vec![1, 3]);
    }
}
