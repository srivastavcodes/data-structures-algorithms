use crate::{concatenation_of_array::get_concatenation, contains_duplicate::contains_duplicate};

mod concatenation_of_array;
mod contains_duplicate;
mod valid_anagram;

fn main() {
    let arr = Vec::from_iter(6..=9);
    let res = get_concatenation(arr);
    println!("result={:?}", res);

    let arr = [1, 2, 3, 1].to_vec();
    let res = contains_duplicate(arr);
    println!("result={:?}", res);
}
