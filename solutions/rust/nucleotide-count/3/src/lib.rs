use std::collections::HashMap;

fn is_valid(nucleotide: char) -> bool {
    matches!(nucleotide, 'A' | 'C' | 'G' | 'T')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) {
        return Err(nucleotide);
    }
    
    dna.chars().try_fold(0, |acc, c| {
        if !is_valid(c) {
            Err(c)
        } else if c == nucleotide {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect();

    for c in dna.chars() {
        if let Some(count) = counts.get_mut(&c) {
            *count += 1;
        } else {
            return Err(c);
        }
    }
    Ok(counts)
}