// From: https://www.codewars.com/kata/57eae20f5500ad98e50002c5/train/rust

// Implement the function unique_in_order which takes as argument a
// sequence and returns a list of items without any elements with
// the same value next to each other and preserving the original order of elements.

pub fn no_space(x: String) -> String{
    x.split_whitespace().collect()
}

#[cfg(test)]
mod no_space_tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!("8j8mBliB8gimjB8B8jlB", no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string()));
        assert_eq!("88Bifk8hB8BB8BBBB888chl8BhBfd", no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string()));
        assert_eq!("8aaaaaddddr", no_space("8aaaaa dddd r     ".to_string()));
        assert_eq!("jfBmgklf8hg88lbe8", no_space("jfBm  gk lf8hg  88lbe8 ".to_string()));
        assert_eq!("8jaam", no_space("8j aam".to_string()));
    }
}
