impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut count = 0;
        let mut r = 0;
        for c in (0..n).rev() {
            while r < m && grid[r][c] >= 0 {
                r += 1;
            }
            if r < m {
                count += m - r;
            }
        }
        count as i32
    }
}
