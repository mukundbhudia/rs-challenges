// From: https://edabit.com/challenge/Gs2p3whk4D7noHRTe

pub fn quadratic_solutions(a: i32, b: i32, c: i32) -> u32 {
    let discriminant = b*b - 4*a*c;
    if discriminant > 0 {
        2
    } else if discriminant < 0 {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod quadratic_solutions_tests {
    use super::*;
    #[test]
    fn general_tests() {
        // x² - 1 = 0 has two solutions (x = 1 and x = -1).
        assert_eq!(quadratic_solutions(1, 0, -1), 2);
        // x² = 0 has one solution (x = 0).
        assert_eq!(quadratic_solutions(1, 0, 0), 1);
        // x² + 1 = 0 has no solutions.
        assert_eq!(quadratic_solutions(1, 0, 1), 0);
    }
}
