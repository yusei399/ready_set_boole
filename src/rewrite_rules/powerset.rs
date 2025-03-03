pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];
    
    for num in set {
        let mut new_subsets = Vec::new();
        for subset in result.iter() {
            let mut new_subset = subset.clone();
            new_subset.push(num);
            new_subsets.push(new_subset);
        }
        result.extend(new_subsets);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerset_empty() {
        let input: Vec<i32> = vec![];
        let output = powerset(input);
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_powerset_single() {
        let input = vec![1];
        let output = powerset(input);
        let expected = vec![vec![], vec![1]];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_powerset_two_elements() {
        let input = vec![1, 2];
        let output = powerset(input);
        let expected = vec![vec![], vec![1], vec![2], vec![1, 2]];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_powerset_three_elements() {
        let input = vec![1, 2, 3];
        let output = powerset(input);
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(output, expected);
    }
}

// fn main() {
//     println!("--- Powerset Tests in main() ---");

//     let test_sets = vec![
//         vec![],
//         vec![1],
//         vec![1, 2],
//         vec![1, 2, 3],
//     ];

//     for set in test_sets {
//         println!("Input set: {:?}", set);
//         let pset = powerset(set.clone());
//         println!("Powerset: {:?}", pset);
//         println!("-----------------------");
//     }

// }
