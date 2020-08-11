// From: https://www.codewars.com/kata/514b92a657cdc65150000006/train/rust

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in.
// Note: If the number is a multiple of both 3 and 5, only count it once.

pub fn multiples_3_or_5(num: i32) -> i32 {
    (1..num)
        .fold(0, |n, i| {
           if i % 3 == 0 || i % 5 == 0 {
               n + i
           } else {
               n
           }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(multiples_3_or_5(10), 23);
        assert_eq!(multiples_3_or_5(11), 33);
        assert_eq!(multiples_3_or_5(6), 8);
    }
}
