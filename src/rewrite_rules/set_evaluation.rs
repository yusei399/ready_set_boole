
pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    // 全体集合を、渡されたすべての集合の和集合として求める
    let mut universe = Vec::new();
    for s in &sets {
        universe.extend(s.iter());
    }
    universe.sort();
    universe.dedup();

    let mut stack = Vec::new();
    
    for ch in formula.chars() {
        match ch {
            'A'..='Z' => {
                let idx = (ch as usize) - ('A' as usize);
                if idx < sets.len() {
                    stack.push(sets[idx].clone());
                } else {
                    stack.push(vec![]);
                }
            }
            '!' => {
                let set = stack.pop().unwrap();
                let complement: Vec<i32> = universe.iter()
                    .filter(|x| !set.contains(x))
                    .cloned()
                    .collect();
                stack.push(complement);
            }
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let intersection: Vec<i32> = a.into_iter().filter(|x| b.contains(x)).collect();
                stack.push(intersection);
            }
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let mut union: Vec<i32> = a.into_iter().chain(b.into_iter()).collect();
                union.sort();
                union.dedup();
                stack.push(union);
            }
            _ => (),
        }
    }
    
    stack.pop().unwrap()
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
        // A = [0,1,2] , B = [0,3,4]
        // "AB&" → A ∩ B = [0]
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        let mut result = eval_set("AB&", sets);
        result = sorted(result);
        let expected = sorted(vec![0]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_eval_set_union() {
        // A = [0,1,2], B = [3,4,5]
        // "AB|" → A ∪ B = [0,1,2,3,4,5]
        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let mut result = eval_set("AB|", sets);
        result = sorted(result);
        let expected = sorted(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_eval_set_complement() {
        // A = [0,1,2]
        // "A!" → 補集合 : [0..=100] から [0,1,2] を除く
        let sets = vec![vec![0, 1, 2]];
        let mut result = eval_set("A!", sets);
        result = sorted(result);
        // 期待値は 0..=100 から 0,1,2 を除いたもの
        let expected: Vec<i32> = (0..=100).filter(|x| ![0, 1, 2].contains(x)).collect();
        let expected = sorted(expected);
        assert_eq!(result, expected);
    }
}

// fn main() {
//     println!("--- eval_set Tests in main() ---");
    
//     // Test 1: Intersection
//     let sets1 = vec![vec![0, 1, 2], vec![0, 3, 4]];
//     let result1 = eval_set("AB&", sets1.clone());
//     println!("Formula: \"AB&\", Sets: {:?} => Result: {:?}", sets1, result1);
    
//     // Test 2: Union
//     let sets2 = vec![vec![0, 1, 2], vec![3, 4, 5]];
//     let result2 = eval_set("AB|", sets2.clone());
//     println!("Formula: \"AB|\", Sets: {:?} => Result: {:?}", sets2, result2);
    
//     // Test 3: Complement of A = [0,1,2]
//     let sets3 = vec![vec![0, 1, 2]];
//     let result3 = eval_set("A!", sets3.clone());
//     println!("Formula: \"A!\", Sets: {:?} => Result (length={}): {:?}", sets3, result3.len(), result3);
    
// }
