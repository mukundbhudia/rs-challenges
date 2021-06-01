// From: https://www.codewars.com/kata/513e08acc600c94f01000001/train/rust

// Very simple, given a number, find its opposite.

pub fn rgb(r: i32, g: i32, b: i32) -> String {
    let r = r.clamp(0, 255);
    let g = g.clamp(0, 255);
    let b = b.clamp(0, 255);

    format!("{:02X}{:02X}{:02X}", r, g, b)
}

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected {
            panic!("Got: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

#[cfg(test)]
mod rgb_to_hex_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
