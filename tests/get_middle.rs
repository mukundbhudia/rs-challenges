// From: https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust
// You are going to be given a word. Your job is to return the middle character
// of the word. If the word's length is odd, return the middle character.
// If the word's length is even, return the middle 2 characters.

pub fn get_middle(s:&str) -> &str {
    let mid_point = s.len()/2;
    match s.len() % 2 {
        0 => &s[mid_point - 1..mid_point + 1],
        1 => &s[mid_point..mid_point + 1],
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod get_middle_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(get_middle("test"),"es");
        assert_eq!(get_middle("testing"),"t");
        assert_eq!(get_middle("middle"),"dd");
        assert_eq!(get_middle("A"),"A");
        assert_eq!(get_middle("of"),"of");
    }
}
