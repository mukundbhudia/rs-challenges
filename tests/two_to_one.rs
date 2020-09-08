// From: https://www.codewars.com/kata/5656b6906de340bd1b0000ac/train/rust

// Take 2 strings s1 and s2 including only letters from ato z.
// Return a new sorted string, the longest possible, containing distinct letters,


pub fn longest(a1: &str, a2: &str) -> String {
    let mut output = String::new();
    let mut chars = [a1, a2]
        .concat()
        .chars()
        .collect::<Vec<_>>();
    chars.sort();
    for c in chars {
        if !output.contains(c) {
            output.push(c);
        }
    }
    output
}

#[cfg(test)]
    mod two_to_one_tests {
    use super::*;
   
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
        
    }
}
