pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code_zero() {
        assert_eq!(0, gray_code(0));
    }

    #[test]
    fn gray_code_one() {
        assert_eq!(1, gray_code(1));
    }

    #[test]
    fn gray_code_two() {
        assert_eq!(3, gray_code(2));
    }

    #[test]
    fn gray_code_three() {
        assert_eq!(2, gray_code(3));
    }

    #[test]
    fn gray_code_four() {
        assert_eq!(6, gray_code(4));
    }

    #[test]
    fn gray_code_five() {
        assert_eq!(7, gray_code(5));
    }

    #[test]
    fn gray_code_matches_remaining_subject_examples_and_high_bit() {
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
        assert_eq!(gray_code(u32::MAX), 0x8000_0000);
    }

    #[test]
    fn successive_codes_change_one_bit() {
        for n in 1..=65_536 {
            assert_eq!((gray_code(n) ^ gray_code(n - 1)).count_ones(), 1);
        }
        assert_eq!(
            (gray_code(0x7fff_ffff) ^ gray_code(0x8000_0000)).count_ones(),
            1
        );
        assert_eq!(
            (gray_code(u32::MAX - 1) ^ gray_code(u32::MAX)).count_ones(),
            1
        );
    }
}
