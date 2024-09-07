fn adder(a: u32, b: u32) -> u32{
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    adder(a ^ b, (a & b) << 1)
}

fn main() {
    println!("{}", adder(1, 2));
}