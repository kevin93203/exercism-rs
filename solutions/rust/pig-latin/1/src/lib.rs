
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const RULE1_PREFIXES: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];

pub fn translate(input: &str) -> String {
    let words: Vec<&str> = input.split(' ').collect();
    let mut pig_latin_words: Vec<String> = Vec::new();
    for word in words.iter() {
        let mut pig_latin_word = String::new();
        // Rule1
        if RULE1_PREFIXES.iter().any(|&p| word.starts_with(p)) {
            pig_latin_word.push_str(&(word.to_string() + "ay"));
        } else {
            for (i, c) in word.chars().enumerate() {
                if VOWELS.contains(&c) || (c == 'y' && i > 0) { // (c == 'y' && i > 0) is Rule4
                    // Rule3
                    if i >0 && word.chars().nth(i -1).unwrap() == 'q' && c == 'u' {
                        pig_latin_word.push_str(&(word[i+1..].to_string() + &word[..i-1].to_string() + "qu" + "ay"));
                    } else { // Rule2 + Rule4
                        pig_latin_word.push_str(&(word[i..].to_string() + &word[..i].to_string() + "ay"));
                    }
                    break;
                }
            }
        }
        pig_latin_words.push(pig_latin_word);
    }
    pig_latin_words.join(" ")
}
