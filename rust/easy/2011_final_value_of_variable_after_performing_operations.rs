impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for op in &operations {
            if op.contains("--") {
                x -= 1;
            } else {
                x += 1;
            }
        }
        x
    }
}
