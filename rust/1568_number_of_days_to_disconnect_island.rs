impl Solution {
    fn dfs(grid: &Vec<Vec<i32>>, r: isize, c: isize, visited: &mut Vec<Vec<bool>>) {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in directions.iter() {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr >= grid.len() as isize || nc < 0 || nc >= grid[0].len() as isize {
                continue;
            }
            if visited[nr as usize][nc as usize] || grid[nr as usize][nc as usize] == 0 {
                continue
            }   
            visited[nr as usize][nc as usize] = true;
            Self::dfs(grid, nr, nc, visited);
        }
    }

    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut islands = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 || visited[r][c] {
                    continue;
                }
                islands += 1;
                visited[r][c] = true;
                Self::dfs(&grid, r as isize, c as isize, &mut visited);
            }
        }
        if islands != 1 {
            return 0;
        }
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    grid[r][c] = 0;
                    visited = vec![vec![false; cols]; rows];
                    let mut new_islands = 0;
                    for rr in 0..rows {
                        for cc in 0..cols {
                            if grid[rr][cc] == 0 || visited[rr][cc] {
                                continue;
                            }
                            new_islands += 1;
                            visited[rr][cc] = true;
                            Self::dfs(&grid, rr as isize, cc as isize, &mut visited);
                        }
                    }
                    grid[r][c] = 1;
                    if new_islands != 1 {
                        return 1;
                    }
                }
            }
        }
        2
    }
}
