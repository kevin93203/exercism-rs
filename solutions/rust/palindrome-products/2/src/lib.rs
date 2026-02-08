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
            for j in i..=self.max {
                if self.value == i * j {
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
    let mut min_value: Option<u64> = None;
    let mut max_value: Option<u64> = None;

    for i in min..=max {
        for j in i..=max {
            let prod = i * j;
            if let Some(v) = min_value {
                if prod >= v {break;}
            }

            if is_palindrome(prod){
                min_value = Some(prod);
            }
        }
    }

    for i in (min..=max).rev() {
        for j in (i..=max).rev() {
            let prod = i * j;
            if let Some(v) = max_value {
                if prod <= v {break;}
            }

            if is_palindrome(prod) {
                max_value = Some(prod);
            }
        }
    }

    match (min_value, max_value) {
        (Some(low), Some(high)) => Some((
                Palindrome::new(low, min, max), 
                Palindrome::new(high, min, max),
        )),
        _ => None,
    }
}
