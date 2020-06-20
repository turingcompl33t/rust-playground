// lib.rs

use std::cell::RefCell;

struct ResourcePool<T> {
    resources: RefCell<Vec<T>>
}

impl<T: Default> ResourcePool<T> {
    pub fn new() -> Self {
        Self { resources: RefCell::new(Vec::new()) }
    }

    pub fn get(&self) -> ResourceGuard<T> {
        let resource = match self.resources.borrow_mut().pop() {
            Some(item) => item,
            None => T::default()
        };

        ResourceGuard { 
            resource: Some(resource), 
            resources: &self.resources 
        }
    }
}

struct ResourceGuard<'a, T> {
    resource: Option<T>,
    resources: &'a RefCell<Vec<T>>
}

impl<T> Drop for ResourceGuard<'_, T> {
    fn drop(&mut self) {
        let resource = self.resource.take().unwrap();
        self.resources.borrow_mut().push(resource);
    }
}

impl<T> std::ops::Deref for ResourceGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.resource.as_ref().unwrap()
    }
} 

impl<T> std::ops::DerefMut for ResourceGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.resource.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Data {
        count: usize
    }

    impl Data {
        fn print(&self) {
            println!("do_thing()");
        }

        fn inc(&mut self) {
            self.count += 1
        }
    }

    impl Default for Data {
        fn default() -> Data {
            Data{count: 0}
        }
    }

    #[test]
    fn resources_default_initialized() {
        let pool : ResourcePool<Data> = ResourcePool::new();

        let r = pool.get();
        assert_eq!(r.count, 0);
    }

    #[test]
    fn resources_returned_to_pool() {
        let pool = ResourcePool::<Data>::new();

        {
            // creates a new resource in the pool
            let mut r = pool.get();

            // the resource is default initialized
            assert_eq!(r.count, 0);

            // mutate the resource
            r.inc();

            // now the resource is in a new state
            assert_eq!(r.count, 1);

        }  // resource dropped here, returned to the pool

        // now the next time we acquire a resource from the pool,
        // it should be the one that was previously initialized and mutated
        let r = pool.get();
        assert_eq!(r.count, 1);
    }
}
