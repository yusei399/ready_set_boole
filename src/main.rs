mod instructions;

use crate::instructions::adder::adder;

fn main() {
    println!("{}", adder(1, 2));
    println!("{}", adder(4_000, 2_000));
    println!("{}", adder(4_0000, 2_0000));
    println!("{}", adder(4_00000, 2_00000));
}
