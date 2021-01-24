// From: https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust

// You get an array of numbers, return the sum of all of the positives ones.
// Example `[1,-4,7,12]` => `1 + 7 + 12 = 20`
// Note: if there is nothing to sum, the sum is default to 0.

pub fn positive_sum(arr: &[i32]) -> i32 {
    arr.iter().filter(|x| x.is_positive()).sum()
}

#[cfg(test)]
mod positive_sum_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
        assert_eq!(positive_sum(&[]), 0);
        assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }
}
