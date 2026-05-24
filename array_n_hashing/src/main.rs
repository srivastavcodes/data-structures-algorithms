use crate::{
    concatenation_of_array::get_concatenation, contains_duplicate::contains_duplicate,
    longest_common_prefix::longest_common_prefix, majority_element::majority_element,
    remove_element::remove_element, two_sum::two_sum,
};

mod concatenation_of_array;
mod contains_duplicate;
mod design_hashset;
mod group_anagrams;
mod longest_common_prefix;
mod majority_element;
mod remove_element;
mod two_sum;
mod valid_anagram;

fn main() {
    let arr = Vec::from_iter(6..=9);
    let res = get_concatenation(arr);
    println!("result={:?}", res);

    let arr = [1, 2, 3, 1].to_vec();
    let res = contains_duplicate(arr);
    println!("result={:?}", res);

    // Two sum
    let arr = [2, 7, 11, 15].to_vec();
    let res = two_sum(arr, 14);
    println!("result={:?}", res);

    // Longest common prefix
    let arr = [
        "flower".to_string(),
        "flop".to_string(),
        "flubbing".to_string(),
    ]
    .to_vec();
    let res = longest_common_prefix(arr);
    println!("result={:?}", res);

    // Remove Element
    let mut arr = [0, 1, 2, 2, 3, 0, 4, 2].to_vec();
    let res = remove_element(&mut arr, 2);
    println!("result={:?}", res);
    println!("nums={:?}", arr);

    let mut arr = [3, 2, 2, 3].to_vec();
    let res = remove_element(&mut arr, 3);
    println!("result={:?}", res);
    println!("nums={:?}", arr);

    // Majority Element
    let arr = [6, 5, 5].to_vec();
    let res = majority_element(arr);
    println!("majority={:?}", res);
}
