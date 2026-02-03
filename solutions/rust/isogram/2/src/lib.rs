use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .all(|c| seen.insert(c)) // seen.insert() return bool
}
