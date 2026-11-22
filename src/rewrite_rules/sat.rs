use crate::instructions::evaluation::{assignments, evaluate, variable_index, variables};

pub fn sat(formula: &str) -> bool {
    assignments(&variables(formula)).any(|values| {
        evaluate(formula, |variable| Some(values[variable_index(variable)])) == Some(true)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sat_literals() {
        // 単一リテラル
        assert_eq!(sat("1"), true);
        assert_eq!(sat("0"), false);
    }

    #[test]
    fn test_sat_variables() {
        assert_eq!(sat("A"), true);
        // A OR B → true OR true = true
        assert_eq!(sat("AB|"), true);
        // A AND B → true AND true = true
        assert_eq!(sat("AB&"), true);
    }

    #[test]
    fn test_sat_xor() {
        // "AA^" → true XOR true = false
        assert_eq!(sat("AA^"), false);
        assert!(sat("AB^"));
    }

    #[test]
    fn test_sat_complex() {
        assert_eq!(sat("AA!&"), false);
    }
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
    fn test_sat_searches_assignments_with_false_values() {
        assert!(sat("Z!"));
        assert!(sat("AB>!"));
        assert!(sat("AB=!"));
        assert!(!sat("AA>!"));
        assert!(!sat("ZZ=!"));
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
        // 例: "101|&" → 0 OR 1 → true, その後 1 AND true → true
        assert_eq!(sat("101|&"), true);
    }
}
