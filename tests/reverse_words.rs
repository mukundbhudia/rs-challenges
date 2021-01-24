// From: https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust

pub fn reverse_words(string_to_reverse: &str) -> String {
    let mut output = String::new();
    let mut temp_word_buffer = String::new();

    for c in string_to_reverse.chars() {
        if c.is_whitespace() {
            while let Some(t) = temp_word_buffer.pop() {
                output.push(t);
            }
            output.push(c);
        } else {
            temp_word_buffer.push(c);
        }
    }

    let last_word = temp_word_buffer.chars().rev().collect::<String>();

    output.push_str(&last_word);
    output
}

#[cfg(test)]
mod reverse_words_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(
            reverse_words("The quick brown fox jumps over the lazy dog."),
            "ehT kciuq nworb xof spmuj revo eht yzal .god"
        );
        assert_eq!(reverse_words("apple"), "elppa");
        assert_eq!(reverse_words("a b c d"), "a b c d");
        assert_eq!(
            reverse_words("double  spaced  words"),
            "elbuod  decaps  sdrow"
        );
    }
}
