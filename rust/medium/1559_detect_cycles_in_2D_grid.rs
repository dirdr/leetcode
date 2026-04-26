impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];

        fn dfs(
            grid: &[Vec<char>],
            visited: &mut Vec<Vec<bool>>,
            r: usize, c: usize,
            pr: usize, pc: usize,
        ) -> bool {
            visited[r][c] = true;
            const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for (dr, dc) in DIRS {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0 || nr as usize >= grid.len() || nc < 0 || nc as usize >= grid[0].len() {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] != grid[r][c] || (nr == pr && nc == pc) {
                    continue;
                }
                if visited[nr][nc] || dfs(grid, visited, nr, nc, r, c) {
                    return true;
                }
            }
            false
        }

        for i in 0..rows {
            for j in 0..cols {
                if !visited[i][j] && dfs(&grid, &mut visited, i, j, usize::MAX, usize::MAX) {
                    return true;
                }
            }
        }
        false
    }
}
