impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        fn backtrack(n: i32, k: i32, combination: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>, curr: i32) {
            if combination.len() == k as usize {
                answer.push(combination.clone());
            }
            for i in curr..=n {
                combination.push(i);
                backtrack(n, k, combination, answer, i + 1);
                combination.pop();
            }
        }
        backtrack(n, k, &mut vec![], &mut answer, 1);
        answer
    }
}
