impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        fn backtrack(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, sum: i32) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            let directions: [(isize, isize); 4] = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ];
            let gold = grid[r][c];
            let mut max_gold = sum + gold;
            grid[r][c] = -1;
            for (dr, dc) in directions {
                let (nr, nc) = (r as isize + dr, c as isize + dc);
                if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == 0 || grid[nr][nc] == -1 {
                    continue;
                }
                max_gold = max_gold.max(backtrack(grid, nr, nc, sum + gold));
            }
            grid[r][c] = gold;
            max_gold
        }

        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    max = max.max(backtrack(&mut grid.clone(), i, j, 0));
                }
            }
        }
        max
    }
}
