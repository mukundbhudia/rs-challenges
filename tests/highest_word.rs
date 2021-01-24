// From: https://www.codewars.com/kata/57eb8fcdf670e99d9b000272/train/rust

// Given a string of words, you need to find the highest scoring word.
// Each letter of a word scores points according to its position in
// the alphabet: a = 1, b = 2, c = 3 etc.
// You need to return the highest scoring word as a string.
// If two words score the same, return the word that appears earliest
// in the original string.
// All letters will be lowercase and all inputs will be valid.

pub fn high(input: &str) -> &str {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut scores = input
        .split_whitespace()
        .map(|w| {
            (
                w.chars()
                    .fold(0, |sum, c| sum + alphabet.find(c).unwrap() + 1),
                w,
            )
        })
        .collect::<Vec<(usize, &str)>>();
    scores.sort_by_key(|k| k.0);
    scores.pop().unwrap_or((0, "")).1
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aaa b"), "aaa");
        assert_eq!(high(""), "");
    }
}
