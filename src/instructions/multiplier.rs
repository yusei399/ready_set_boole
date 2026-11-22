use crate::instructions::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut factor = a;
    let mut multiplier = b;
    let mut result = 0;

    // At most 32 iterations for a u32 multiplier.
    while multiplier != 0 {
        if multiplier & 1 != 0 {
            result = adder(result, factor);
        }
        multiplier >>= 1;
        factor <<= 1;
    }
    result
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
    fn multiplier_with_thousands() {
        assert_eq!(multiplier(4_000, 2_000), 8_000_000);
    }

    #[test]
    fn multiplier_uses_the_highest_bit() {
        assert_eq!(multiplier(1, 0x8000_0000), 0x8000_0000);
    }
}
