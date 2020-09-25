// From: https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust

// Your task is to write a function maskify, which changes all but the last four characters into '#'.

pub fn maskify(cc: &str) -> String {
    cc.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| if i > 3 { '#' } else { c })
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

#[cfg(test)]
mod credit_card_mask_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
