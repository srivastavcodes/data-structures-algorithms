#![allow(unused)]

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
    for str in strs {
        // This count of alphabets frequency will work as keys for a map that will
        // hold the values as the anagrams vector.
        let mut count = [0u8; 26];
        // Counting the frequency of the alphabets.
        for b in str.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        // If the anagram-key (count) exists in the map, means we've encountered
        // this anagram before and it exists in the map so we'll just push this
        // string in the same key's value-vector, or else entry().or_default().push()
        // will create a new key-value entry.
        groups.entry(count).or_default().push(str);
    }
    groups.into_values().collect()
}
