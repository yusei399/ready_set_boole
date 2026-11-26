use crate::instructions::adder::adder;

/// Multiplies modulo 2^32. The required u32 type bounds both this loop and
/// each iterative adder call to 32 rounds, with constant auxiliary space.
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut factor = a;
    let mut multiplier = b;
    let mut result = 0;

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
    fn multiplier_with_1k() {
        assert_eq!(multiplier(4_000, 2_000), 8_000_000);
    }

    #[test]
    fn multiplier_with_1k_and_1k() {
        assert_eq!(multiplier(1_000, 1_000), 1_000_000);
    }

    #[test]
    fn multiplier_matches_u32_arithmetic_at_boundaries() {
        let values = [
            0,
            1,
            2,
            31,
            32,
            65_535,
            65_536,
            0x7fff_ffff,
            0x8000_0000,
            u32::MAX,
        ];
        for a in values {
            for b in values {
                assert_eq!(multiplier(a, b), a.wrapping_mul(b), "{a} * {b}");
            }
        }
    }
}
