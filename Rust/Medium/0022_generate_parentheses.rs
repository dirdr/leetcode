impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        pub fn dfs(
            n: usize,
            combination: &mut String,
            result: &mut Vec<String>,
            left: usize,
            right: usize,
        ) -> () {
            if (combination.len() == 2*n) {
                result.push(combination.clone());
                return;
            }
            if (left < n) {
                combination.push('(');
                dfs(n, combination, result, left + 1, right);
                combination.pop();
            }
            if (right < left) {
                combination.push(')');
                dfs(n, combination, result, left, right + 1);
                combination.pop();
            }
        }
        let mut result: Vec<String> = vec![];
        dfs(n as usize, &mut String::from(""), &mut result, 0, 0);
        result
    }
}
