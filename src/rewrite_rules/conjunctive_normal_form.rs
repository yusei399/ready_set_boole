use std::collections::{HashSet, VecDeque};

use super::negation_normal_form::{push_literal, NormalForm, Token};

type Clause = Vec<(char, bool)>;
type Clauses = VecDeque<Clause>;

fn disjoin(left: Clauses, right: Clauses) -> Clauses {
    let mut result = VecDeque::new();
    let mut seen = HashSet::new();
    for a in &left {
        for b in &right {
            let mut merged = a.clone();
            for &(symbol, negated) in b {
                if !merged.contains(&(symbol, negated)) {
                    merged.push((symbol, negated));
                }
            }

            // A clause containing both polarities is always true and can be dropped.
            let mut key = merged.clone();
            key.sort_unstable();
            let tautology = key.windows(2).any(|pair| pair[0].0 == pair[1].0);
            if !tautology && seen.insert(key) {
                result.push_back(merged);
            }
        }
    }
    result
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let Some(form) = NormalForm::parse(formula) else {
        return "Error: invalid formula".to_string();
    };
    let mut operands = Vec::<Clauses>::new();
    for token in form.tokens() {
        let clauses = match token {
            Token::Literal(symbol, negated) => Clauses::from([vec![(symbol, negated)]]),
            Token::And => {
                let mut right = operands.pop().unwrap();
                let mut left = operands.pop().unwrap();
                // Move the smaller side and keep the original clause order.
                if left.len() >= right.len() {
                    left.append(&mut right);
                    left
                } else {
                    for clause in left.into_iter().rev() {
                        right.push_front(clause);
                    }
                    right
                }
            }
            Token::Or => {
                let right = operands.pop().unwrap();
                let left = operands.pop().unwrap();
                disjoin(left, right)
            }
        };
        operands.push(clauses);
    }

    let clauses = operands.pop().unwrap();
    if clauses.is_empty() {
        let symbol = formula
            .chars()
            .find(char::is_ascii_uppercase)
            .unwrap_or('A');
        return format!("{symbol}{symbol}!|");
    }

    let mut output = String::new();
    for clause in &clauses {
        for &(symbol, negated) in clause {
            push_literal(&mut output, symbol, negated);
        }
        for _ in 1..clause.len() {
            output.push('|');
        }
    }
    for _ in 1..clauses.len() {
        output.push('&');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_conjunctive_normal_form_distributes_disjunction() {
        assert_eq!(conjunctive_normal_form("ABCD&|&"), "ABC|BD|&&");
    }

    #[test]
    fn test_conjunctive_normal_form_handles_all_operators() {
        assert_eq!(conjunctive_normal_form("AB>"), "A!B|");
        assert_eq!(conjunctive_normal_form("AB^!"), "A!B|AB!|&");
        assert_eq!(conjunctive_normal_form("AB=!"), "A!B!|AB|&");
    }

    #[test]
    fn handles_deep_valid_formulas() {
        assert_eq!(
            conjunctive_normal_form(&format!("A{}", "!".repeat(200_000))),
            "A"
        );
        let nested = format!("{}C{}", "AB".repeat(20_000), "&".repeat(40_000));
        assert_eq!(conjunctive_normal_form(&nested), nested);
    }

    #[test]
    fn simplifies_repeated_xor_before_distribution() {
        assert_eq!(conjunctive_normal_form("AA^A^A^A^"), "A");
        assert_eq!(
            conjunctive_normal_form(&format!("A{}", "A^".repeat(200))),
            "A"
        );
    }

    #[test]
    fn drops_tautological_and_duplicate_clauses() {
        assert_eq!(conjunctive_normal_form("AB&A!|"), "BA!|");
        assert_eq!(conjunctive_normal_form("AB&BA&|"), "AB|AB&&");
    }
}
