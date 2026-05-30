#![allow(dead_code)]

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if let Some(&value) = map.get(&(target - num)) {
            return vec![i as i32, value];
        }
        map.insert(num, i as i32);
    }
    vec![]
}
