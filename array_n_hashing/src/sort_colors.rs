#![allow(dead_code)]

pub fn sort_colors(nums: &mut Vec<i32>) {
    // The index zero and one will point to the respective indices where the value
    // of 0 and 1 elements should be. The value 2 will keep getting written and
    // re-written with small values until there are none, which by then means the
    // array is sorted.
    // See the printed results for better understanding.
    let (mut zero, mut one) = (0, 0);
    for two in 0..nums.len() {
        println!("{:?}", nums);
        let tmp = nums[two];
        nums[two] = 2;
        if tmp < 2 {
            nums[one] = 1;
            one += 1;
        }
        if tmp < 1 {
            nums[zero] = 0;
            zero += 1;
        }
    }
}
