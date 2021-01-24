// From: https://edabit.com/challenge/M7Xzr3ez5xBPTu3bq

pub fn triangle(a: u32) -> u32 {
    (1..a + 1).sum()
}

#[cfg(test)]
mod triangle_numbers_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(triangle(1), 1);
        assert_eq!(triangle(2), 3);
        assert_eq!(triangle(3), 6);
        assert_eq!(triangle(8), 36);
        assert_eq!(triangle(1), 1);
        assert_eq!(triangle(2153), 2318781);
    }
}
