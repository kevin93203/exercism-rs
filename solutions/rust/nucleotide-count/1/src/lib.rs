use std::collections::HashMap;

fn is_valid_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

fn is_valid_dna(dna: &str) -> (bool, char) {
    for c in dna.chars() {
        if !is_valid_nucleotide(c) {
            return (false, c);
        }
    }
    (true, '1')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    let (is_valid, n) = is_valid_dna(dna);
    if !is_valid {
        return Err(n);
    }

    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let (is_valid, n) = is_valid_dna(dna);
    if !is_valid {
        return Err(n);
    }

    let mut counts = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    counts.insert('T', 0);
    for c in dna.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    Ok(counts)
}
