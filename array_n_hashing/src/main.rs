use crate::{
    best_time_buy_sell_stock_ii::max_profit,
    concatenation_of_array::get_concatenation,
    contains_duplicate::contains_duplicate,
    encode_decode_strings::{decode, encode},
    first_missing_positive::first_missing_positive,
    longest_common_prefix::longest_common_prefix,
    longest_consecutive_seq::longest_consecutive,
    majority_element::majority_element,
    majority_element_ii::majority_element as majority_element_ii,
    product_arr_except_self::product_except_self,
    remove_element::remove_element,
    sort_colors::sort_colors,
    two_sum::two_sum,
};

mod best_time_buy_sell_stock_ii;
mod concatenation_of_array;
mod contains_duplicate;
mod design_hashmap;
mod design_hashset;
mod encode_decode_strings;
mod first_missing_positive;
mod group_anagrams;
mod longest_common_prefix;
mod longest_consecutive_seq;
mod majority_element;
mod majority_element_ii;
mod product_arr_except_self;
mod range_sum_query_2d;
mod remove_element;
mod sort_an_array;
mod sort_colors;
mod subarray_sum_eq_k;
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
    let arr = ["flower".to_string(), "flop".to_string(), "flubbing".to_string()].to_vec();
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

    // Sort Colors
    let mut arr = [2, 0, 2, 1, 1, 0].to_vec();
    sort_colors(&mut arr);
    println!("colors={:?}", arr);

    // Encode Decode Strings
    let input = [
        "Hello".to_string(),
        "World".to_string(),
        "long string this is more than 10 chars".to_string(),
    ]
    .to_vec();
    let encoded = encode(input);
    let output = decode(encoded);
    println!("output={:?}", output);

    // Product array except self
    let arr = [-1, 1, 0, -3, 3].to_vec();
    let output = product_except_self(arr);
    println!("product={:?}", output);

    // Longest consecutive sequence
    let arr = [2, 20, 4, 10, 3, 4, 5].to_vec();
    let output = longest_consecutive(arr);
    println!("longest={:?}", output);

    // Best Time to Buy and Sell Stocks II
    let arr = [0].to_vec();
    let output = max_profit(arr);
    println!("profit={:?}", output);

    // Majority Elements II
    let arr = [3, 2, 3, 4, 2, 2].to_vec();
    let output = majority_element_ii(arr);
    println!("majorities={:?}", output);

    // First Missing Positive
    let arr = [1, 2, 3].to_vec();
    let output = first_missing_positive(arr);
    println!("positive={:?}", output);
}
