use std::io::{self, Write};

use super::evaluation::{assignments, evaluate, variable_index, variables};

pub fn print_truth_table(formula: &str) {
    write_truth_table(formula, &mut io::stdout().lock()).expect("Could not write truth table");
}

fn write_truth_table(formula: &str, output: &mut impl Write) -> io::Result<()> {
    let variables = variables(formula);
    write!(output, "|")?;
    for &variable in &variables {
        write!(output, " {} |", char::from(variable))?;
    }
    writeln!(output, " = |")?;
    writeln!(output, "|{}", "---|".repeat(variables.len() + 1))?;

    for values in assignments(&variables) {
        let value = |variable| Some(values[variable_index(variable)]);
        let result = evaluate(formula, value).unwrap_or(false);
        write!(output, "|")?;
        for &variable in &variables {
            write!(output, " {} |", u8::from(values[variable_index(variable)]))?;
        }
        writeln!(output, " {} |", u8::from(result))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(formula: &str) -> String {
        let mut output = Vec::new();
        write_truth_table(formula, &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn matches_subject_example() {
        assert_eq!(
            table("AB&C|"),
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
            ),
        );
    }

    #[test]
    fn sorts_columns_and_evaluates_repeated_variables_consistently() {
        assert_eq!(
            table("ZA>Z&"),
            concat!(
                "| A | Z | = |\n",
                "|---|---|---|\n",
                "| 0 | 0 | 0 |\n",
                "| 0 | 1 | 0 |\n",
                "| 1 | 0 | 0 |\n",
                "| 1 | 1 | 1 |\n",
            ),
        );
    }
}
