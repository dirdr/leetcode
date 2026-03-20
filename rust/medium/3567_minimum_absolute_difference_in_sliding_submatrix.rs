use std::collections::HashSet;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        fn sub_min_diff(set: &[i32]) -> i32 {
            let mut min = i32::MAX;
            for i in 0..set.len() - 1 {
                for j in i + 1..set.len() {
                    min = min.min((set[j] - set[i]).abs());
                }
            }
            if min == i32::MAX { 0 } else { min } 
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut answer = vec![vec![0; n - k + 1]; m - k + 1];
        for i in 0..answer.len() {
            for j in 0..answer[0].len() {
                let mut set = HashSet::new(); 
                for di in 0..k {
                    for dj in 0..k {
                        set.insert(grid[i + di][j + dj]);
                    } 
                }
                answer[i][j] = sub_min_diff(&set.iter().cloned().collect::<Vec<i32>>());
            }
        }
        answer
    }
}
