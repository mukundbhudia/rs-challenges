// From: https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/rust

// Write a method that takes an array of consecutive (increasing) letters
// as input and that returns the missing letter in the array.
// You will always get an valid array. And it will be always exactly one
// letter be missing. The length of the array will always be at least 2.
// The array will always contain letters in only one case.

pub fn find_missing_letter(chars: &[char]) -> char {
    let mut alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let first_char = chars[0];
    if first_char.is_uppercase() {
        alphabet = alphabet.to_uppercase();
    }
    let first_pos = alphabet.find(first_char).unwrap_or_default();
    alphabet
        .get(first_pos..first_pos + chars.len() + 1)
        .unwrap_or_default()
        .to_string()
        .chars()
        .filter(|x| !chars.contains(x))
        .collect::<String>()
        .pop()
        .unwrap_or_default()
}

#[cfg(test)]
mod find_missing_letter_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}
