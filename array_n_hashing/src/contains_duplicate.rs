pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = std::collections::HashSet::new();
    for num in nums {
        if !set.insert(num) {
            return true;
        }
    }
    false
}

pub fn _contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = std::collections::HashSet::new();
    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }
    false
}
