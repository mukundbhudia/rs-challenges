// From: https://www.codewars.com/kata/515e271a311df0350d00000f/train/rust

// Complete the square sum function so that it squares each number passed
//into it and then sums the results together.

// For example, for `[1, 2, 2]` it should return `9` because `1^2 + 2^2 + 2^2 = 9`.

pub fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x * x).sum()
}

#[cfg(test)]
mod square_sum_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
    }
}
