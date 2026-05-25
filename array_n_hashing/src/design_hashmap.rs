#![allow(unused)]

struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 1000],
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % 1000
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = Self::hash(key);
        for pair in self.buckets[index].iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        self.buckets[index].push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        let index = Self::hash(key);
        for pair in self.buckets[index].iter() {
            if pair.0 == key {
                return pair.1;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let index = Self::hash(key);
        if let Some(offset) = self.buckets[index].iter().position(|&x| x.0 == key) {
            self.buckets[index].remove(offset);
        }
    }
}
