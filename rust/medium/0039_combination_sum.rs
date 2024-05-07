use std::collections::HashSet;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidates: &Vec<i32>,
            target: i32,
            answer: &mut Vec<Vec<i32>>,
            current: &mut Vec<i32>,
            start: usize,
        ) {
            if target == 0 {
                answer.push(current.clone());
                return;
            }
            if target < 0 {
                return;
            }
            for i in start..candidates.len() {
                current.push(candidates[i]);
                backtrack(candidates, target - candidates[i], answer, current, i);
                current.pop();
            }
        }
        let mut answer = vec![];
        backtrack(&candidates, target, &mut answer, &mut vec![], 0);
        answer
    }
}
