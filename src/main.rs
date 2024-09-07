mod instructions;

use crate::instructions::adder::adder;
use crate::instructions::multiplier::multiplier;

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


}
