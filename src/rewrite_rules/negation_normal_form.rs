#[derive(Clone, Copy)]
enum Node {
    Variable(char),
    Constant(bool),
    Not(usize),
    Binary {
        operator: char,
        left: usize,
        right: usize,
    },
}

// Child indices keep parsing and dropping deep formulas off the call stack.
fn parse_rpn(formula: &str) -> Option<(Vec<Node>, usize)> {
    let mut nodes = Vec::new();
    let mut stack = Vec::new();
    for ch in formula.chars() {
        let node = match ch {
            'A'..='Z' => Node::Variable(ch),
            '0' | '1' => Node::Constant(ch == '1'),
            '!' => Node::Not(stack.pop()?),
            '&' | '|' | '^' | '>' | '=' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                Node::Binary {
                    operator: ch,
                    left,
                    right,
                }
            }
            _ => return None,
        };
        nodes.push(node);
        stack.push(nodes.len() - 1);
    }
    if stack.len() == 1 {
        Some((nodes, stack.pop()?))
    } else {
        None
    }
}

enum Task {
    Visit(usize, bool),
    Emit(char),
}

// Carry the requested polarity instead of cloning and recursively rewriting subtrees.
fn write_nnf(nodes: &[Node], root: usize) -> String {
    let mut result = String::new();
    let mut tasks = vec![Task::Visit(root, false)];
    while let Some(task) = tasks.pop() {
        match task {
            Task::Emit(operator) => result.push(operator),
            Task::Visit(index, negated) => match nodes[index] {
                Node::Variable(variable) => {
                    result.push(variable);
                    if negated {
                        result.push('!');
                    }
                }
                Node::Constant(value) => result.push(if value != negated { '1' } else { '0' }),
                Node::Not(child) => tasks.push(Task::Visit(child, !negated)),
                Node::Binary {
                    operator,
                    left,
                    right,
                } => match operator {
                    '&' | '|' => {
                        let output_operator = if negated {
                            if operator == '&' {
                                '|'
                            } else {
                                '&'
                            }
                        } else {
                            operator
                        };
                        // The task stack is LIFO: visit left, then right, then emit.
                        tasks.push(Task::Emit(output_operator));
                        tasks.push(Task::Visit(right, negated));
                        tasks.push(Task::Visit(left, negated));
                    }
                    '>' => {
                        // A > B = !A | B; its negation is A & !B.
                        tasks.push(Task::Emit(if negated { '&' } else { '|' }));
                        tasks.push(Task::Visit(right, negated));
                        tasks.push(Task::Visit(left, !negated));
                    }
                    '^' | '=' => {
                        // XOR: (A | B) & (!A | !B).
                        // Equivalence: (A & B) | (!A & !B).
                        // Under negation, De Morgan swaps both operators and polarities.
                        let inner = if (operator == '=') != negated {
                            '&'
                        } else {
                            '|'
                        };
                        let outer = if inner == '&' { '|' } else { '&' };
                        tasks.push(Task::Emit(outer));
                        tasks.push(Task::Emit(inner));
                        tasks.push(Task::Visit(right, !negated));
                        tasks.push(Task::Visit(left, !negated));
                        tasks.push(Task::Emit(inner));
                        tasks.push(Task::Visit(right, negated));
                        tasks.push(Task::Visit(left, negated));
                    }
                    _ => unreachable!(),
                },
            },
        }
    }
    result
}

/// RPN 式を NNF に変換する。深い式も明示的なスタックで処理する。
pub fn negation_normal_form(formula: &str) -> String {
    match parse_rpn(formula) {
        Some((nodes, root)) => write_nnf(&nodes, root),
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
        assert_eq!(negation_normal_form(&negations), "A");
        assert_eq!(negation_normal_form(&(negations + "!")), "A!");

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
            for formula in [left_chain, right_chain] {
                assert_eq!(negation_normal_form(&formula), formula);
                let negated = negation_normal_form(&(formula + "!"));
                assert!(evaluate(&negated, &['A'], 0));
                assert!(!evaluate(&negated, &['A'], 1));
            }
        }
    }

    // Evaluate the original and rewritten formulas independently of the AST passes.
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
    fn test_all_operators_preserve_truth_values_and_nnf_shape() {
        let formulas = [
            "A", "Z", "A!!", "A!!!", "AB^", "AB^!", "AB>!", "AB=!", "A!B!>!", "AB^C>", "AB>C^!",
            "AB=CD^>!", "AB^CD=^!", "AB>CD>&!", "AB^CD^=",
        ];
        for formula in formulas {
            let result = negation_normal_form(formula);
            let chars: Vec<char> = result.chars().collect();
            for (index, &ch) in chars.iter().enumerate() {
                assert!(ch.is_ascii_uppercase() || matches!(ch, '!' | '&' | '|'));
                if ch == '!' {
                    assert!(index > 0 && chars[index - 1].is_ascii_uppercase());
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
