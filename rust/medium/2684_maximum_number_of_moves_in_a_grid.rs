use std::collections::HashMap;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        fn backtrack(
            grid: &Vec<Vec<i32>>,
            row: usize,
            col: usize,
            cnt: i32,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if col == grid[0].len() - 1 {
                return cnt;
            }
            if let Some(&cached) = memo.get(&(row, col)) {
                return cached;
            }
            let moves: [(isize, isize); 3] = [(-1, 1), (0, 1), (1, 1)];
            let mut max_child = cnt;
            for &(dr, dc) in &moves {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if new_row < 0
                    || new_row >= grid.len() as isize
                    || new_col >= grid[0].len() as isize
                {
                    continue;
                }
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                if grid[new_row][new_col] > grid[row][col] {
                    max_child = max_child.max(backtrack(grid, new_row, new_col, cnt + 1, memo));
                }
            }
            memo.insert((row, col), max_child);
            max_child
        }

        let mut max = 0;
        let mut memo = HashMap::new();
        for i in 0..grid.len() {
            max = max.max(backtrack(&grid, i, 0, 0, &mut memo));
        }
        max
    }
}

// impl Solution {
//     fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
//         let m = grid.len();
//         let n = grid[0].len();
//         let mut dp = vec![vec![0; n]; m];
//         let mut result = 0;
//         for j in (0..n - 1).rev() {
//             for i in 0..m {
//                 let mut max_moves = 0;
//                 if i > 0 && grid[i - 1][j + 1] > grid[i][j] {
//                     max_moves = max_moves.max(dp[i - 1][j + 1] + 1);
//                 }
//                 if grid[i][j + 1] > grid[i][j] {
//                     max_moves = max_moves.max(dp[i][j + 1] + 1);
//                 }
//                 if i < m - 1 && grid[i + 1][j + 1] > grid[i][j] {
//                     max_moves = max_moves.max(dp[i + 1][j + 1] + 1);
//                 }
//                 dp[i][j] = max_moves;
//             }
//         }
//         for i in 0..m {
//             result = result.max(dp[i][0]);
//         }
//         result
//     }
// }
