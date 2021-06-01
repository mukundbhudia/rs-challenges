// From: https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust

pub fn digital_root(n: i64) -> i64 {
    let sum: i64 = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
        .into();

    match sum > 10 {
        true => digital_root(sum),
        false => sum,
    }
}

#[cfg(test)]
mod digital_root_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
    }
}
