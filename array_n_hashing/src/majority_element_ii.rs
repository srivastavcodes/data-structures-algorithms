#![allow(dead_code)]

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    use std::collections::HashSet;

    if nums.len() < 3 {
        return nums.into_iter().collect::<HashSet<_>>().into_iter().collect();
    }
    let mut counts = HashMap::new();
    let mut res = HashSet::new();
    let threshold = nums.len() / 3;

    for key in nums {
        let count = counts.entry(key).or_insert(1);
        *count += 1;
        if *count > threshold {
            res.insert(key);
        }
    }
    res.into_iter().collect()
}
