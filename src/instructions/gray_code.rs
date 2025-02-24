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
}
