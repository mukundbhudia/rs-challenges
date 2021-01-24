// From: https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust

pub fn accum(s: &str) -> String {
    let mut output = String::new();
    let mut i = 0;

    for c in s.chars() {
        if i == 0 {
            let d = c.to_uppercase().to_string();
            output.push_str(&d);
        } else {
            for j in 0..i + 1 {
                if j == 0 {
                    let a = c.to_uppercase().to_string();
                    output.push_str(&a);
                } else {
                    let b = c.to_lowercase().to_string();
                    output.push_str(&b);
                }
            }
        }
        if i < s.len() - 1 {
            output.push_str("-");
        }
        i += 1;
    }
    output
}

#[cfg(test)]
mod unique_in_order_tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            accum("ZpglnRxqenU"),
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("NyffsGeyylB"),
            "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
        );
        assert_eq!(
            accum("MjtkuBovqrU"),
            "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("EvidjUnokmM"),
            "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
        );
        assert_eq!(
            accum("HbideVbxncC"),
            "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
        );
    }
}
