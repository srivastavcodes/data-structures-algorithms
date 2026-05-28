#![allow(dead_code)]

use std::collections::HashMap;

// Do again
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut curr_sum = 0;
    let mut result = 0;
    let mut prefix_sums = HashMap::new();

    prefix_sums.insert(0, 1);
    for &num in nums.iter() {
        curr_sum += num;
        let diff = curr_sum - k;
        result += prefix_sums.get(&diff).unwrap_or(&0);
        *prefix_sums.entry(curr_sum).or_insert(0) += 1;
    }
    result
}
