// From: https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust
// Create a function (or write a script in Shell) that takes an integer as
// an argument and returns "Even" for even numbers or "Odd" for odd numbers.

pub fn even_or_odd(i: i32) -> &'static str {
    match (i % 2).abs() {
        0 => "Even",
        1 => "Odd",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod even_or_odd_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(even_or_odd(0), "Even");
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(1), "Odd");
        assert_eq!(even_or_odd(7), "Odd");
        assert_eq!(even_or_odd(-1), "Odd");
    }
}
