impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut answer = vec![];
        fn dfs(curr: i32, n: i32, answer: &mut Vec<i32>) {
            if curr > n {
                return;
            }
            answer.push(curr);
            for i in 0..10 {
                dfs(curr * 10 + i, n, answer);
            }
        }
        for i in 1..10 {
            dfs(i, n, &mut answer);
        }
        answer
    }
}
