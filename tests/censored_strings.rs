// From: https://edabit.com/challenge/Wv9ZeXyC32EMfRWGB

pub fn uncensor(sentence: &str, chars_to_censor: &str) -> String {
    let sentence = sentence.to_string();
    let mut chars_to_censor = chars_to_censor.to_string();

    sentence
        .chars()
        .map(|c|
            if c == '*' {
                chars_to_censor.remove(0)
            } else {
                c
            }
        )
        .collect::<String>()
}

#[cfg(test)]
mod censored_strings_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(uncensor("Wh*r* d*d my v*w*ls g*?", "eeioeo"), "Where did my vowels go?");
        assert_eq!(uncensor("abcd", ""), "abcd");
        assert_eq!(uncensor("*PP*RC*S*", "UEAE"), "UPPERCASE");
    }
}
