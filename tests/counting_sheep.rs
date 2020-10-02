// From: https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust

// Consider an array/list of sheep where some sheep may be missing from their place.
// We need a function that counts the number of sheep present in the array (true means present).

pub fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&x| *x).count() as u8
}

#[cfg(test)]
mod get_middle_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(count_sheep(&[false]), 0);
        assert_eq!(count_sheep(&[true]), 1);
        assert_eq!(count_sheep(&[true, false]), 1);
    }
}
