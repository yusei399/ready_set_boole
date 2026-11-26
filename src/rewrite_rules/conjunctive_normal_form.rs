use super::negation_normal_form::negation_normal_form;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Literal {
    symbol: char,
    negated: bool,
}

type Clause = VecDeque<Literal>;
type Clauses = VecDeque<Clause>;

// Move the smaller side while preserving order. Deep left/right chains stay efficient.
fn concatenate<T>(mut left: VecDeque<T>, mut right: VecDeque<T>) -> VecDeque<T> {
    if left.len() >= right.len() {
        left.append(&mut right);
        left
    } else {
        while let Some(item) = left.pop_back() {
            right.push_front(item);
        }
        right
    }
}

// A CNF is a conjunction of clauses; each clause is a disjunction of literals.
fn nnf_to_clauses(formula: &str) -> Option<Clauses> {
    let mut stack: Vec<Clauses> = Vec::new();
    for ch in formula.chars() {
        match ch {
            'A'..='Z' | '0' | '1' => {
                let literal = Literal {
                    symbol: ch,
                    negated: false,
                };
                stack.push(VecDeque::from([VecDeque::from([literal])]));
            }
            '!' => {
                let clauses = stack.last_mut()?;
                if clauses.len() != 1 || clauses[0].len() != 1 {
                    return None;
                }
                clauses[0][0].negated = !clauses[0][0].negated;
            }
            '&' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(concatenate(left, right));
            }
            '|' => {
                let mut right = stack.pop()?;
                let mut left = stack.pop()?;
                if left.len() == 1 && right.len() == 1 {
                    // Plain OR chains need no distribution or repeated copying.
                    let clause = concatenate(left.pop_front()?, right.pop_front()?);
                    stack.push(VecDeque::from([clause]));
                } else {
                    // (AND_i L_i) OR (AND_j R_j) = AND_i,j (L_i OR R_j).
                    let mut product = VecDeque::new();
                    for left_clause in &left {
                        for right_clause in &right {
                            let mut clause = left_clause.clone();
                            clause.extend(right_clause.iter().copied());
                            product.push_back(clause);
                        }
                    }
                    stack.push(product);
                }
            }
            _ => return None,
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

fn write_cnf(clauses: &Clauses) -> String {
    let mut result = String::new();
    for clause in clauses {
        for literal in clause {
            result.push(literal.symbol);
            if literal.negated {
                result.push('!');
            }
        }
        for _ in 1..clause.len() {
            result.push('|');
        }
    }
    // The subject requires every conjunction at the end of the RPN output.
    for _ in 1..clauses.len() {
        result.push('&');
    }
    result
}

/// 全演算子を含む RPN 式を NNF、節のリストの順に変換し、CNF を返す。
pub fn conjunctive_normal_form(formula: &str) -> String {
    let nnf = negation_normal_form(formula);
    match nnf_to_clauses(&nnf) {
        Some(clauses) => write_cnf(&clauses),
        None => "Error: invalid formula".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deep_negations_and_binary_chains_do_not_overflow_the_stack() {
        let depth = 10_000;
        let negations = format!("A{}", "!".repeat(depth));
        assert_eq!(conjunctive_normal_form(&negations), "A");
        assert_eq!(conjunctive_normal_form(&(negations + "!")), "A!");

        for operator in ['&', '|'] {
            let mut left_chain = String::from("A");
            for _ in 1..depth {
                left_chain.push('A');
                left_chain.push(operator);
            }
            let right_chain = format!(
                "{}{}",
                "A".repeat(depth),
                operator.to_string().repeat(depth - 1),
            );
            for formula in [left_chain, right_chain.clone()] {
                assert_eq!(conjunctive_normal_form(&formula), right_chain);
                let negated = conjunctive_normal_form(&(formula + "!"));
                assert!(evaluate(&negated, &['A'], 0));
                assert!(!evaluate(&negated, &['A'], 1));
            }
        }
    }

    // A separate Boolean evaluator checks meaning without relying on the rewrite passes.
    fn evaluate(formula: &str, variables: &[char], values: usize) -> bool {
        let mut stack = Vec::new();
        for ch in formula.chars() {
            match ch {
                'A'..='Z' => {
                    let index = variables
                        .iter()
                        .position(|&variable| variable == ch)
                        .unwrap();
                    stack.push(values & (1 << index) != 0);
                }
                '!' => {
                    let value = stack.pop().unwrap();
                    stack.push(!value);
                }
                '&' | '|' | '^' | '>' | '=' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(match ch {
                        '&' => left && right,
                        '|' => left || right,
                        '^' => left != right,
                        '>' => !left || right,
                        '=' => left == right,
                        _ => unreachable!(),
                    });
                }
                _ => panic!("invalid output symbol: {}", ch),
            }
        }
        assert_eq!(stack.len(), 1, "invalid RPN: {}", formula);
        stack[0]
    }

    #[test]
    fn test_all_operators_distribution_and_cnf_shape() {
        let formulas = [
            "A", "Z", "A!!", "AB>", "AB=", "AB^", "AB>!", "AB=!", "AB^!", "AB&C|", "AB&CD&|",
            "ABCD&|&", "AB|CD|&", "AB>C^!", "AB=CD^>!", "AB^CD=^!",
        ];
        for formula in formulas {
            let result = conjunctive_normal_form(formula);
            let chars: Vec<char> = result.chars().collect();
            let mut conjunctions_started = false;
            for (index, &ch) in chars.iter().enumerate() {
                assert!(ch.is_ascii_uppercase() || matches!(ch, '!' | '&' | '|'));
                if ch == '!' {
                    assert!(index > 0 && chars[index - 1].is_ascii_uppercase());
                }
                if ch == '&' {
                    conjunctions_started = true;
                }
                if conjunctions_started {
                    assert_eq!(ch, '&', "all conjunctions must be at the end: {}", result);
                }
            }
            let mut variables: Vec<char> =
                formula.chars().filter(char::is_ascii_uppercase).collect();
            variables.sort_unstable();
            variables.dedup();
            for values in 0..(1 << variables.len()) {
                assert_eq!(
                    evaluate(formula, &variables, values),
                    evaluate(&result, &variables, values),
                    "{} -> {} with assignment {}",
                    formula,
                    result,
                    values,
                );
            }
        }
    }

    #[test]
    fn test_conjunctive_normal_form_ab_and_not() {
        // ¬(A ∧ B) → ¬A ∨ ¬B → RPN: "A!B!|"
        let input = "AB&!";
        let expected = "A!B!|";
        assert_eq!(conjunctive_normal_form(input), expected);
    }

    #[test]
    fn test_conjunctive_normal_form_ab_or_not() {
        // ¬(A ∨ B) → ¬A ∧ ¬B → RPN: "A!B!&"
        let input = "AB|!";
        let expected = "A!B!&";
        assert_eq!(conjunctive_normal_form(input), expected);
    }

    #[test]
    fn test_conjunctive_normal_form_ab_or_c_and() {
        // (A ∨ B) ∧ C は既に CNF → RPN: "AB|C&"
        let input = "AB|C&";
        let expected = "AB|C&";
        assert_eq!(conjunctive_normal_form(input), expected);
    }

    #[test]
    fn test_conjunctive_normal_form_ab_or_c_or_d() {
        // A ∨ B ∨ C ∨ D → RPN: "ABCD|||"
        let input = "AB|C|D|";
        let expected = "ABCD|||";
        assert_eq!(conjunctive_normal_form(input), expected);
    }

    #[test]
    fn test_conjunctive_normal_form_ab_and_c_and_d() {
        // A ∧ B ∧ C ∧ D → RPN: "ABCD&&&"
        let input = "AB&C&D&";
        let expected = "ABCD&&&";
        assert_eq!(conjunctive_normal_form(input), expected);
    }

    #[test]
    fn test_conjunctive_normal_form_ab_and_not_c_not_or() {
        // ¬(A ∧ B) ∨ ¬C → (¬A ∨ ¬B) ∨ ¬C → RPN: "A!B!C!||"
        let input = "AB&!C!|";
        let expected = "A!B!C!||";
        assert_eq!(conjunctive_normal_form(input), expected);
    }

    #[test]
    fn test_conjunctive_normal_form_ab_or_not_c_not_and() {
        // ¬(A ∨ B) ∧ ¬C → (¬A ∧ ¬B) ∧ ¬C → RPN: "A!B!C!&&"
        let input = "AB|!C!&";
        let expected = "A!B!C!&&";
        assert_eq!(conjunctive_normal_form(input), expected);
    }
}

// main 関数での手動テスト用出力例
// fn main() {
//     println!("--- Conjunctive Normal Form Conversion ---");
//     let formulas = [
//         ("AB&!", "A!B!|"),
//         ("AB|!", "A!B!&"),
//         ("AB|C&", "AB|C&"),
//         ("AB|C|D|", "ABCD|||"),
//         ("AB&C&D&", "ABCD&&&"),
//         ("AB&!C!|", "A!B!C!||"),
//         ("AB|!C!&", "A!B!C!&&"),
//     ];
//     for (input, expected) in formulas.iter() {
//         let cnf = conjunctive_normal_form(input);
//         println!("Input: {:<10} -> CNF: {:<12} (expected: {})", input, cnf, expected);
//     }
// }
