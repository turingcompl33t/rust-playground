// lib.rs

#![allow(dead_code)]

use std::borrow::Borrow;
use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::RandomState;

const INITIAL_N_BUCKETS : usize = 1;

pub struct HashMap<K, V, S = RandomState> {
    buckets: Vec<Vec<(K, V)>>,
    item_count: usize,
    hash_builder: S
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            item_count: 0,
            hash_builder: RandomState::new()
        }
    }
}

impl<K, V, S> HashMap<K, V, S> 
where
    S: BuildHasher
{
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            buckets: Vec::new(),
            item_count: 0,
            hash_builder
        }
    }
}

impl<K, V, S> HashMap<K, V, S> 
where
    K: Hash + Eq,
    S: BuildHasher
{
    fn bucket<Q>(&self, key: &Q) -> Option<usize> 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.buckets.is_empty() {
            return None;
        }

        let mut hasher = self.hash_builder.build_hasher();
        key.hash(&mut hasher);
        Some((hasher.finish() % self.buckets.len() as u64) as usize)
    }

    fn resize(&mut self) {
        use std::mem;

        let target_size = match self.buckets.len() {
            0 => INITIAL_N_BUCKETS,
            n => n*2
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = self.hash_builder.build_hasher();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }

        mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.item_count > 3*self.buckets.len()/4 {
            self.resize();
        }

        let bucket = self.bucket(&key).expect("buckets.is_empty() handled above");
        let bucket = &mut self.buckets[bucket];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if key == *ekey {
                use std::mem;
                return Some(mem::replace(evalue, value));
            }
        }

        // key not found, insert new
        bucket.push((key, value));
        self.item_count += 1;
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V> 
    where 
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(&key)?;
        self.buckets[bucket]
            .iter()
            .find(|&(ref ekey, _)| { ekey.borrow() == key })
            .map(|&(_, ref evalue)| evalue)        
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(key)?;
        let bucket = &mut self.buckets[bucket];
        let i = bucket.iter().position(|&(ref ekey, _)| ekey.borrow() == key)?;
        
        self.item_count -= 1;
        Some(bucket.swap_remove(i).1)
    }

    pub fn len(&self) -> usize {
        self.item_count
    }

    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }
}

pub struct Iter<'a, K, V, S> {
    map: &'a HashMap<K, V, S>,
    bucket: usize,
    at: usize
}

impl<'a, K, V, S> Iter<'a, K, V, S> {
    pub(crate) fn new(map: &'a HashMap<K, V, S>) -> Self {
        Self {
            map,
            bucket: 0,
            at: 0
        }
    }
}

impl<'a, K, V, S> Iterator for Iter<'a, K, V, S> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => {
                    match bucket.get(self.at) {
                        Some(&(ref k, ref v)) => {
                            self.at += 1;
                            break Some((k, v));
                        },
                        None => {
                            self.bucket += 1;
                            self.at = 0;
                            continue;
                        },
                    }
                },
                None => break None
            }
        }
    }
} 

impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let map : HashMap<i32, i32> = HashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn contains_key() {
        let mut map : HashMap<i32, i32> = HashMap::new();
        
        let key = 128;
        assert!(!map.contains_key(&key));

        map.insert(128, 0);

        assert!(map.contains_key(&key));
    }

    #[test]
    fn iteration() {
        let mut map : HashMap<i32, i32> = HashMap::new();
        map.insert(0, 0);
        map.insert(1, 1);
        map.insert(2, 2);

        for (&k, &v) in &map {
            match k {
                0 => assert_eq!(v, 0),
                1 => assert_eq!(v, 1),
                2 => assert_eq!(v, 2),
                _ => unreachable!(),
            }
        }

        assert_eq!((&map).into_iter().count(), 3);
    }
}
