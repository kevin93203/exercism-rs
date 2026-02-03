/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;
    for c in isbn.chars() {
        match c {
            '0'..='9' => {
                sum += c.to_digit(10).unwrap() * (10 - len);
                len += 1;
            },
            'X' if len == 9 => {
                sum += 10;
                len += 1;
            },
            '-' => continue,
            _ => return false,
        }
        if len > 10 { return false; }
    }
    
    len == 10 && sum % 11 == 0
}
