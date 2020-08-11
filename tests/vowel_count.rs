// From: https://www.codewars.com/kata/54ff3102c1bad923760001f3/train/rust

// Return the number (count) of vowels in the given string.
// We will consider a, e, i, o, u as vowels for this Kata (but not y).
// The input string will only consist of lower case letters and/or spaces.

pub fn vowel_count(string: &str) -> usize {
    string.chars().fold(0, |n, i| {
        if "aeiou".contains(i) {
            n + 1
        } else {
            n
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(vowel_count("abracadabra"), 5);
    }
}
