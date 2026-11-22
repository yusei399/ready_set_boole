pub fn eval_formula(formula: &str) -> bool {
    evaluate(formula, |_| None).unwrap_or(false)
}

pub(crate) fn evaluate(formula: &str, variable_value: impl Fn(u8) -> Option<bool>) -> Option<bool> {
    let mut stack = Vec::<bool>::new();
    for token in formula.bytes() {
        let value = match token {
            b'0' => false,
            b'1' => true,
            b'A'..=b'Z' => variable_value(token)?,
            b'!' => !stack.pop()?,
            b'&' | b'|' | b'^' | b'>' | b'=' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                match token {
                    b'&' => left && right,
                    b'|' => left || right,
                    b'^' => left != right,
                    b'>' => !left || right,
                    b'=' => left == right,
                    _ => unreachable!(),
                }
            }
            _ => return None,
        };
        stack.push(value);
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

pub(crate) fn variable_index(variable: u8) -> usize {
    usize::from(variable - b'A')
}

pub(crate) fn variables(formula: &str) -> Vec<u8> {
    let mut present = [false; 26];
    for variable in formula.bytes().filter(u8::is_ascii_uppercase) {
        present[variable_index(variable)] = true;
    }
    (b'A'..=b'Z')
        .filter(|&variable| present[variable_index(variable)])
        .collect()
}

pub(crate) fn assignments(variables: &[u8]) -> impl Iterator<Item = [bool; 26]> + '_ {
    (0..(1u32 << variables.len())).map(|row| {
        let mut values = [false; 26];
        // The last column changes on every row.
        for (bit, &variable) in variables.iter().rev().enumerate() {
            values[variable_index(variable)] = row & (1 << bit) != 0;
        }
        values
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_values() {
        // 単一の値
        assert_eq!(eval_formula("1"), true);
        assert_eq!(eval_formula("0"), false);
    }

    #[test]
    fn test_negation() {
        // 単純な否定
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("0!"), true);
    }

    #[test]
    fn test_and() {
        // 論理積 (AND)
        assert_eq!(eval_formula("10&"), false); // 1 AND 0
        assert_eq!(eval_formula("11&"), true); // 1 AND 1
        assert_eq!(eval_formula("00&"), false); // 0 AND 0
    }

    #[test]
    fn test_or() {
        // 論理和 (OR)
        assert_eq!(eval_formula("10|"), true); // 1 OR 0
        assert_eq!(eval_formula("00|"), false); // 0 OR 0
        assert_eq!(eval_formula("11|"), true); // 1 OR 1
    }

    #[test]
    fn test_xor() {
        // 排他的論理和 (XOR)
        assert_eq!(eval_formula("10^"), true); // 1 XOR 0
        assert_eq!(eval_formula("11^"), false); // 1 XOR 1
        assert_eq!(eval_formula("00^"), false); // 0 XOR 0
    }

    #[test]
    fn test_implication() {
        // 含意 (>)
        // A > B は「¬A ∨ B」として評価
        assert_eq!(eval_formula("10>"), false); // 1 implies 0 → false
        assert_eq!(eval_formula("11>"), true); // 1 implies 1 → true
        assert_eq!(eval_formula("00>"), true); // 0 implies 0 → true
        assert_eq!(eval_formula("01>"), true); // 0 implies 1 → true
    }

    #[test]
    fn test_equivalence() {
        // 同値 (=)
        assert_eq!(eval_formula("11="), true); // 1 equals 1
        assert_eq!(eval_formula("10="), false); // 1 equals 0
        assert_eq!(eval_formula("00="), true); // 0 equals 0
    }

    #[test]
    fn test_complex_expression() {
        // 複合的な式：例として "1011||=" のテスト
        // 1 0 1 1 | | = → 1 = (0 ∨ (1 ∨ 1)) → true
        assert_eq!(eval_formula("1011||="), true);
    }

    #[test]
    fn test_nested_negation() {
        // 否定の入れ子
        // 式: "1!!" は、1 を2回否定して元に戻るはず
        assert_eq!(eval_formula("1!!"), true);
        // 式: "0!!" も同様
        assert_eq!(eval_formula("0!!"), false);
    }
}
