// 例: src/instructions/set_evaluation.rs などに配置

/// RPN 形式のブール式と、各変数に対応する集合のリストから、
/// 式を評価して結果の集合を返す関数。
/// グローバルな集合は 0..=100 として補集合を求める。
pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let mut stack = Vec::new();
    // 式中に現れる変数（A～Z）を順に取得（ただし、ここでは直接変数の文字を利用）
    let _variables: Vec<char> = formula.chars().filter(|&c| c.is_alphabetic()).collect();
    
    for ch in formula.chars() {
        match ch {
            'A'..='Z' => {
                // 'A' に対応する集合は sets[0]、'B' は sets[1] など
                let idx = (ch as usize) - ('A' as usize);
                stack.push(sets[idx].clone());
            }
            '!' => {
                let set = stack.pop().unwrap();
                // グローバルな集合 [0,100] から set に含まれない要素を集める
                let complement: Vec<i32> = (0..=100).filter(|x| !set.contains(x)).collect();
                stack.push(complement);
            }
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                // a ∩ b
                let intersection: Vec<i32> = a.into_iter().filter(|x| b.contains(x)).collect();
                stack.push(intersection);
            }
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                // a ∪ b
                // 重複があっても順序は問わないので、そのまま連結
                let mut union: Vec<i32> = a.into_iter().chain(b.into_iter()).collect();
                // 重複削除のためソートして dedup する
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

    // 補助関数：Vec<i32> をソートしたコピーを返す
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
    
//     // 他の例も追加可能
//     println!("--- End of eval_set tests ---");
// }
