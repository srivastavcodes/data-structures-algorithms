#![allow(dead_code)]

use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut counts = HashMap::new();
    for &num in nums.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }
    let len = nums.len();
    counts.into_iter().filter(|&(_, val)| val > len / 3).map(|(key, _)| key).collect()
}
