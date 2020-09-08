// From: https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/rust

// Implement the function unique_in_order which takes as argument a
// sequence and returns a list of items without any elements with
// the same value next to each other and preserving the original order of elements.

pub fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

#[cfg(test)]
mod repeat_str_tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(repeat_str("a", 4), "aaaa");
        assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
        assert_eq!(repeat_str("abc", 2), "abcabc");
    }
}
