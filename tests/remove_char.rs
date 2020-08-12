// From: https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust
// Remove the first and last characters of a string

pub fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

#[cfg(test)]
mod remove_char_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(remove_char("eloquent"), "loquen");
        assert_eq!(remove_char("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }
}
