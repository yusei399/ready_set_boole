#[derive(Debug, Clone)]
enum Expr {
    Var(String),
    Const(bool),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Imp(Box<Expr>, Box<Expr>),
    Equ(Box<Expr>, Box<Expr>),
}

fn parse_rpn(formula: &str) -> Option<Expr> {
    let mut stack = Vec::new();
    for ch in formula.chars() {
        match ch {
            '0' => stack.push(Expr::Const(false)),
            '1' => stack.push(Expr::Const(true)),
            'A'..='Z' => stack.push(Expr::Var(ch.to_string())),
            '!' => {
                let expr = stack.pop()?;
                stack.push(Expr::Not(Box::new(expr)));
            }
            '&' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(Expr::And(Box::new(left), Box::new(right)));
            }
            '|' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(Expr::Or(Box::new(left), Box::new(right)));
            }
            '>' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(Expr::Imp(Box::new(left), Box::new(right)));
            }
            '=' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(Expr::Equ(Box::new(left), Box::new(right)));
            }
            _ => return None,
        }
    }
    if stack.len() == 1 {
        Some(stack.pop()?)
    } else {
        None
    }
}

/// 含意、同値を展開する
fn eliminate_implications(expr: Expr) -> Expr {
    match expr {
        Expr::Imp(a, b) => {
            // A > B  => ¬A ∨ B
            Expr::Or(
                Box::new(Expr::Not(Box::new(eliminate_implications(*a)))),
                Box::new(eliminate_implications(*b)),
            )
        }
        Expr::Equ(a, b) => {
            // A = B  => (A ∧ B) ∨ (¬A ∧ ¬B)
            let a_e = eliminate_implications(*a);
            let b_e = eliminate_implications(*b);
            Expr::Or(
                Box::new(Expr::And(Box::new(a_e.clone()), Box::new(b_e.clone()))),
                Box::new(Expr::And(
                    Box::new(Expr::Not(Box::new(a_e))),
                    Box::new(Expr::Not(Box::new(b_e))),
                )),
            )
        }
        Expr::Not(inner) => Expr::Not(Box::new(eliminate_implications(*inner))),
        Expr::And(a, b) => Expr::And(
            Box::new(eliminate_implications(*a)),
            Box::new(eliminate_implications(*b)),
        ),
        Expr::Or(a, b) => Expr::Or(
            Box::new(eliminate_implications(*a)),
            Box::new(eliminate_implications(*b)),
        ),
        other => other,
    }
}

/// NNF 変換
fn to_nnf(expr: Expr) -> Expr {
    match expr {
        Expr::Not(inner) => match *inner {
            Expr::Not(e) => to_nnf(*e), // 二重否定の除去
            Expr::And(a, b) => {
                // ¬(A ∧ B) → ¬A ∨ ¬B
                Expr::Or(
                    Box::new(to_nnf(Expr::Not(a))),
                    Box::new(to_nnf(Expr::Not(b))),
                )
            }
            Expr::Or(a, b) => {
                // ¬(A ∨ B) → ¬A ∧ ¬B
                Expr::And(
                    Box::new(to_nnf(Expr::Not(a))),
                    Box::new(to_nnf(Expr::Not(b))),
                )
            }
            other => Expr::Not(Box::new(to_nnf(other))),
        },
        Expr::And(a, b) => Expr::And(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),
        Expr::Or(a, b) => Expr::Or(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),
        other => other,
    }
}

/// AST を RPN 表記に変換する
fn to_rpn(expr: &Expr) -> String {
    match expr {
        Expr::Var(s) => s.clone(),
        Expr::Const(true) => "1".to_string(),
        Expr::Const(false) => "0".to_string(),
        Expr::Not(e) => format!("{}!", to_rpn(e)),
        Expr::And(a, b) => format!("{}{}&", to_rpn(a), to_rpn(b)),
        Expr::Or(a, b) => format!("{}{}|", to_rpn(a), to_rpn(b)),
        _ => "".to_string(), // Imp, Equ should have been eliminated
    }
}

/// 指定された RPN 式を否定正規形 (NNF) に変換し、RPN 表記で返す。
pub fn negation_normal_form(formula: &str) -> String {
    if let Some(parsed) = parse_rpn(formula) {
        let no_imp = eliminate_implications(parsed);
        let nnf = to_nnf(no_imp);
        to_rpn(&nnf)
    } else {
        "Error: invalid formula".to_string()
    }
}

// Cargo テスト用
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation_normal_form_ab_and_not() {
        // AB&! なら ¬(A ∧ B) → 期待値例として "A!B!|"
        let input = "AB&!";
        let expected = "A!B!|";
        assert_eq!(negation_normal_form(input), expected);
    }

    #[test]
    fn test_negation_normal_form_ab_or_not() {
        // AB|! なら ¬(A ∨ B) → 期待値例として "A!B!&"
        let input = "AB|!";
        let expected = "A!B!&";
        assert_eq!(negation_normal_form(input), expected);
    }

    #[test]
    fn test_negation_normal_form_ab_implication() {
        // AB> は A ⇒ B, 期待値例として "A!B|"
        let input = "AB>";
        let expected = "A!B|";
        assert_eq!(negation_normal_form(input), expected);
    }

    #[test]
    fn test_negation_normal_form_ab_equivalence() {
        // AB= は A ⇔ B, 期待値例として "AB&A!B!&|"
        let input = "AB=";
        let expected = "AB&A!B!&|";
        assert_eq!(negation_normal_form(input), expected);
    }

    #[test]
    fn test_negation_normal_form_complex() {
        // AB|C&! なら ¬((A ∨ B) ∧ C) → 期待値例として "A!B!&C!|"
        let input = "AB|C&!";
        let expected = "A!B!&C!|";
        assert_eq!(negation_normal_form(input), expected);
    }
}