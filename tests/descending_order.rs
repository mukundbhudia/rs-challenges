// From: https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust

pub fn descending_order(x: u64) -> u64 {
    let mut sorted_digits = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    sorted_digits.sort_by(|a, b| b.cmp(a));

    sorted_digits
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod descending_order_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}
