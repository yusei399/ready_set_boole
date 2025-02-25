mod instructions;
mod rewrite_rules;

use crate::instructions::adder::adder;
use crate::instructions::multiplier::multiplier;
use crate::instructions::gray_code::gray_code;
use crate::instructions::evaluation::eval_formula;
use crate::instructions::truth_table::print_truth_table;
use crate::rewrite_rules::negation_normal_form::negation_normal_form;
use crate::rewrite_rules::conjunctive_normal_form::conjunctive_normal_form;

fn main() {
    println!("Adder");
    println!("{}", adder(1, 2));
    println!("{}", adder(4_000, 2_000));
    println!("{}", adder(4_0000, 2_0000));
    println!("{}", adder(4_00000, 2_00000));
    println!("----------------------------------");


    println!("Multiplier");
    println!("{}", multiplier(2, 3));
    println!("{}", multiplier(0, 3));
    println!("{}", multiplier(2, 0));
    println!("{}", multiplier(2, 1));
    println!("{}", multiplier(3, 1));
    println!("{}", multiplier(4_000, 2_000));
    println!("{}", multiplier(4_0000, 2_0000));
    println!("{}", multiplier(4_00000, 2_00000));
    println!("----------------------------------");

    println!("Gray Code");
    println!("{}", gray_code(0));
    println!("{}", gray_code(1));
    println!("{}", gray_code(2));
    println!("{}", gray_code(3));
    println!("{}", gray_code(4));
    println!("{}", gray_code(5));

    println!("Evaluation");
    println!("Test literal values:");
    println!("eval_formula(\"1\") = {} (expected: true)", eval_formula("1"));
    println!("eval_formula(\"0\") = {} (expected: false)", eval_formula("0"));

    // negation
    println!("Test negation:");
    println!("eval_formula(\"1!\") = {} (expected: false)", eval_formula("1!"));
    println!("eval_formula(\"0!\") = {} (expected: true)", eval_formula("0!"));

    // and
    println!("Test AND:");
    println!("eval_formula(\"10&\") = {} (expected: false)", eval_formula("10&"));
    println!("eval_formula(\"11&\") = {} (expected: true)", eval_formula("11&"));
    println!("eval_formula(\"00&\") = {} (expected: false)", eval_formula("00&"));

    // or -- 正しい RPN 形式なら "01|" となる
    println!("Test OR:");
    println!("eval_formula(\"01|\") = {} (expected: true)", eval_formula("01|"));
    println!("eval_formula(\"00|\") = {} (expected: false)", eval_formula("00|"));
    println!("eval_formula(\"11|\") = {} (expected: true)", eval_formula("11|"));

    // xor
    println!("Test XOR:");
    println!("eval_formula(\"10^\") = {} (expected: true)", eval_formula("10^"));
    println!("eval_formula(\"11^\") = {} (expected: false)", eval_formula("11^"));
    println!("eval_formula(\"00^\") = {} (expected: false)", eval_formula("00^"));

    // implication
    println!("Test implication:");
    println!("eval_formula(\"10>\") = {} (expected: false)", eval_formula("10>"));
    println!("eval_formula(\"11>\") = {} (expected: true)", eval_formula("11>"));
    println!("eval_formula(\"00>\") = {} (expected: true)", eval_formula("00>"));
    println!("eval_formula(\"01>\") = {} (expected: true)", eval_formula("01>"));

    // equivalence
    println!("Test equivalence:");
    println!("eval_formula(\"11=\") = {} (expected: true)", eval_formula("11="));
    println!("eval_formula(\"10=\") = {} (expected: false)", eval_formula("10="));
    println!("eval_formula(\"00=\") = {} (expected: true)", eval_formula("00="));

    // complex expression -- RPN では "1011||=" となる
    println!("Test complex expression:");
    println!("eval_formula(\"1011||=\") = {} (expected: true)", eval_formula("1011||="));

    // nested negation
    println!("Test nested negation:");
    println!("eval_formula(\"1!!\") = {} (expected: true)", eval_formula("1!!"));
    println!("eval_formula(\"0!!\") = {} (expected: false)", eval_formula("0!!"));

    println!("Truth Table for expression \"A\":");
    print_truth_table("A");

    println!("Truth Table for expression \"AB|\" (A OR B):");
    print_truth_table("AB|");

    println!("Truth Table for expression \"AB&C|\" ((A AND B) OR C):");
    print_truth_table("AB&C|");

    println!("--- Negation Normal Form Test in main() ---");

    let formulas = [
        ("AB&!", "A!B!|"),
        ("AB|!", "A!B!&"),
        ("AB>", "A!B|"),
        ("AB=", "AB&A!B!&|"),
        ("AB|C&!", "A!B!&C!|"),
    ];

    for (input, expected) in formulas.iter() {
        let nnf = negation_normal_form(input);
        println!("Input: {:<8} -> NNF: {:<12} (expected: {})", input, nnf, expected);
    }

        println!("--- Conjunctive Normal Form Conversion ---");
        let formulas = [
            ("AB&!", "A!B!|"),
            ("AB|!", "A!B!&"),
            ("AB|C&", "AB|C&"),
            ("AB|C|D|", "ABCD|||"),
            ("AB&C&D&", "ABCD&&&"),
            ("AB&!C!|", "A!B!C!||"),
            ("AB|!C!&", "A!B!C!&&"),
        ];
        for (input, expected) in formulas.iter() {
            let cnf = conjunctive_normal_form(input);
            println!("Input: {:<10} -> CNF: {:<12} (expected: {})", input, cnf, expected);
        }
}
