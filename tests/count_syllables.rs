// From: https://edabit.com/challenge/zRe5fx3MbiBxcrvXH
//
// Create a function that counts the number of syllables
// a word has. Each syllable is separated with a dash `-`.

pub fn number_syllables(word: &str) -> u32 {
    word.split("-").count() as u32
}

#[cfg(test)]
mod count_syllables_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(number_syllables("buf-fet"), 2);
        assert_eq!(number_syllables("beau-ti-ful"), 3);
        assert_eq!(number_syllables("mon-u-men-tal"), 4);
        assert_eq!(number_syllables("on-o-mat-o-poe-ia"), 6);
    }
}
