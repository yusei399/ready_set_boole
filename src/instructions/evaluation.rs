pub fn eval_formula(formula: &str) -> bool {
    let mut stack = Vec::new();
    
    for token in formula.chars() {
        match token {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '!' => {
                if let Some(operand) = stack.pop() {
                    stack.push(!operand);
                } else {
                    // オペランドが不足している場合は false を返す
                    return false;
                }
            }
            '|' => {
                if stack.len() < 2 {
                    return false;
                }
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                stack.push(operand1 || operand2);
            }
            '&' => {
                if stack.len() < 2 {
                    return false;
                }
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                stack.push(operand1 && operand2);
            }
            '>' => {
                if stack.len() < 2 {
                    return false;
                }
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                // 含意: A > B を ¬A ∨ B として評価
                stack.push(!operand1 || operand2);
            }
            '=' => {
                if stack.len() < 2 {
                    return false;
                }
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                stack.push(operand1 == operand2);
            }
            '^' => {
                if stack.len() < 2 {
                    return false;
                }
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                stack.push(operand1 != operand2);
            }
            _ => {
                // 無効なトークンが現れた場合も false を返す
                return false;
            }
        }
    }
    
    // 最終的にスタックに1つの要素しか残っていなければ、その値を返す
    // そうでなければ不正な式とみなして false を返す
    if stack.len() == 1 {
        stack.pop().unwrap()
    } else {
        false
    }
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
        assert_eq!(eval_formula("11&"), true);  // 1 AND 1
        assert_eq!(eval_formula("00&"), false); // 0 AND 0
    }

    #[test]
    fn test_or() {
        // 論理和 (OR)
        assert_eq!(eval_formula("10|"), true);  // 1 OR 0
        assert_eq!(eval_formula("00|"), false); // 0 OR 0
        assert_eq!(eval_formula("11|"), true);  // 1 OR 1
    }

    #[test]
    fn test_xor() {
        // 排他的論理和 (XOR)
        assert_eq!(eval_formula("10^"), true);  // 1 XOR 0
        assert_eq!(eval_formula("11^"), false); // 1 XOR 1
        assert_eq!(eval_formula("00^"), false); // 0 XOR 0
    }

    #[test]
    fn test_implication() {
        // 含意 (>)
        // A > B は「¬A ∨ B」として評価
        assert_eq!(eval_formula("10>"), false); // 1 implies 0 → false
        assert_eq!(eval_formula("11>"), true);  // 1 implies 1 → true
        assert_eq!(eval_formula("00>"), true);  // 0 implies 0 → true
        assert_eq!(eval_formula("01>"), true);  // 0 implies 1 → true
    }

    #[test]
    fn test_equivalence() {
        // 同値 (=)
        assert_eq!(eval_formula("11="), true);  // 1 equals 1
        assert_eq!(eval_formula("10="), false); // 1 equals 0
        assert_eq!(eval_formula("00="), true);  // 0 equals 0
    }

    #[test]
    fn test_complex_expression() {
        // 複合的な式：例として "1011||=" のテスト
        // この式は、逆ポーランド記法で表現されると
        // 1 0 1 1 || = → (1 ∨ 0) = (1 ∨ 1) → (true) = (true) → true
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

// fn main() {
//     // サンプルとしていくつかの結果を表示
//     println!("eval_formula(\"10&\") = {}", eval_formula("10&")); // false
//     println!("eval_formula(\"10|\") = {}", eval_formula("10|")); // true
//     println!("eval_formula(\"1011||=\") = {}", eval_formula("1011||=")); // true
// }
