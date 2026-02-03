use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::<char>::new();
    for c in candidate.chars() {
        if c.is_ascii_alphanumeric() {
            let lower = c.to_ascii_lowercase();
            if letters.contains(&lower) {
                return false;
            }
            letters.insert(lower);
        }
    }
    true
}
