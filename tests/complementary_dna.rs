// From: https://www.codewars.com/kata/554e4a2f232cdd87d9000038/train/rust
//
// In DNA strings, symbols "A" and "T" are complements of each other, as
// "C" and "G". You have function with one side of the DNA (string,
// except for Haskell); you need to get the other complementary side.
// DNA strand is never empty or there is no DNA at all (again, except for Haskell).

pub fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|x| match x {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod shortest_word_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(dna_strand("AAAA"), "TTTT");
        assert_eq!(dna_strand("ATTGC"), "TAACG");
        assert_eq!(dna_strand("GTAT"), "CATA");
    }
}
