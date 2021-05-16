// From: https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust

pub fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect::<_>()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
}
