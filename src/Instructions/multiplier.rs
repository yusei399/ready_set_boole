use crate::instructions::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    if b == 1 {
        return a;
    }
    adder(a, multiplier(a, b - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplier_two_positive_numbers() {
        assert_eq!(6, multiplier(2, 3));
    }

    #[test]
    fn multiplier_with_zero() {
        assert_eq!(0, multiplier(0, 3));
        assert_eq!(0, multiplier(2, 0));
    }

    #[test]
    fn multiplier_with_one() {
        assert_eq!(2, multiplier(2, 1));
        assert_eq!(3, multiplier(3, 1));
    }

    #[test]
    fn multiplier_with_1k() {
        assert_eq!(adder(4_000, 2_000), 6_000);
    }

    #[test]
    fn multiplier_with_1k_and_1k() {
        assert_eq!(adder(4_000, 2_000), 6_000);
    }
}
