#![allow(unused)]

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let (mut i, mut j) = (0, 0);

    'outer: while i < nums.len() && j < nums.len() {
        if nums[i] != nums[j] {
            let temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
            i += 1;
            j += 1;
            continue 'outer;
        }
        while nums[j] == val {
            j += 1;
            continue 'outer;
        }
        i += 1;
        j += 1;
    }
    i as i32
}
