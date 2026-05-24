#![allow(unused)]

struct MyHashSet {
    buckets: Vec<Vec<i32>>,
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 10_000],
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % 10_000
    }

    fn add(&mut self, key: i32) {
        let index = Self::hash(key);
        if !self.buckets[index].contains(&key) {
            self.buckets[index].push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let index = Self::hash(key);
        if let Some(offset) = self.buckets[index].iter().position(|&x| x == key) {
            self.buckets[index].remove(offset);
        }
    }

    fn contains(&self, key: i32) -> bool {
        let index = Self::hash(key);
        self.buckets[index].contains(&key)
    }
}

// struct MyHashSet {
//     values: [bool; 1_000_000],
// }

// impl MyHashSet {
//     fn new() -> Self {
//         Self {
//             values: [false; 1_000_000],
//         }
//     }

//     fn add(&mut self, key: i32) {
//         self.values[key as usize] = true
//     }

//     fn remove(&mut self, key: i32) {
//         self.values[key as usize] = false
//     }

//     fn contains(&self, key: i32) -> bool {
//         self.values[key as usize]
//     }
// }
