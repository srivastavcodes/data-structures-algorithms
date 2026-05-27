#![allow(dead_code)]

use std::{cmp::max, collections::HashSet};

/// Use a HashSet for O(1) lookups while checking for consecutive elements.
/// We'll always start looking for elements that are start of a sequence;
/// meaning (n - 1) shouldn't exist in the HashSet. This way we only need
/// to visit sequential elements linearly once.
///
/// So for `[10, 200, 4, 100, 2, 3, 1]`; we only ever start lookups for 10,
/// 200, 100, and 1 -> (which is the actual longest sequence).
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.iter().cloned().collect();
    let mut longest = 0;

    'outer: for &num in set.iter() {
        if set.contains(&(num - 1)) {
            continue 'outer;
        }
        let mut count = 1;
        while set.contains(&(num + count)) {
            count += 1;
        }
        longest = max(longest, count)
    }
    longest
}
