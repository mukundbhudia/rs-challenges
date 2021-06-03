// From: https://www.codewars.com/kata/552c028c030765286c00007d/train/rust

pub fn iq_test(numbers: &str) -> u64 {
    // match numbers
    //     .split_whitespace()
    //     .map(|x| x.parse::<u64>().unwrap())
    //     .enumerate()
    //     .find(|(_, x)| x % 2 != 0)
    // {
    //     Some((i, _)) => i as u64 + 1,
    //     None => 0,
    // }

    let digits = numbers
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .enumerate()
        // .map(|(_, x)| if x % 2 != 0 { true } else { false} )
        .collect::<Vec<(usize, u64)>>();

    let mut is_prev_odd = digits[0].1 % 2 != 0;
    for i in 1..digits.len() {
        let pair = digits[i];
        let is_current_odd = pair.1 % 2 != 0;
        println!("{:?}, prev odd: {}, current odd: {}", pair, is_prev_odd, is_current_odd);
        if is_current_odd != is_prev_odd {
            return pair.0 as u64 + 1
        }
        is_prev_odd = is_current_odd;
    }
    println!("{:?}", digits);
    0
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
