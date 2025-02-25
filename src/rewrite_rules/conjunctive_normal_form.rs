// src/instructions/conjunctive_normal_form.rs

#[derive(Debug, Clone)]
enum Expr {
    Var(String),
    Const(bool),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

/// RPN 文字列を AST にパースする（対応する記号: '0','1','A'..='Z','!','&','|'）
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
            _ => return None,
        }
    }
    if stack.len() == 1 {
        Some(stack.pop()?)
    } else {
        None
    }
}

/// NNF 変換：否定をリテラルの直前まで下ろす
fn to_nnf(expr: Expr) -> Expr {
    match expr {
        Expr::Not(inner) => match *inner {
            Expr::Not(e) => to_nnf(*e), // 二重否定
            Expr::And(a, b) => {
                // ¬(A ∧ B) → ¬A ∨ ¬B
                Expr::Or(Box::new(to_nnf(Expr::Not(a))), Box::new(to_nnf(Expr::Not(b))))
            }
            Expr::Or(a, b) => {
                // ¬(A ∨ B) → ¬A ∧ ¬B
                Expr::And(Box::new(to_nnf(Expr::Not(a))), Box::new(to_nnf(Expr::Not(b))))
            }
            other => Expr::Not(Box::new(to_nnf(other))),
        },
        Expr::And(a, b) => Expr::And(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),
        Expr::Or(a, b) => Expr::Or(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),
        other => other,
    }
}

/// OR 分配則を適用して CNF にする
fn distribute_or(expr: Expr) -> Expr {
    match expr {
        Expr::Or(a, b) => {
            let a = distribute_or(*a);
            let b = distribute_or(*b);
            match (a, b) {
                (Expr::And(a1, a2), b) => Expr::And(
                    Box::new(distribute_or(Expr::Or(a1, Box::new(b.clone())))),
                    Box::new(distribute_or(Expr::Or(a2, Box::new(b)))),
                ),
                (a, Expr::And(b1, b2)) => Expr::And(
                    Box::new(distribute_or(Expr::Or(Box::new(a.clone()), b1))),
                    Box::new(distribute_or(Expr::Or(Box::new(a), b2))),
                ),
                (a, b) => Expr::Or(Box::new(a), Box::new(b)),
            }
        }
        Expr::And(a, b) => Expr::And(Box::new(distribute_or(*a)), Box::new(distribute_or(*b))),
        Expr::Not(e) => Expr::Not(Box::new(distribute_or(*e))),
        other => other,
    }
}

/// AND チェーンをフラット化する
fn flatten_and(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::And(a, b) => {
            let mut left = flatten_and(a);
            let mut right = flatten_and(b);
            left.append(&mut right);
            left
        }
        _ => vec![expr.clone()],
    }
}

/// OR チェーンをフラット化する
fn flatten_or(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::Or(a, b) => {
            let mut left = flatten_or(a);
            let mut right = flatten_or(b);
            left.append(&mut right);
            left
        }
        _ => vec![expr.clone()],
    }
}

/// AST を RPN 表記の文字列に変換する。なお、AND と OR はフラット化して出力する。
fn to_rpn(expr: &Expr) -> String {
    match expr {
        Expr::Var(s) => s.clone(),
        Expr::Const(true) => "1".to_string(),
        Expr::Const(false) => "0".to_string(),
        Expr::Not(e) => format!("{}!", to_rpn(e)),
        Expr::And(_, _) => {
            let operands = flatten_and(expr);
            let mut rpn = String::new();
            for op in &operands {
                rpn.push_str(&to_rpn(op));
            }
            for _ in 0..(operands.len() - 1) {
                rpn.push('&');
            }
            rpn
        }
        Expr::Or(_, _) => {
            let operands = flatten_or(expr);
            let mut rpn = String::new();
            for op in &operands {
                rpn.push_str(&to_rpn(op));
            }
            for _ in 0..(operands.len() - 1) {
                rpn.push('|');
            }
            rpn
        }
    }
}

/// 入力の RPN 式（入力は NNF 状態であることを前提）を CNF に変換し、RPN 表記の文字列として返す。
pub fn conjunctive_normal_form(formula: &str) -> String {
    if let Some(ast) = parse_rpn(formula) {
        let nnf_ast = to_nnf(ast);
        let cnf_ast = distribute_or(nnf_ast);
        to_rpn(&cnf_ast)
    } else {
        "Error: invalid formula".to_string()
    }
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
