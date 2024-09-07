pub fn adder(a: u32, b: u32) -> u32{
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    adder(a ^ b, (a & b) << 1)
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
}