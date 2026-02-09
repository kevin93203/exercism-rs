
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
            for (i, c) in word.char_indices() { 
                if VOWELS.contains(&c) || (c == 'y' && i > 0) {
                    // Rule3
                    if i > 0 && word[..i].ends_with('q') && c == 'u' {
                        // 使用位元組索引 i+1 是安全的，因為 'u' 固定佔 1 byte
                        pig_latin_word.push_str(&(word[i+1..].to_string() + &word[..i+1] + "ay"));
                    } else { 
                        // 現在 i 是正確的 byte index，可以直接用於切片
                        pig_latin_word.push_str(&(word[i..].to_string() + &word[..i] + "ay"));
                    }
                    break;
                }
            }
        }
        pig_latin_words.push(pig_latin_word);
    }
    pig_latin_words.join(" ")
}
