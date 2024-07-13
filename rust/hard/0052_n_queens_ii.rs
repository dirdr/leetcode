impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn dfs(availables: &mut Vec<Vec<bool>>, curr_row: usize) -> i32 {
            let n = availables.len();
            if curr_row == n {
                return 1;
            }
            let mut solutions = 0;
            for j in 0..n {
                if availables[curr_row][j] {
                    let prev = availables.clone();
                    Solution::update_availables(availables, (curr_row, j));
                    solutions += dfs(availables, curr_row + 1);
                    *availables = prev;
                }
            }
            solutions
        }
        dfs(&mut vec![vec![true; n as usize]; n as usize], 0)
    }

    fn update_availables(availables: &mut Vec<Vec<bool>>, pos: (usize, usize)) {
        let (row, col) = pos;
        let n = availables.len();
        for i in 0..n {
            availables[i][col] = false;
            availables[row][i] = false;
        }
        let directions: [(isize, isize); 4] = [(-1, -1), (1, 1), (-1, 1), (1, -1)];
        for (di, dj) in directions.iter() {
            let mut i = row as isize + di;
            let mut j = col as isize + dj;
            while i >= 0 && i < n as isize && j >= 0 && j < n as isize {
                availables[i as usize][j as usize] = false;
                i += di;
                j += dj;
            }
        }
    }
}
