// From: https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec

// Write a function, persistence, that takes in a positive parameter
// num and returns its multiplicative persistence, which is the number
// of times you must multiply the digits in num until you reach a single digit.

pub fn persistence(num: u64) -> u64 {
    let mut i = 0;
    let mut num = num;
    while num.to_string().len() > 1 {
        num = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .product::<u64>();
        i += 1;
    }
    i
}

#[cfg(test)]
mod multiplicative_persistence_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(persistence(39), 3);
        assert_eq!(persistence(4), 0);
        assert_eq!(persistence(25), 2);
        assert_eq!(persistence(999), 4);
    }
}
