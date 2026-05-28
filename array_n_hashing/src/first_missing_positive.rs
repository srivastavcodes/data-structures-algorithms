#![allow(dead_code)]

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut i = 0;

    while i < len {
        if nums[i] <= 0 || nums[i] as usize > len {
            i += 1;
            continue;
        }
        let idx = nums[i] as usize - 1;

        if nums[i] != nums[idx] {
            nums.swap(i, idx);
        } else {
            i += 1;
        }
    }
    for i in 0..len {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    (len + 1) as i32
}
