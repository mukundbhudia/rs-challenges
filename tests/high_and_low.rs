// From: https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust
//
//In this little assignment you are given a string of space separated numbers,
// and have to return the highest and lowest number.
// Example:
// `highAndLow("1 2 3 4 5");`  // return "5 1"
// `highAndLow("1 2 -3 4 5");` // return "5 -3"
// `highAndLow("1 9 3 4 -5");` // return "9 -5"
// Notes:
//  All numbers are valid Int32, no need to validate them.
//  There will always be at least one number in the input string.
//  Output string must be two numbers separated by a single space, and highest number is first.

pub fn high_and_low(numbers: &str) -> String {
    let numbers = numbers
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let max = numbers
        .iter()
        .max()
        .unwrap();
    let min = numbers
        .iter()
        .min()
        .unwrap();
    format!("{} {}", max, min)
}

#[cfg(test)]
mod high_and_low_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
        assert_eq!("5 1", high_and_low("1 2 3 4 5"));
        assert_eq!("5 -3", high_and_low("1 2 -3 4 5"));
        assert_eq!("9 -5", high_and_low("1 9 3 4 -5"));
    }
}
