// From: https://www.codewars.com/kata/552c028c030765286c00007d/train/rust

pub fn iq_test(numbers: &str) -> u64 {
    match numbers
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .enumerate()
        .find(|(_, x)| x % 2 != 0)
    {
        Some((i, _)) => i as u64 + 1,
        None => 0,
    }
}

#[cfg(test)]
mod digital_root_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(iq_test("2 4 7 8 10"), 3);
        assert_eq!(iq_test("1 2 2"), 1);
        // assert_eq!(iq_test("1 2 1 1"), 2);
    }
}
