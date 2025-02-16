func eval_formula(formula: &str) -> bool{
    let mut stack: Vec<bool> = Vec::new();

    for ch in formula.chars(){
        match ch{
            '0' => stack.push(false)
            '1' => stack.push(false)
            '!' => {
                if let Some(a) = stack.pop(){
                    stack.push(!a)
                } else{
                    panic!("Invalid formula")
                }
            }
            '&' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()){
                    stack.push(a && b)
                }else{
                    panic!("Invalid formula")
                }
            }
            '|' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()){
                    stack.push(a || b)
                }else{
                    panic!("Invalid formula")
                }
            },
            '^' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()){
                    stack.push(a ^ b)
                }else{
                    panic!("Invalid formula")
                }
            }
        }
    }
}