/// Adds modulo 2^32 using only bitwise operations and carry propagation.
/// Carry bits move left on every round, so u32 needs at most 32 rounds.
pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let carry = (a & b) << 1;
        a = a ^ b;
        b = carry;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder_two_positive_numbers() {
        assert_eq!(3, adder(2, 1));
    }

    #[test]
    fn adder_two_zero() {
        assert_eq!(0, adder(0, 0));
    }

    #[test]
    fn adder_with_one_zero() {
        assert_eq!(adder(4, 0), 4);
        assert_eq!(adder(0, 2), 2);
    }

    #[test]
    fn adder_with_1k() {
        assert_eq!(adder(4_000, 2_000), 6_000);
    }

    #[test]
    fn adder_with_10k() {
        assert_eq!(adder(4_0000, 2_0000), 6_0000);
    }

    #[test]
    fn adder_with_100k() {
        assert_eq!(adder(4_00000, 2_00000), 6_00000);
    }

    #[test]
    fn adder_propagates_carries_through_all_bits() {
        assert_eq!(adder(0x7fff_ffff, 1), 0x8000_0000);
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX - 1);
    }

    #[test]
    fn adder_matches_u32_arithmetic_at_boundaries() {
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
                assert_eq!(adder(a, b), a.wrapping_add(b), "{a} + {b}");
            }
        }
    }
}
