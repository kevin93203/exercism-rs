/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let flags = sentence.to_lowercase()
        .bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .fold(0u32, |acc, b| {
            acc | 1 << (b - b'a')
        });

    flags == (1 << 26) -1 
}
