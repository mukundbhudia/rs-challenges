// From: https://www.codewars.com/kata/55a2d7ebe362935a210000b2/train/rust

// Given an array of integers your solution should find the smallest integer.

pub fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

#[cfg(test)]
mod find_smallest_int_tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
        assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
    }
}
