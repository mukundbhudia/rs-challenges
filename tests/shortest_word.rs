// From: https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9/train/rust
//
// Simple, given a string of words, return the length of the shortest word(s).
// String will never be empty and you do not need to account for different data types.

pub fn shortest_word(s: &str) -> u32 {
    s.split_whitespace()
        .map(|x| x.len())
        .min()
        .unwrap() as u32
}

#[cfg(test)]
mod shortest_word_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(shortest_word("bitcoin take over the world maybe who knows perhaps"), 3);
        assert_eq!(shortest_word("turns out random test cases are easier than writing out basic ones"), 3);
        assert_eq!(shortest_word("lets talk about javascript the best language"), 3);
        assert_eq!(shortest_word("i want to travel the world writing code one day"), 1);
        assert_eq!(shortest_word("Lets all go on holiday somewhere very cold"), 2);
    }
}
