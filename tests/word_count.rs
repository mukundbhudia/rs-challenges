// From: https://edabit.com/challenge/2RZXNLZMNMnGtS5d5
// Create a function that takes a string and returns the word count. The string will be a sentence.

pub fn count_words(nums_to_sum: &str) -> u32 {
    nums_to_sum.split(" ").count() as u32
}

#[cfg(test)]
mod word_count_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(count_words("Just an example here move along"), 6);
        assert_eq!(count_words("This is a test"), 4);
        assert_eq!(count_words("What an easy task, right"), 5);
    }
}
