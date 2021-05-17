// From: https://www.codewars.com/kata/54d81488b981293527000c8f/train/rust

// Given a list of integers and a single sum value, return the first two
// values (parse from the left please) in order of appearance that add up to form the sum.

pub fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // let output = None;
    let mut pairs_found: Vec<((i8, i8), (usize, usize))> = Vec::new();
    for (a, i) in ints.iter().enumerate() {
        for (b, j) in ints.iter().enumerate() {
            if i + j == s && a != b {
                println!("a: {}, b: {},", a, b);
                // return Some((*i, *j));
                pairs_found.push(((*i, *j), (a, b)));
            }
        }
    }
    println!("pairs: {:?}", pairs_found);
    pairs_found.sort_by_key(|k| k.1 .0); // TODO: fix this sorting
    pairs_found.reverse();
    println!("pairs: {:?}", pairs_found);
    if let Some(x) = pairs_found.pop() {
        Some(x.0)
    } else {
        None
    }
}

#[cfg(test)]
mod sum_pairs_tests {
    use super::*;
    #[test]
    fn general_tests() {
        let l1 = [1, 4, 8, 7, 3, 15];
        let l2 = [1, -2, 3, 0, -6, 1];
        let l3 = [20, -13, 40];
        let l4 = [1, 2, 3, 4, 1, 0];
        let l5 = [10, 5, 2, 3, 7, 5];
        let l6 = [4, -2, 3, 3, 4];
        let l7 = [0, 2, 0];
        let l8 = [5, 9, 13, -3];
        assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
        assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
        assert_eq!(sum_pairs(&l3, -7), None);
        assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
        assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
        assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
        assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
        assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    }
}
