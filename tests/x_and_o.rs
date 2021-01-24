// From: https://www.codewars.com/kata/55908aad6620c066bc00002a/train/rust

// Check to see if a string has the same amount of 'x's and 'o's.
// The method must return a boolean and be case insensitive.
// The string can contain any char.

pub fn xo(string: &'static str) -> bool {
    string
        .chars()
        .filter(|c| {
            let x = c.to_lowercase().to_string();
            x == "x"
        })
        .count()
        == string
            .chars()
            .filter(|c| {
                let o = c.to_lowercase().to_string();
                o == "o"
            })
            .count()
}

#[cfg(test)]
mod x_and_o_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(xo("xo"), true);
        assert_eq!(xo("Xo"), true);
        assert_eq!(xo("xxOo"), true);
        assert_eq!(xo("xxxm"), false);
        assert_eq!(xo("Oo"), false);
        assert_eq!(xo("ooom"), false);
    }
}
