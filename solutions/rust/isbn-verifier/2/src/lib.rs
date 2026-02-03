/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut numbers = Vec::<u32>::new();
    for c in isbn.chars() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => numbers.push(c.to_digit(10).unwrap()),
            'X' if numbers.len() == 9 => numbers.push(10),
            '-' => {},
            _ => return false,
        } 
    }
    
    if numbers.len() != 10 {
        return false
    }

    let sum: u32 = numbers.iter()
        .enumerate()
        .map(|(i, n)| n * (10 - (i as u32)))
        .sum();
    
    sum.is_multiple_of(11)
}
