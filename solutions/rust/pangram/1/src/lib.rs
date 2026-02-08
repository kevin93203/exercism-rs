use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut counts: HashMap<char, usize> = ('a'..='z').map(|c| (c, 0)).collect();
    for c in sentence.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            let count = counts.get_mut(&c).unwrap();
            *count += 1;
        }
    }

    counts.values().all(|&v| v > 0)
}
