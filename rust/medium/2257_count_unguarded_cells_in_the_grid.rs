#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
    Wall,
    Guard,
    Visited,
    Unvisited,
}

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![CellType::Unvisited; n]; m];
        for w in &walls {
            grid[w[0] as usize][w[1] as usize] = CellType::Wall;
        }
        for g in &guards {
            grid[g[0] as usize][g[1] as usize] = CellType::Guard;
        }
        for r in 0..m {
            for c in 0..n {
                if grid[r as usize][c as usize] != CellType::Guard {
                    continue;
                }
                Self::guard_projection(r, c, m, n, &mut grid);
            }
        }
        grid.iter().flatten().filter(|&&e| e == CellType::Unvisited).count() as i32
    }

    fn guard_projection(r: usize, c: usize, m: usize, n: usize, grid: &mut Vec<Vec<CellType>>) {
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in DIRS {
            let mut nr = r as isize;
            let mut nc = c as isize;
            loop {
                nr += dr;
                nc += dc;
                if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                    break;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == CellType::Guard || grid[nr][nc] == CellType::Wall {
                    break;
                }
                grid[nr][nc] = CellType::Visited;
            }                       
        }
    }
}
