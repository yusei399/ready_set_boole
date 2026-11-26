pub fn eval_set(formula: &str, mut sets: Vec<Vec<i32>>) -> Vec<i32> {
    for set in &mut sets {
        set.sort_unstable();
        set.dedup();
    }
    // All complements, including complements of intermediate results, use
    // this same union of the input sets.
    let mut universe: Vec<i32> = sets.iter().flatten().copied().collect();
    universe.sort_unstable();
    universe.dedup();

    let mut stack: Vec<Vec<i32>> = Vec::new();
    for symbol in formula.chars() {
        match symbol {
            'A'..='Z' => {
                let index = (symbol as u8 - b'A') as usize;
                stack.push(
                    sets.get(index)
                        .expect("Missing input set for variable")
                        .clone(),
                );
            }
            '!' => {
                let set = stack.pop().expect("Not enough operands for '!'");
                let complement = universe
                    .iter()
                    .copied()
                    .filter(|value| !set.contains(value))
                    .collect();
                stack.push(complement);
            }
            '&' | '|' | '^' | '>' | '=' => {
                let right = stack
                    .pop()
                    .expect("Not enough operands for binary operator");
                let left = stack
                    .pop()
                    .expect("Not enough operands for binary operator");
                // Apply each Boolean operator to membership in the two sets.
                // Iterating over U also includes elements in neither operand,
                // which are needed for implication and equivalence.
                let result = universe
                    .iter()
                    .copied()
                    .filter(|value| {
                        let in_left = left.contains(value);
                        let in_right = right.contains(value);
                        match symbol {
                            '&' => in_left && in_right,
                            '|' => in_left || in_right,
                            '^' => in_left ^ in_right,
                            '>' => !in_left || in_right,
                            '=' => in_left == in_right,
                            _ => unreachable!(),
                        }
                    })
                    .collect();
                stack.push(result);
            }
            _ => panic!("Invalid symbol in formula: {symbol}"),
        }
    }
    assert_eq!(stack.len(), 1, "Formula is invalid");
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_examples() {
        assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]), vec![0]);
        assert_eq!(
            eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
            vec![0, 1, 2, 3, 4, 5]
        );
        assert_eq!(eval_set("A!", vec![vec![0, 1, 2]]), Vec::<i32>::new());
    }

    #[test]
    fn evaluates_xor_implication_and_equivalence() {
        let sets = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(eval_set("AB^", sets.clone()), vec![0, 2]);
        assert_eq!(eval_set("AB=", sets.clone()), vec![1]);
        assert_eq!(eval_set("A!B>", sets.clone()), vec![0, 1, 2]);
        assert_eq!(eval_set("AB!>", sets), vec![0, 2]);
    }

    #[test]
    fn every_intermediate_complement_uses_the_global_universe() {
        let sets = vec![vec![1], vec![2], vec![3]];
        assert_eq!(eval_set("AB>CC!|&", sets.clone()), vec![2, 3]);
        assert_eq!(eval_set("AB=CC!|&", sets), vec![3]);
        assert_eq!(
            eval_set("AB&C|!", vec![vec![1, 2], vec![2, 3], vec![4]]),
            vec![1, 3]
        );
    }

    #[test]
    fn empty_sets_and_signed_integer_extremes() {
        assert!(eval_set("A", vec![vec![]]).is_empty());
        assert!(eval_set("A!", vec![vec![]]).is_empty());
        assert_eq!(eval_set("AB^", vec![vec![], vec![-1]]), vec![-1]);
        assert_eq!(
            eval_set("A!!", vec![vec![i32::MAX, -1, i32::MIN, 0]]),
            vec![i32::MIN, -1, 0, i32::MAX]
        );
    }

    #[test]
    fn all_26_sets_are_addressable() {
        let sets: Vec<Vec<i32>> = (0..26).map(|value| vec![value]).collect();
        let union = format!("ABCDEFGHIJKLMNOPQRSTUVWXYZ{}", "|".repeat(25));
        assert_eq!(
            eval_set(&union, sets.clone()),
            (0..26).collect::<Vec<i32>>()
        );
        assert!(eval_set(&format!("{union}!"), sets).is_empty());
    }

    #[test]
    fn binary_operators_match_independent_membership_truth_tables() {
        let domain = [-2, 0, 5];
        for left_bits in 0..8 {
            for right_bits in 0..8 {
                let left: Vec<i32> = domain
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| left_bits & (1 << index) != 0)
                    .map(|(_, value)| *value)
                    .collect();
                let right: Vec<i32> = domain
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| right_bits & (1 << index) != 0)
                    .map(|(_, value)| *value)
                    .collect();
                // Truth-table bits correspond to (left, right) = 00,01,10,11.
                for (operator, truth_table) in [
                    ('&', 0b1000),
                    ('|', 0b1110),
                    ('^', 0b0110),
                    ('>', 0b1011),
                    ('=', 0b1001),
                ] {
                    let expected: Vec<i32> = domain
                        .iter()
                        .copied()
                        .filter(|value| {
                            let row = 2 * usize::from(left.contains(value))
                                + usize::from(right.contains(value));
                            truth_table & (1 << row) != 0
                        })
                        .collect();
                    // C supplies the full universe, including elements absent
                    // from both A and B; C OR NOT C is the universal set.
                    let formula = format!("AB{operator}CC!|&");
                    let actual =
                        eval_set(&formula, vec![left.clone(), right.clone(), domain.to_vec()]);
                    assert_eq!(
                        actual, expected,
                        "formula: {formula}; A={left:?}; B={right:?}"
                    );
                }
            }
        }
    }
}
