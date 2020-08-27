// From: https://www.codewars.com/kata/55c6126177c9441a570000cc/train/rust

// My friend John and I are members of the "Fat to Fit Club (FFC)".
// John is worried because each month a list with the weights of
// members is published and each month he is the last on the list
// which means he is the heaviest.
// I am the one who establishes the list so I told him: "Don't
// worry any more, I will modify the order of the list". It was
// decided to attribute a "weight" to numbers. The weight of a
// number will be from now on the sum of its digits.
// For example 99 will have "weight" 18, 100 will have "weight"
// 1 so in the list 100 will come before 99. Given a string with
// the weights of FFC members in normal order can you give this
// string ordered by "weights" of these numbers?

pub fn weight_for_weight(s: &str) -> String {
    let mut numbers = s.split_whitespace()
        .map(|x| (
            x, x.chars()
                .map(|y| y.to_digit(10)
                .unwrap())
                .sum())
            )
        .collect::<Vec<(&str, u32)>>();
    numbers.sort_by_key(|k| k.1);
    let numbers = numbers
        .iter()
        .map(|x| x.0.to_string() + " ")
        .collect::<String>();
    numbers.trim_end().to_string()
}

#[cfg(test)]
mod weight_for_weight_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(weight_for_weight("103 123 4444 99 2000"), "2000 103 123 4444 99");
        assert_eq!(weight_for_weight("2000 10003 1234000 44444444 9999 11 11 22 123"), "2000 11 11 10003 22 123 1234000 44444444 9999");
    }
}
