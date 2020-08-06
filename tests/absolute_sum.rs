// From: https://edabit.com/challenge/jBFpjEG3tvsjTZbD4
// Take an array of integers (positive or negative or both)
// and return the sum of the absolute value of each element.

pub fn absolute_sum(nums_to_sum: Vec<i32>) -> u32 {
    nums_to_sum
        .iter()
        .map(|n| {
            n.abs()
            // n*n.signum() // Or this way if usage of `.abs` disallowed
        })
        .sum::<i32>() as u32
}

#[cfg(test)]
mod absolute_sum_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(absolute_sum(vec![2, -1, 4, 8, 10]), 25);
        assert_eq!(absolute_sum(vec![-3, -4, -10, -2, -3]), 22);
        assert_eq!(absolute_sum(vec![2, 4, 6, 8, 10]), 30);
        assert_eq!(absolute_sum(vec![-1]), 1);
    }
}
