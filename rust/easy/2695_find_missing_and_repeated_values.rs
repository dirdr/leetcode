use std::collections::HashSet;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut set: HashSet<i32> = (1..=grid.len() * grid.len()).map(|e| e as i32).collect();
        let mut ans = Vec::with_capacity(2);
        for vec in &grid {
            for &el in vec {
                if !set.remove(&el) {
                    ans.push(el);
                }
            }
        }
        ans.push(set.into_iter().nth(0).unwrap());
        ans
    }
}
