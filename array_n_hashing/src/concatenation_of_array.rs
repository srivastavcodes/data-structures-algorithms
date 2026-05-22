#![allow(unused)]

pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; 2 * nums.len()];
    for i in 0..nums.len() {
        ans[i] = nums[i];
        ans[i + nums.len()] = nums[i];
    }
    ans
}

pub fn _get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len() * 2);
    for i in 0..nums.len() * 2 {
        ans.push(nums[i % nums.len()]);
    }
    ans
}
