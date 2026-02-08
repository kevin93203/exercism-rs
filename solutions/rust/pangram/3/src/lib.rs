const A_LOWER_ASCII: u8 = b'a';

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut flags: u32 = 0;
    for c in sentence.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            flags |=  1 << (c as u32 - A_LOWER_ASCII as u32);
        }
    }

    flags == (1 << 26) - 1
}
