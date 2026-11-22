use std::collections::BTreeMap;

use crate::instructions::evaluation::{evaluate, variable_index};

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    // An element is in the universe if it belongs to at least one input set.
    let mut universe = BTreeMap::<i32, u32>::new();
    for (index, set) in sets.into_iter().enumerate() {
        for element in set {
            *universe.entry(element).or_default() |= 1 << index;
        }
    }

    universe
        .into_iter()
        .filter_map(|(element, membership)| {
            let value = |variable| Some(membership & (1 << variable_index(variable)) != 0);
            (evaluate(formula, value) == Some(true)).then_some(element)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    #[test]
    fn test_eval_set_intersection() {
        // A = [0,1,2], B = [0,3,4] なら A ∩ B = [0]
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        let mut result = eval_set("AB&", sets);
        result = sorted(result);
        let expected = sorted(vec![0]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_eval_set_union() {
        // A = [0,1,2], B = [3,4,5] なら A ∪ B = [0,1,2,3,4,5]
        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let mut result = eval_set("AB|", sets);
        result = sorted(result);
        let expected = sorted(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_eval_set_complement() {
        // "A!" は [0,1,2] の補集合  [0,1,2] → [] となる
        let sets = vec![vec![0, 1, 2]];
        let result = eval_set("A!", sets);
        assert_eq!(sorted(result), sorted(vec![]));
    }

    #[test]
    fn test_eval_set_xor_implication_and_equivalence() {
        let sets = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(eval_set("AB^", sets.clone()), vec![1, 3]);
        assert_eq!(eval_set("AB>", sets.clone()), vec![2, 3]);
        assert_eq!(eval_set("BA>", sets.clone()), vec![1, 2]);
        assert_eq!(eval_set("AB=", sets), vec![2]);
    }

    #[test]
    fn test_eval_set_nested_operations_use_the_whole_universe() {
        let sets = vec![vec![1, 2], vec![2, 3], vec![-1, 3]];
        // -1 belongs to neither A nor B, so A = B holds there.
        assert_eq!(eval_set("AB=C&", sets.clone()), vec![-1]);
        assert_eq!(eval_set("AB|!C&", sets), vec![-1]);
    }
}
