#![allow(dead_code)]

pub fn encode(strs: Vec<String>) -> String {
    let mut str = String::new();
    // Suppose the raw String is ["Hello", "World"], the encode format makes it be
    // "5#Hello5#World". So when later we decode it, we read and parse the length
    // till we encounter our own '#' and then we skip the '#' and extract the str
    // '#'+1 till the length.
    for s in strs.iter() {
        str.push_str(&s.len().to_string());
        str.push('#');
        str.push_str(s);
    }
    str
}

pub fn decode(str: String) -> Vec<String> {
    let mut result = vec![];
    let bytes = str.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let mut j = i;
        // In "100#(some long string)" 'i' will be at 1 and j will be at 0 after
        //    i^j^
        // the inner loop ends.
        while bytes[j] != b'#' {
            j += 1;
        }
        // parse converts the bytes into actual integer representing length.
        let length: usize = str[i..j].parse().unwrap();
        // i moves next to '#' since that's where the actual string starts.
        i = j + 1;
        // j moves to where the string being decoded ends+1 according to the
        // length.
        j = i + length;
        result.push(str[i..j].to_string());
        // prepare for the next string.
        i = j;
    }
    result
}
