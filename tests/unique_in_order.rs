// From: https://www.codewars.com/kata/54e6533c92449cc251001667/train/rust

// Implement the function unique_in_order which takes as argument a
// sequence and returns a list of items without any elements with
// the same value next to each other and preserving the original order of elements.

pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug, <T as std::iter::IntoIterator>::Item: std::fmt::Display

{
    sequence
        .into_iter()
        .fold(vec![], |mut acc, item| {
            if acc.len() == 0 {
                acc.push(item);
            } else if acc[acc.len() - 1] != item {
                acc.push(item);
            }
            acc
        })
}

#[cfg(test)]
mod general_tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}
