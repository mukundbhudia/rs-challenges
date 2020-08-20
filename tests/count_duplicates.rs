// From: https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/rust

// Write a function that will return the count of distinct case-insensitive
// alphabetic characters and numeric digits that occur more than once in the
// input string. The input string can be assumed to contain only alphabets
// (both uppercase and lowercase) and numeric digits.

pub fn count_duplicates(text: &str) -> u32 {
    let mut duplicates = String::new();
    for c in text.chars() {
        let lowercase_char = c.to_lowercase().to_string();
        if c.is_alphanumeric()
            && text.matches(|b: char| 
                c.to_lowercase().to_string() == b.to_string()
                || c.to_uppercase().to_string() == b.to_string()
            ).count() > 1
            && !duplicates.contains(&lowercase_char)
        {
            duplicates.push_str(&lowercase_char);
        }
    }
    duplicates.len() as u32
}

#[cfg(test)]
mod multiplicative_persistence_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(count_duplicates("abcde"), 0);
        assert_eq!(count_duplicates("abcdea"), 1);
        assert_eq!(count_duplicates("indivisibility"), 1);
        assert_eq!(count_duplicates("abcdeaB"), 2);
        assert_eq!(count_duplicates("codewarsisawesome"), 5);
    }
}
