// From: https://www.codewars.com/kata/56dec885c54a926dcd001095/train/rust

// Very simple, given a number, find its opposite.

pub fn opposite(number: i32) -> i32 {
    number * -1 // Also can do `-number`
}

#[cfg(test)]
mod opposite_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(opposite(1), -1);
        assert_eq!(opposite(-1), 1);
    }
}
