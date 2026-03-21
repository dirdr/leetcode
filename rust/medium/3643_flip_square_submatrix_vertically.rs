impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let (x, y) = (x as usize, y as usize);
        let (m, n) = (grid.len(), grid[0].len());
        let mut rows = vec![vec![0; k]; k];
        for i in 0..k {
            for j in 0..k {
                rows[i][j] = grid[x + i][y + j];
            }
        }
        rows.reverse();
        for i in 0..k {
            for j in 0..k {
                grid[x + i][y + j] = rows[i][j];
            }
        }
        grid
    }
}
