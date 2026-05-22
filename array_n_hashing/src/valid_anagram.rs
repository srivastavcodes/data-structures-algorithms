#![allow(unused)]

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut count = vec![0; 26];

    for (a, b) in s.bytes().zip(t.bytes()) {
        count[(a - b'a') as usize] += 1;
        count[(b - b'a') as usize] -= 1;
    }
    if count.iter().all(|&v| v == 0) {
        return true;
    }
    false
}
