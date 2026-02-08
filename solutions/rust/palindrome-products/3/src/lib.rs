use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    min: u64,
    max: u64,
}

impl Palindrome {
    pub fn new(value: u64, min: u64, max: u64) -> Palindrome {
        Palindrome {value, min, max}
    }
    
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let mut factors = HashSet::new();
        for i in self.min..=self.max {
            if i * i > self.value {break;}
            if self.value.is_multiple_of(i) {
                let j = self.value / i;
                if j >= self.min && j <= self.max {
                    factors.insert((i, j));
                }
            }
        }
        factors
    }
}

fn is_palindrome(x: u64) -> bool {
    if x == 0 {
        return false;
    }
    
    let mut original = x;
    let mut reversed = 0;

    while original > 0 {
        let digit = original % 10;
        reversed = reversed * 10 + digit;
        original /= 10;
    }

    x == reversed
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes = Vec::new();
    for i in min..=max {
        for j in i..=max {
            if is_palindrome(i * j) {
                palindromes.push(i * j);
            }
        }
    }

    if palindromes.is_empty(){
        return None;
    }

    let max_value = palindromes.iter().max().unwrap();
    let min_value = palindromes.iter().min().unwrap();

    Some((Palindrome::new(*min_value, min, max), Palindrome::new(*max_value, min, max)))
}