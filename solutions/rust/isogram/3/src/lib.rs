// Use Binary Flags and Bit Operation

const A_LCASE:u8 = 'a' as u8; 
const Z_LCASE:u8 = 'z' as u8;
const A_UCASE:u8 = 'A' as u8;
const Z_UCASE:u8 = 'Z' as u8;

pub fn check(candidate: &str) -> bool {
    let mut letter_flags = 0u32; //32 bits flags to store seen 26 letters

    for b in candidate.bytes() {
        if b >= A_LCASE && b <= Z_LCASE {  // Lower Case
            if letter_flags & (1 << (b - A_LCASE)) != 0 {
                return false;
            }

            letter_flags |= (1 << (b - A_LCASE));

        } else if b >= A_UCASE && b <= Z_UCASE {  // Upper Case
            if letter_flags & (1 << (b - A_UCASE)) != 0 {
                return false;
            }

            letter_flags |= (1 << (b - A_UCASE));
        }
    }
    true
}
