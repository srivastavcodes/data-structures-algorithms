#![allow(unused)]

use std::num;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let half = nums.len().div_ceil(2);

    let mut curr = (0, 0);
    for num in nums {
        if curr.1 == 0 {
            curr = (num, 0);
        }
        if curr.1 >= half {
            return curr.0;
        }
        if curr.0 != num {
            curr.1 -= 1;
            continue;
        }
        curr.1 += 1;
    }
    curr.0
}
