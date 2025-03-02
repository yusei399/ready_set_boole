// sat.rs

pub fn sat(formula: &str) -> bool {
    let mut stack = Vec::new();
    
    for ch in formula.chars() {
        match ch {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '!' => {
                let val = stack.pop().expect("Not enough operands for '!'");
                stack.push(!val);
            }
            '&' => {
                let b = stack.pop().expect("Not enough operands for '&'");
                let a = stack.pop().expect("Not enough operands for '&'");
                stack.push(a && b);
            }
            '|' => {
                let b = stack.pop().expect("Not enough operands for '|'");
                let a = stack.pop().expect("Not enough operands for '|'");
                stack.push(a || b);
            }
            _ => (),
        }
    }
    
    stack.pop().expect("Formula is invalid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sat_literal_true() {
        // 単一のリテラル "1" は true を返す
        assert_eq!(sat("1"), true);
    }

    #[test]
    fn test_sat_literal_false() {
        // 単一のリテラル "0" は false を返す
        assert_eq!(sat("0"), false);
    }

    #[test]
    fn test_sat_negation() {
        // "1!" は 1 を否定して false、"0!" は 0 を否定して true
        assert_eq!(sat("1!"), false);
        assert_eq!(sat("0!"), true);
    }

    #[test]
    fn test_sat_and() {
        // "11&" は true AND true → true
        // "10&" は true AND false → false
        assert_eq!(sat("11&"), true);
        assert_eq!(sat("10&"), false);
    }

    #[test]
    fn test_sat_or() {
        // "10|" は true OR false → true
        // "00|" は false OR false → false
        assert_eq!(sat("10|"), true);
        assert_eq!(sat("00|"), false);
    }

    #[test]
    fn test_sat_complex_expression() {
        // 例: "101|&" → まず 0 OR 1 → true, その後 1 AND true → true
        assert_eq!(sat("101|&"), true);
    }
}

// fn main() {
//     println!("SAT Evaluation Tests (manual):");
//     println!("sat(\"1\") = {}", sat("1"));         // true
//     println!("sat(\"0\") = {}", sat("0"));         // false
//     println!("sat(\"1!\") = {}", sat("1!"));       // false
//     println!("sat(\"0!\") = {}", sat("0!"));       // true
//     println!("sat(\"11&\") = {}", sat("11&"));     // true
//     println!("sat(\"10&\") = {}", sat("10&"));     // false
//     println!("sat(\"10|\") = {}", sat("10|"));     // true
//     println!("sat(\"00|\") = {}", sat("00|"));     // false
//     println!("sat(\"101|&\") = {}", sat("101|&")); // true
// }
