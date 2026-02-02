use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut t = BTreeMap::new();
    for (&points, vector) in h.iter() {
        for letter in vector.iter() {
            t.insert(letter.to_ascii_lowercase(), points);
        }
    }
    t
}
