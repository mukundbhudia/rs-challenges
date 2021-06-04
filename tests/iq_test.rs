// From: https://www.codewars.com/kata/552c028c030765286c00007d/train/rust

pub fn iq_test(numbers: &str) -> u64 {
    let mut evens = vec![];
    let mut odds = vec![];

    for pair in numbers
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .enumerate()
    {
        if pair.1 % 2 != 0 {
            odds.push(pair);
        } else {
            evens.push(pair);
        }
    }

    if evens.len() == 1 {
        evens.pop().unwrap_or((0, 0)).0 as u64 + 1
    } else {
        odds.pop().unwrap_or((0, 0)).0 as u64 + 1
    }
}

#[cfg(test)]
mod digital_root_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(iq_test("2 4 7 8 10"), 3);
        assert_eq!(iq_test("1 2 2"), 1);
        assert_eq!(iq_test("1 2 1 1"), 2);
    }
}
