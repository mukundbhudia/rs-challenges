// From: https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/rust

// Your task is to sort a given string. Each word in the string will
// contain a single number. This number is the position the word 
// should have in the result.

// Note: Numbers can be from 1 to 9. So 1 will be the first word (not 0).

//If the input string is empty, return an empty string. The words in the
// input String will only contain valid consecutive numbers.

pub fn word_order(sentence: &str) -> String {
    let mut word_index_pair = sentence
        .split_whitespace()
        .map(|x| {
            let index = x
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            (index, x)
        })
        .collect::<Vec<(u32, &str)>>();
    word_index_pair.sort_by_key(|k| k.0);
    word_index_pair
        .iter()
        .map(|x| x.1.to_string() + " ")
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod weight_for_weight_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(word_order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(word_order("4of Fo1r pe6ople g3ood th5e the2"), "Fo1r the2 g3ood 4of th5e pe6ople");
        assert_eq!(word_order(""), "");
    }
}
