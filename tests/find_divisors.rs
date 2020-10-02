// From: https://www.codewars.com/kata/544aed4c4a30184e960010f4/train/rust

// Create a function named divisors/Divisors that takes an integer n > 1
// and returns an array with all of the integer's divisors(except for 1
// and the number itself), from smallest to largest. If the number is
// prime return the string '(integer) is prime' (null in C#) (use Either
// String a in Haskell and Result<Vec<u32>, String> in Rust).

pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let output = (2..integer - 1)
        .filter(|i| integer % i == 0)
        .collect::<Vec<_>>();
    if output.is_empty() {
        Err(format!("{} is prime", integer))
    } else {
        Ok(output)
    }
}

#[cfg(test)]
mod get_middle_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(divisors(15), Ok(vec![3, 5]));
        assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
        assert_eq!(divisors(13), Err("13 is prime".to_string()));
    }
}
