impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        fn backtrack(k: i32, n: i32, curr: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>, start: usize) {
            if curr.len() == k as usize && curr.iter().sum::<i32>() == n {
                answer.push(curr.clone());
            }
            for i in start..=9 {
                if curr.len() < k as usize {
                    curr.push(i as i32);
                    backtrack(k, n, curr, answer, i + 1);
                    curr.pop();
                }
            }
        }
        backtrack(k, n, &mut vec![], &mut answer, 1);
        answer
    }
}
