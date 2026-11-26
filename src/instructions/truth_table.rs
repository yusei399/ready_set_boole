use super::evaluation::eval_formula;
use std::io::{self, Write};

fn write_truth_table<W: Write>(formula: &str, output: &mut W) -> io::Result<()> {
    let mut variables: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    variables.sort_unstable();
    variables.dedup();

    write!(output, "|")?;
    for variable in &variables {
        write!(output, " {} |", variable)?;
    }
    writeln!(output, " = |")?;

    write!(output, "|")?;
    for _ in 0..=variables.len() {
        write!(output, "---|")?;
    }
    writeln!(output)?;

    // At most 26 distinct variables, regardless of their occurrence count.
    let combinations = 1usize << variables.len();
    for row in 0..combinations {
        let mut assignment = [false; 26];
        for (column, &variable) in variables.iter().enumerate() {
            // The rightmost column changes fastest, as in the subject's example.
            assignment[(variable as u8 - b'A') as usize] =
                ((row >> (variables.len() - column - 1)) & 1) != 0;
        }

        // Repeated occurrences of a variable share exactly one assignment.
        let evaluated_formula: String = formula
            .chars()
            .map(|token| {
                if token.is_ascii_uppercase() {
                    if assignment[(token as u8 - b'A') as usize] {
                        '1'
                    } else {
                        '0'
                    }
                } else {
                    token
                }
            })
            .collect();

        write!(output, "|")?;
        for &variable in &variables {
            write!(
                output,
                " {} |",
                assignment[(variable as u8 - b'A') as usize] as u8
            )?;
        }
        writeln!(output, " {} |", eval_formula(&evaluated_formula) as u8)?;
    }
    Ok(())
}

// Keep the collector for callers that need a String; stdout streams directly.
#[allow(dead_code)]
pub fn get_truth_table(formula: &str) -> String {
    let mut output = Vec::new();
    write_truth_table(formula, &mut output).expect("writing to a Vec cannot fail");
    String::from_utf8(output).expect("truth tables contain only ASCII characters")
}

pub fn print_truth_table(formula: &str) {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    if let Err(error) = write_truth_table(formula, &mut output).and_then(|()| output.flush()) {
        eprintln!("Could not write truth table: {}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_subject_example_exactly() {
        assert_eq!(
            get_truth_table("AB&C|"),
            concat!(
                "| A | B | C | = |\n",
                "|---|---|---|---|\n",
                "| 0 | 0 | 0 | 0 |\n",
                "| 0 | 0 | 1 | 1 |\n",
                "| 0 | 1 | 0 | 0 |\n",
                "| 0 | 1 | 1 | 1 |\n",
                "| 1 | 0 | 0 | 0 |\n",
                "| 1 | 0 | 1 | 1 |\n",
                "| 1 | 1 | 0 | 1 |\n",
                "| 1 | 1 | 1 | 1 |\n",
            )
        );
    }

    #[test]
    fn repeated_variables_share_one_column_and_assignment() {
        let identity = "| A | = |\n|---|---|\n| 0 | 0 |\n| 1 | 1 |\n";
        assert_eq!(get_truth_table("AA&"), identity);
        assert_eq!(get_truth_table("AA|"), identity);
        assert_eq!(
            get_truth_table("AA^"),
            "| A | = |\n|---|---|\n| 0 | 0 |\n| 1 | 0 |\n"
        );
        assert_eq!(
            get_truth_table("ABA&|"),
            concat!(
                "| A | B | = |\n|---|---|---|\n",
                "| 0 | 0 | 0 |\n| 0 | 1 | 0 |\n",
                "| 1 | 0 | 1 |\n| 1 | 1 | 1 |\n",
            )
        );
    }

    #[test]
    fn columns_are_alphabetical_even_with_sparse_or_reversed_variables() {
        assert_eq!(
            get_truth_table("ZA>"),
            concat!(
                "| A | Z | = |\n|---|---|---|\n",
                "| 0 | 0 | 1 |\n| 0 | 1 | 0 |\n",
                "| 1 | 0 | 1 |\n| 1 | 1 | 1 |\n",
            )
        );
    }

    #[test]
    fn many_occurrences_do_not_overflow_or_create_extra_rows() {
        let expected = get_truth_table("A");
        for occurrences in [31, 32, 64, 128] {
            let formula = "A".repeat(occurrences) + &"&".repeat(occurrences - 1);
            assert_eq!(get_truth_table(&formula), expected);
        }
    }

    #[test]
    fn streams_large_tables_and_propagates_write_errors() {
        struct LimitedOutput {
            lines: usize,
        }
        impl Write for LimitedOutput {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                if self.lines >= 3 {
                    return Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed output"));
                }
                self.lines += bytes.iter().filter(|&&byte| byte == b'\n').count();
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let formula = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string() + &"&".repeat(25);
        let mut output = LimitedOutput { lines: 0 };
        let error = write_truth_table(&formula, &mut output).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
        assert_eq!(output.lines, 3);
    }
}
