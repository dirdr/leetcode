impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        println!("");
        let mut stack: Vec<i32> = vec![];
        for ch in tokens.iter() {
            match ch.as_str() {
                "+" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b + a);
                }
                "-" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                }
                "*" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b * a);
                }
                "/" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b / a);
                }
                value => {
                    stack.push(value.parse::<i32>().unwrap());
                }
            }
        }
        stack.pop().unwrap()
    }
}
