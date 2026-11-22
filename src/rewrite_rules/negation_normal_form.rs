#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Token {
    Literal(char, bool),
    And,
    Or,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Node {
    Literal(char, bool),
    Constant(bool),
    Binary(Token, usize, usize),
}

#[derive(Clone, Copy)]
struct Polarities {
    positive: usize,
    negative: usize,
}

pub(super) struct NormalForm {
    // Children are indices, so traversing and dropping deep expressions is stack-safe.
    nodes: Vec<Node>,
    root: usize,
    variable: Option<char>,
}

impl NormalForm {
    pub(super) fn parse(formula: &str) -> Option<Self> {
        let mut form = Self {
            nodes: vec![Node::Constant(false), Node::Constant(true)],
            root: 0,
            variable: None,
        };
        let mut operands = Vec::<Polarities>::new();
        for symbol in formula.chars() {
            let term = match symbol {
                'A'..='Z' => {
                    form.variable.get_or_insert(symbol);
                    Polarities {
                        positive: form.push(Node::Literal(symbol, false)),
                        negative: form.push(Node::Literal(symbol, true)),
                    }
                }
                '0' => Polarities {
                    positive: 0,
                    negative: 1,
                },
                '1' => Polarities {
                    positive: 1,
                    negative: 0,
                },
                '!' => {
                    let term = operands.pop()?;
                    Polarities {
                        positive: term.negative,
                        negative: term.positive,
                    }
                }
                '&' | '|' | '^' | '>' | '=' => {
                    let right = operands.pop()?;
                    let left = operands.pop()?;
                    form.binary(symbol, left, right)
                }
                _ => return None,
            };
            operands.push(term);
        }
        if operands.len() != 1 {
            return None;
        }
        form.root = operands.pop()?.positive;
        Some(form)
    }

    fn push(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    fn combine(&mut self, operator: Token, left: usize, right: usize) -> usize {
        if self.nodes[left] == self.nodes[right] {
            return left;
        }
        match (operator, self.nodes[left], self.nodes[right]) {
            (Token::And, Node::Constant(false), _) | (Token::And, _, Node::Constant(false)) => 0,
            (Token::Or, Node::Constant(true), _) | (Token::Or, _, Node::Constant(true)) => 1,
            (Token::And, Node::Constant(true), _) | (Token::Or, Node::Constant(false), _) => right,
            (Token::And, _, Node::Constant(true)) | (Token::Or, _, Node::Constant(false)) => left,
            _ => self.push(Node::Binary(operator, left, right)),
        }
    }

    fn binary(&mut self, operator: char, left: Polarities, right: Polarities) -> Polarities {
        use Token::{And, Or};

        let (l, nl) = (left.positive, left.negative);
        let (r, nr) = (right.positive, right.negative);
        match operator {
            '&' => Polarities {
                positive: self.combine(And, l, r),
                negative: self.combine(Or, nl, nr),
            },
            '|' => Polarities {
                positive: self.combine(Or, l, r),
                negative: self.combine(And, nl, nr),
            },
            '>' => Polarities {
                positive: self.combine(Or, nl, r),
                negative: self.combine(And, l, nr),
            },
            '^' | '=' if self.nodes[l] == self.nodes[r] => {
                let equal = operator == '=';
                Polarities {
                    positive: usize::from(equal),
                    negative: usize::from(!equal),
                }
            }
            '^' | '=' => {
                // Equivalent means both operands are true or both are false.
                // XOR swaps the right operand's polarities.
                let (r, nr) = if operator == '^' { (nr, r) } else { (r, nr) };
                let both_true = self.combine(And, l, r);
                let both_false = self.combine(And, nl, nr);
                let either_false = self.combine(Or, nl, nr);
                let either_true = self.combine(Or, l, r);
                Polarities {
                    positive: self.combine(Or, both_true, both_false),
                    negative: self.combine(And, either_false, either_true),
                }
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn tokens(&self) -> impl Iterator<Item = Token> + '_ {
        enum Pending {
            Visit(usize),
            Emit(Token),
        }
        use Pending::{Emit, Visit};

        let mut work = vec![Visit(self.root)];
        std::iter::from_fn(move || loop {
            match work.pop()? {
                Emit(token) => return Some(token),
                Visit(index) => match self.nodes[index] {
                    Node::Literal(symbol, negated) => return Some(Token::Literal(symbol, negated)),
                    Node::Binary(operator, left, right) => {
                        work.extend([Emit(operator), Visit(right), Visit(left)]);
                    }
                    Node::Constant(value) => match self.variable {
                        Some(symbol) => {
                            let operator = if value { Token::Or } else { Token::And };
                            work.extend([Emit(operator), Emit(Token::Literal(symbol, true))]);
                            return Some(Token::Literal(symbol, false));
                        }
                        None => return Some(Token::Literal(if value { '1' } else { '0' }, false)),
                    },
                },
            }
        })
    }
}

pub(super) fn push_literal(output: &mut String, symbol: char, negated: bool) {
    output.push(symbol);
    if negated {
        output.push('!');
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let Some(form) = NormalForm::parse(formula) else {
        return "Error: invalid formula".to_string();
    };
    let mut output = String::new();
    for token in form.tokens() {
        match token {
            Token::Literal(symbol, negated) => push_literal(&mut output, symbol, negated),
            Token::And => output.push('&'),
            Token::Or => output.push('|'),
        }
    }
    output
}

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

    #[test]
    fn test_negation_normal_form_xor_and_its_negation() {
        assert_eq!(negation_normal_form("AB^"), "AB!&A!B&|");
        assert_eq!(negation_normal_form("AB^!"), "A!B|AB!|&");
    }

    #[test]
    fn handles_deep_negations_and_preserves_the_output_alphabet() {
        assert_eq!(
            negation_normal_form(&format!("A{}", "!".repeat(200_000))),
            "A"
        );
        assert_eq!(negation_normal_form("ZZ^"), "ZZ!&");
        assert_eq!(negation_normal_form("ZZ="), "ZZ!|");
    }
}
