#![allow(dead_code)]

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; nums.len()];
    let length = nums.len();

    let mut prefix = 1;
    for i in 0..length {
        result[i] = prefix;
        prefix *= nums[i];
    }
    let mut postfix = 1;
    for i in (0..length).rev() {
        result[i] *= postfix;
        postfix *= nums[i];
    }
    result
}
