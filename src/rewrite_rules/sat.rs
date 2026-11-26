pub fn sat(formula: &str) -> bool {
    let symbols: Vec<char> = formula.chars().collect();
    let mut variable_bits = [0u32; 26];
    let mut variable_count = 0;

    // Repeated occurrences of a variable share one assignment bit. Compact
    // indices also ensure that a lone Z needs only two assignments, not 2^26.
    for &symbol in &symbols {
        if symbol.is_ascii_uppercase() {
            let index = (symbol as u8 - b'A') as usize;
            if variable_bits[index] == 0 {
                variable_bits[index] = 1u32 << variable_count;
                variable_count += 1;
            }
        }
    }

    let mut stack = Vec::with_capacity(symbols.len());
    for assignment in 0..(1u32 << variable_count) {
        stack.clear();
        for &symbol in &symbols {
            match symbol {
                '0' => stack.push(false),
                '1' => stack.push(true),
                'A'..='Z' => {
                    let index = (symbol as u8 - b'A') as usize;
                    stack.push(assignment & variable_bits[index] != 0);
                }
                '!' => {
                    let value = stack.pop().expect("Not enough operands for '!'");
                    stack.push(!value);
                }
                '&' | '|' | '^' | '>' | '=' => {
                    // The first popped operand is the right-hand side in RPN.
                    let right = stack
                        .pop()
                        .expect("Not enough operands for binary operator");
                    let left = stack
                        .pop()
                        .expect("Not enough operands for binary operator");
                    let value = match symbol {
                        '&' => left && right,
                        '|' => left || right,
                        '^' => left ^ right,
                        '>' => !left || right,
                        '=' => left == right,
                        _ => unreachable!(),
                    };
                    stack.push(value);
                }
                _ => panic!("Invalid symbol in formula: {symbol}"),
            }
        }

        assert_eq!(stack.len(), 1, "Formula is invalid");
        if stack[0] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_examples() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }

    #[test]
    fn finds_assignments_other_than_all_true() {
        assert!(sat("A!"));
        assert!(sat("AB^"));
        assert!(sat("AB!&"));
        assert!(sat("AB|!"));
        assert!(!sat("AA!&"));
    }

    #[test]
    fn implication_and_equivalence_are_evaluated() {
        assert!(!sat("A!A="));
        assert!(!sat("A!A!>!"));
        assert!(!sat("AB>AB!&&"));
        assert!(sat("BA>AB!&&"));
        assert!(sat("AB="));
        assert!(sat("AB=!"));
    }

    #[test]
    fn repeated_and_nonconsecutive_variables_share_assignments() {
        assert!(sat("Z!"));
        assert!(sat("AZ^"));
        assert!(!sat("ZZ^"));
        assert!(!sat("AZ^AZ=&"));
    }

    #[test]
    fn accepts_all_26_distinct_variables() {
        // A disjunction is satisfiable quickly, while exercising all 26 bits.
        let formula = format!("ABCDEFGHIJKLMNOPQRSTUVWXYZ{}", "|".repeat(25));
        assert!(sat(&formula));
    }

    #[test]
    fn preserves_literal_support_used_by_main() {
        assert!(sat("1"));
        assert!(!sat("0"));
        assert!(sat("0!"));
        assert!(!sat("1!"));
        assert!(sat("101|&"));
    }

    #[test]
    fn matches_independently_constructed_truth_masks() {
        // Each mask describes the four assignments 00, 01, 10, 11. Construct
        // formulas and their truth tables together; do not reuse the parser.
        let mut by_length = vec![
            Vec::<(String, u8)>::new(),
            vec![("A".into(), 0b1100), ("B".into(), 0b1010)],
        ];
        for length in 2..=7 {
            let mut next = Vec::new();
            for (formula, mask) in &by_length[length - 1] {
                next.push((format!("{formula}!"), mask ^ 0b1111));
            }
            for left_length in 1..length - 1 {
                let right_length = length - 1 - left_length;
                for (left, left_mask) in &by_length[left_length] {
                    for (right, right_mask) in &by_length[right_length] {
                        for (operator, mask) in [
                            ('&', left_mask & right_mask),
                            ('|', left_mask | right_mask),
                            ('^', left_mask ^ right_mask),
                            ('>', (left_mask ^ 0b1111) | right_mask),
                            ('=', (left_mask ^ right_mask) ^ 0b1111),
                        ] {
                            next.push((format!("{left}{right}{operator}"), mask));
                        }
                    }
                }
            }
            by_length.push(next);
        }
        for (formula, mask) in by_length.iter().flatten() {
            assert_eq!(sat(formula), *mask != 0, "formula: {formula}");
        }
    }
}
