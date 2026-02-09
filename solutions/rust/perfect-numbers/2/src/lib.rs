use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    } else if num == 1 {
        return Some(Classification::Deficient);
    }

    let sum: u64 = (1..=(num as f64).sqrt() as u64)
            .filter(|&i| num.is_multiple_of(i))
            .map(|i|{
                if i == 1 || i * i == num {1} // exclude num
                else {i + num / i}
            })
            .sum();
    
    match sum.cmp(&num) {
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Equal => Some(Classification::Perfect),
    }
}
