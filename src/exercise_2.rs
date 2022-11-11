use is_vowel;
use is_vowel::IsRomanceVowel;

pub fn pig_latin(s: &str) -> String {
    let mut chars = s.chars();
    let first_letter = chars.next();
    match first_letter {
        Some(f_letter) => {
            if (f_letter.is_romance_vowel()) {
                let mut s0 = s.to_string();
                s0.push_str("-hay");
                return s0;
            } else {
                let mut res = String::new();
                for c in chars {
                    res.push(c);
                }
                res.push(f_letter);
                res.push_str("-ay");
                return res;
            }
        },
        _ => return s.to_string(),
    };
}