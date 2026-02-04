use std::collections::HashMap;
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .get(&nucleotide)
        .ok_or(nucleotide)
        .copied()
}
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let counts: HashMap<char, usize> = ['A', 'C', 'G', 'T']
        .into_iter()
        .map(|n| (n, 0))
        .collect();

    dna.chars().try_fold(counts, |mut acc, c| {
        match acc.get_mut(&c) {
            Some(count) => {
                *count += 1;
                Ok(acc)
            }
            None => Err(c),
        }
    })
}