use super::evaluation::eval_formula;

pub fn get_truth_table(formula: &str) -> String {
    let variables: Vec<char> = formula.chars().filter(|&c| c.is_alphabetic()).collect();
    let num_variables = variables.len();

    let mut header = String::from("|");
    for var in &variables {
        header.push_str(&format!(" {} |", var));
    }
    header.push_str(" = |");
    
    let mut separator = String::from("|");
    for _ in 0..(num_variables + 1) {
        separator.push_str("---|");
    }
    
    let combination = 1 << num_variables;
    let mut rows = String::new();
    
    for i in 0..combination {
        let mut assignment = vec![];
        for j in 0..num_variables {
            assignment.push(((i >> j) & 1) == 1);
        }
        let mut local_formula = formula.to_string();
        for (idx, &val) in assignment.iter().enumerate(){
            local_formula = local_formula.replace(variables[idx], if val { "1" } else { "0" });
        }
        let result = eval_formula(&local_formula);
        let mut line = String::from("|");
        for &val in &assignment {
            line.push_str(&format!(" {} |", if val { '1' } else { '0' }));
        }
        line.push_str(&format!(" {} |", result as u8));
        rows.push_str(&line);
        rows.push('\n');
    }
    
    format!("{}\n{}\n{}", header, separator, rows)
}

pub fn print_truth_table(formula: &str) {
    let output = get_truth_table(formula);
    println!("{}", output);
}
