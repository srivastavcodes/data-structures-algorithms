#![allow(unused)]

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    // Assume the first string to be the answer and then later check if the prefix
    // continues to match for the later coming strings, if the prefix is not
    // matching pop out the last letters from prefix until it starts to match all
    // of the other strings in the vector.
    let mut prefix = strs[0].clone();

    for s in strs[1..].iter() {
        while !s.starts_with(&prefix) {
            prefix.pop();
            // If we exhaust the prefix before we have finished iterating over all
            // the vector elements, return the empty prefix early.
            if prefix.is_empty() {
                return prefix;
            }
        }
    }
    prefix
}
