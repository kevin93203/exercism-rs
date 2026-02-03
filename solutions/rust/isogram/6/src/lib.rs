// Use Binary Flags and Bit Operation

const A_LCASE:u8 = b'a';

pub fn check(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|b| 1u32 << b.to_ascii_lowercase() - A_LCASE)
        .try_fold(0, |flags, ltr|{ // if None will immediately return None
            (flags & ltr == 0).then(|| flags | ltr) // if true return flags | ltr, else None
        })
        .is_some()
}
