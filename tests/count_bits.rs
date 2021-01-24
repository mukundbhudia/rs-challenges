// From: https://www.codewars.com/kata/526571aae218b8ee490006f4/train/rust
//
// Write a function that takes an integer as input, and returns the number
// of bits that are equal to one in the binary representation of that number.
// You can guarantee that input is non-negative.
// Example: The binary representation of 1234 is 10011010010, so the function should return 5 in this case.

pub fn count_bits(n: i64) -> u32 {
    format!("{:b}", n).rmatches("1").count() as u32
}

#[cfg(test)]
mod count_bits_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(7), 3);
        assert_eq!(count_bits(9), 2);
        assert_eq!(count_bits(10), 2);
    }
}
