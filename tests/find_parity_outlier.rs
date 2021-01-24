// From: https://www.codewars.com/kata/5526fc09a1bbd946250002dc/train/rust
//
// You are given an array (which will have a length of at least 3, but could be very large)
// containing integers. The array is either entirely comprised of odd integers or entirely
// comprised of even integers except for a single integer N. Write a method that takes the
// array as an argument and returns this "outlier" N.

pub fn find_outlier(values: &[i32]) -> i32 {
    let mut output_index = 0;
    for i in 1..values.len() - 1 {
        let before_is_odd = (values[i - 1] % 2 | values[i - 1] % 1).abs();
        let current_is_odd = (values[i] % 2 | values[i] % 1).abs();
        let after_is_odd = (values[i + 1] % 2 | values[i + 1] % 1).abs();

        if before_is_odd == 1 && current_is_odd == 0 && after_is_odd == 0 {
            output_index = i - 1;
        } else if before_is_odd == 0 && current_is_odd == 1 && after_is_odd == 0 {
            output_index = i;
        } else if before_is_odd == 0 && current_is_odd == 0 && after_is_odd == 1 {
            output_index = i + 1;
        } else if before_is_odd == 0 && current_is_odd == 1 && after_is_odd == 1 {
            output_index = i - 1;
        } else if before_is_odd == 1 && current_is_odd == 0 && after_is_odd == 1 {
            output_index = i;
        } else if before_is_odd == 1 && current_is_odd == 1 && after_is_odd == 0 {
            output_index = i + 1;
        }
    }
    values[output_index]
}

#[cfg(test)]
mod remove_char_tests {
    use super::*;
    #[test]
    fn find_parity_outlier_tests() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [std::i32::MAX, 0, 1];
        let t4 = [0, 0, 0, 1];
        let t5 = [-1, -5, -23001, -301, 5, -2];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
        assert_eq!(1, find_outlier(&t4));
        assert_eq!(-2, find_outlier(&t5));
    }
}
