// From: https://www.codewars.com/kata/5168bb5dfe9a00b126000018/train/rust

pub fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(solution("world"), "dlrow");
    }
}
