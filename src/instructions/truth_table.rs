use super::evaluation::eval_formula;

pub fn get_truth_table(formula: &str) -> String
{
    let variables: Vec<char> = formula.chars().filter(|&c| c.is_alphabetic()).collect();
    let num_variables = variables.len();

    let combination = 1 << num_variables;
    let mut output = String::new();

    for i in 0..combination {
        let mut assignment = vec![];
        for j in 0..num_variables{
            assignment.push(((i >> j) & 1) == 1);
        }
        let mut local_formula = formula.to_string();
        for (idx, &val) in assignment.iter().enumerate(){
            local_formula = local_formula.replace(variables[idx], if val{"1"} else {"0"})
        }
        let result = eval_formula(&local_formula);
        let line = format!(
            "| {} | = | {} |\n",
            assignment
                .iter()
                .map(|&v| if v { '1' } else { '0' })
                .collect::<String>(),
            result as u8
        );
        output.push_str(&line);
    }
    output
}

pub fn print_truth_table(formula: &str) {
    let output = get_truth_table(formula);
    println!("{}", output);
}