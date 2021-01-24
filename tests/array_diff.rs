// From: https://www.codewars.com/kata/523f5d21c841566fde000009/train/rust

// Your goal in this kata is to implement a difference function, which subtracts one list from another and returns the result.
// It should remove all values from list a, which are present in list b.
// array_diff(vec![1,2], vec![1]) == vec![2]
// If a value is present in b, all of its occurrences must be removed from the other:
// array_diff(vec![1,2,2,2,3], vec![2]) == vec![1,3]

pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut a = a;
    for item_in_b in b {
        a.retain(|item_in_a| item_in_a != &item_in_b);
    }
    a
}

// Add your tests here
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    }
}
